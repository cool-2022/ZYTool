/**
 * 统一 API 服务层（services/api.ts）
 * 项目所有后端请求的唯一出口，核心机制：
 * - 双后端：rustApi（axum 主后端 :10003）+ pythonApi（FastAPI 兜底 :10004），均挂 /api/v1
 * - requestWithFallback：仅网络错误/超时/5xx 时降级 Python，4xx 不降级
 * - chatStream：axios + fetch adapter 手动解析 SSE 的流式聊天，支持 AbortSignal 中止
 * - 401 统一拦截：clearAuth 后跳转 /?redirect=<当前路径>
 */
import axios, { type AxiosError, type AxiosInstance, type AxiosRequestConfig } from 'axios'
import { getToken, clearAuth, getBaseInfo, buildDefaultBaseInfo } from '../utils/auth'
import { AppConfig } from '../config/appConfig'

// ===================== 双后端 axios 实例 =====================

/** 创建一个 axios 实例（Rust / Python 各一个），统一注入 token、X-Base-Info 头与耗时日志拦截器 */
function createApiInstance(baseURL: string): AxiosInstance {
    const instance = axios.create({
        baseURL,
        timeout: 60000,
        headers: {
            'Content-Type': 'application/json',
        },
    })

    // 请求拦截器 - 添加 token 和基础信息头，并记录请求起始时间
    instance.interceptors.request.use(
        (config) => {
            if (import.meta.env.DEV) console.log('发送请求:', config.method?.toUpperCase(), config.url);

            (config as any).__startTime = performance.now()

            const token = getToken()
            if (token) {
                config.headers.Authorization = `Bearer ${token}`
            }

            const baseInfo = getBaseInfo() || buildDefaultBaseInfo()
            config.headers['X-Base-Info'] = JSON.stringify(baseInfo)

            return config
        },
        (error) => {
            if (import.meta.env.DEV) console.error('请求错误:', error)
            return Promise.reject(error)
        }
    )

    // 响应拦截器 - 记录请求耗时（毫秒），处理 token 过期
    instance.interceptors.response.use(
        (response) => {

            const startTime = (response.config as any).__startTime as number | undefined
            const duration = startTime !== undefined ? `${(performance.now() - startTime).toFixed(1)} ms` : 'unknown'
            console.log(
                `[API] ${response.config.method?.toUpperCase()} ${response.config.baseURL}${response.config.url} 耗时 ${duration}`
            )
            if (import.meta.env.DEV) console.log('收到响应:', response.status, response.data)
            return response
        },
        (error) => {
            const startTime = (error.config as any)?.__startTime as number | undefined
            const duration = startTime !== undefined ? `${(performance.now() - startTime).toFixed(1)} ms` : 'unknown'
            console.error(
                `[API] ${error.config?.method?.toUpperCase()} ${error.config?.baseURL}${error.config?.url} 耗时 ${duration}（失败）`
            )
            if (import.meta.env.DEV) console.error('响应错误:', error.response?.status, error.response?.data)

            // 处理 401 未授权错误（token 过期或无效）：清除登录态并跳转登录页
            if (error.response?.status === 401) {
                clearAuth()
                redirectToLogin()
            }

            return Promise.reject(error)
        }
    )

    return instance
}

// 跳转登录页（携带当前路径，登录后原路返回；避免在登录页重复跳转）
function redirectToLogin() {
    const currentPath = window.location.pathname + window.location.search
    if (window.location.pathname === '/') return
    window.location.href = `/?redirect=${encodeURIComponent(currentPath)}`
}

export const rustApi = createApiInstance(`${AppConfig.rustApiBaseUrl}${AppConfig.apiPrefix}`)
export const pythonApi = createApiInstance(`${AppConfig.pythonApiBaseUrl}${AppConfig.apiPrefix}`)

// ===================== 统一响应结构 =====================

/** 统一响应结构解包：仅当返回体同时含 success 和 data 字段（BaseResponse）时才取 data，否则原样返回 */
function unwrapResponse<T>(data: any): T {
    if (data && typeof data === 'object' && 'success' in data && 'data' in data) {
        return data.data as T
    }
    return data as T
}

// ===================== 降级兜底逻辑 =====================

/**
 * 判断错误是否应该触发 Python 后端降级
 *
 * 触发降级的场景：
 * - 无响应（Rust 未启动或网络不可达）
 * - 请求超时
 * - 5xx 服务器错误（503/502/500/504 等，表示 Rust 资源不足或服务异常）
 *
 * 不触发的场景：
 * - 4xx 客户端错误（401/403/422 等，Python 也会返回同样错误）
 */
function shouldFallbackToPython(error: AxiosError): boolean {
    if (!error.response) {
        // 网络错误、Rust 未启动、请求被取消、超时等
        return true
    }

    const status = error.response.status
    // 5xx 表示服务端问题，降级到 Python
    if (status >= 500 && status < 600) {
        return true
    }

    return false
}

/**
 * 优先请求 Rust 后端；失败且满足降级条件时，自动请求 Python 后端
 */
export async function requestWithFallback<T>(
    method: 'get' | 'post' | 'put' | 'delete' | 'patch',
    url: string,
    data?: unknown,
    config?: AxiosRequestConfig
): Promise<T> {
    try {
        const response = await rustApi.request({ method, url, data, ...config })
        return unwrapResponse<T>(response.data)
    } catch (err) {
        const axiosError = err as AxiosError
        // 仅网络错误/超时/5xx 才降级 Python；4xx 属于业务错误，原样抛出
        if (!shouldFallbackToPython(axiosError)) {
            throw err
        }

        if (import.meta.env.DEV) {
            console.warn(`[Fallback] Rust 后端不可用，降级到 Python 后端: ${method.toUpperCase()} ${url}`)
        }

        const response = await pythonApi.request({ method, url, data, ...config })
        return unwrapResponse<T>(response.data)
    }
}

// ===================== 类型定义 =====================

export interface Category {
    id: number
    name: string
    description: string
    tools: Tool[]
}

export interface Tool {
    id: number
    name: string
    icon: string
    description: string
    type?: 'frontend' | 'backend'
    /** 前端路由（后端数据驱动）；空串表示未接入页面 */
    route?: string
}

export interface TextCompareRequest {
    text1: string
    text2: string
}

export interface DiffLine {
    lineNumber: number
    content: string
}

export interface FileDiffResult {
    onlyInA: DiffLine[]
    onlyInB: DiffLine[]
}

export interface FolderDiffResult {
    totalFiles: number
    differentFiles: string[]
}

export type DiffResult = FileDiffResult | FolderDiffResult

export interface AdminTableInfo {
    name: string
    comment?: string | null
    row_estimate: number
}

export interface AdminColumnInfo {
    name: string
    data_type: string
    nullable: boolean
    is_primary: boolean
    column_default?: string | null
    comment?: string | null
}

export interface AdminRowsData {
    table: string
    columns: string[]
    rows: Record<string, any>[]
    total: number
    page: number
    page_size: number
}

export interface FieldRootRef {
    table_name: string
    data_type: string
    data_length: number | null
    data_precision: number | null
    data_scale: number | null
    type_expr: string
    comment: string
    type_synced: boolean
    comment_synced: boolean
    synced: boolean
    diffs: string[]
}

export interface FieldRootInfo {
    id: number | null
    registered: boolean
    field_name: string
    data_type: string
    data_length: number | null
    data_precision: number | null
    data_scale: number | null
    type_expr: string
    comment: string
    remark: string
    updated_at: string | null
    refs: FieldRootRef[]
}

export interface FieldRootPayload {
    field_name: string
    data_type?: string
    data_length?: number | null
    data_precision?: number | null
    data_scale?: number | null
    comment?: string
    remark?: string
}

export interface FieldRootsData {
    rows: FieldRootInfo[]
    total: number
    page: number
    page_size: number
}

export interface FieldRootSyncResult {
    table_name: string
    ok: boolean
    message: string
}

export interface AdminServiceInfo {
    name: string
    version: string
    description: string
    status: string
    db_connected: boolean
    uptime_seconds: number
    timestamp: string
}

export interface ServiceHealth {
    ok: boolean
    status?: string
    version?: string
    timestamp?: string
    error?: string
}

// ===================== API 服务类 =====================

export interface TokenData {
    access_token: string
    token_type: string
    expires_in: number
}

export interface UserInfo {
    username: string
    user_id?: number
    roles: string[]
}

export class ApiService {
    // ========== 认证相关 ==========

    static async login(username: string, password: string): Promise<TokenData> {
        return requestWithFallback('post', '/auth/login', { username, password })
    }

    static async register(username: string, password: string, email?: string): Promise<TokenData> {
        return requestWithFallback('post', '/auth/register', { username, password, email })
    }

    static async getCurrentUser(): Promise<UserInfo> {
        return requestWithFallback('get', '/auth/me')
    }

    static async getBindings(): Promise<{
        phone?: string
        phone_verified: boolean
        email?: string
        providers: Array<{
            provider: string
            open_id: string
            union_id?: string
            nickname: string
        }>
    }> {
        return requestWithFallback('get', '/auth/bindings')
    }

    static async bindPhone(phone: string): Promise<void> {
        await requestWithFallback('post', '/auth/bind/phone', { phone })
    }

    static async bindThirdParty(
        provider: 'qq' | 'wechat',
        openId: string,
        nickname?: string,
        unionId?: string
    ): Promise<void> {
        await requestWithFallback('post', `/auth/bind/${provider}`, {
            open_id: openId,
            nickname,
            union_id: unionId,
        })
    }

    static async getQQAuthUrl(): Promise<{ url: string }> {
        return requestWithFallback('get', '/auth/qq/auth-url')
    }

    static async loginWithQQ(code: string): Promise<TokenData> {
        return requestWithFallback('post', '/auth/qq/login', { code })
    }

    // ========== 工具相关 ==========

    static async getCategories(): Promise<{ categories: Category[] }> {
        return requestWithFallback('get', '/tools/categories')
    }

    static async compareFiles(request: TextCompareRequest): Promise<FileDiffResult> {
        return requestWithFallback('post', '/tools/diff/compare', request)
    }

    static async healthCheck(): Promise<{ status: string; message: string }> {
        return requestWithFallback('get', '/health')
    }

    static async getRoute(request: any): Promise<any> {
        return requestWithFallback('post', '/tools/map/route', request)
    }

    // ========== 后台管理（数据库表） ==========

    static async getAdminTables(): Promise<{ tables: AdminTableInfo[] }> {
        return requestWithFallback('get', '/admin/tables')
    }

    static async getAdminTableColumns(table: string): Promise<{ table: string; columns: AdminColumnInfo[] }> {
        return requestWithFallback('get', `/admin/tables/${encodeURIComponent(table)}/columns`)
    }

    static async getAdminTableRows(
        table: string,
        page: number,
        pageSize: number,
        keyword?: string
    ): Promise<AdminRowsData> {
        return requestWithFallback('get', `/admin/tables/${encodeURIComponent(table)}/rows`, undefined, {
            params: { page, page_size: pageSize, keyword: keyword || undefined },
        })
    }

    // ========== 后台管理（字段字根） ==========

    static async getAdminRoots(page: number, pageSize: number, keyword?: string): Promise<FieldRootsData> {
        return requestWithFallback('get', '/admin/roots', undefined, {
            params: { page, page_size: pageSize, keyword: keyword || undefined },
        })
    }

    // 扫描现有表字段入库（已存在的字段名不覆盖）
    static async scanAdminRoots(): Promise<{ inserted: number; total_roots: number }> {
        return requestWithFallback('post', '/admin/roots/scan')
    }

    static async createAdminRoot(payload: FieldRootPayload): Promise<{ root_item: FieldRootInfo }> {
        return requestWithFallback('post', '/admin/roots', payload)
    }

    static async updateAdminRoot(id: number, payload: FieldRootPayload): Promise<{ root_item: FieldRootInfo }> {
        return requestWithFallback('put', `/admin/roots/${id}`, payload)
    }

    static async deleteAdminRoot(id: number): Promise<void> {
        await requestWithFallback('delete', `/admin/roots/${id}`)
    }

    // 手动把字根的注释/类型同步到所有引用表字段
    static async syncAdminRoot(
        id: number,
        syncComment: boolean,
        syncType: boolean
    ): Promise<{ results: FieldRootSyncResult[] }> {
        return requestWithFallback('post', `/admin/roots/${id}/sync`, {
            sync_comment: syncComment,
            sync_type: syncType,
        })
    }

    // ========== 后台管理（服务状态） ==========

    static async getAdminServices(): Promise<{ service: AdminServiceInfo }> {
        return requestWithFallback('get', '/admin/services')
    }

    // 直接探测 Python 兜底后端健康状态（不走降级，用于服务监控展示）
    static async checkPythonHealth(): Promise<ServiceHealth> {
        try {
            const response = await pythonApi.get('/health')
            const data = unwrapResponse<{ status?: string; version?: string; timestamp?: string }>(response.data)
            return { ok: true, status: data.status, version: data.version, timestamp: data.timestamp }
        } catch (e: any) {
            return { ok: false, error: e?.message || '无法连接' }
        }
    }

    // ========== AI 助手会话/消息 ==========

    static async getChatSessions(): Promise<{
        sessions: Array<{
            id: string
            title: string
            date: string
            message_count: number
            total_tokens: number
            model_id?: number
            updated_at: string
        }>
    }> {
        return requestWithFallback('get', '/agents/sessions')
    }

    static async createChatSession(title?: string, modelId?: number): Promise<{
        id: string
        title: string
        date: string
        message_count: number
        total_tokens: number
        model_id?: number
        updated_at: string
    }> {
        return requestWithFallback('post', '/agents/sessions', { title, model_id: modelId })
    }

    static async deleteChatSession(sessionId: string): Promise<void> {
        await requestWithFallback('delete', `/agents/sessions/${sessionId}`)
    }

    static async updateChatSessionTitle(sessionId: string, title: string): Promise<void> {
        await requestWithFallback('patch', `/agents/sessions/${sessionId}`, { title })
    }

    static async getChatMessages(sessionId: string): Promise<{
        messages: Array<{
            id: string
            role: string
            content: string
            content_type: string
            tokens_used: number
            model_id?: number
            created_at: string
        }>
    }> {
        return requestWithFallback('get', `/agents/sessions/${sessionId}/messages`)
    }

    // 可选模型列表（ai_models 表）
    static async getChatModels(): Promise<{
        models: Array<{
            id: number
            provider: string
            model_name: string
            display_name: string
            is_enabled: boolean
            priority: number
        }>
    }> {
        return requestWithFallback('get', '/agents/models')
    }

    // AI 聊天 - 流式响应（使用 axios + fetch adapter，支持双后端兜底，可传入 signal 中止）
    static async *chatStream(message: string, sessionId?: string, signal?: AbortSignal, modelId?: number) {
        const body = { message, session_id: sessionId, model_id: modelId }
        const token = getToken()
        const headers: Record<string, string> = {}
        if (token) {
            headers.Authorization = `Bearer ${token}`
        }

        async function requestChatStream(instance: AxiosInstance): Promise<ReadableStream<Uint8Array>> {
            const response = await instance.request({
                method: 'POST',
                url: '/agents/chat',
                data: body,
                headers,
                adapter: 'fetch',
                responseType: 'stream',
                signal,
            })
            // fetch adapter + responseType: 'stream' 在浏览器下返回 ReadableStream
            const stream = response.data as ReadableStream<Uint8Array> | undefined
            if (!stream) {
                throw new Error('Response body is not a stream')
            }
            return stream
        }

        let stream: ReadableStream<Uint8Array>
        let usedBackend = 'rust'

        try {
            stream = await requestChatStream(rustApi)
        } catch (err) {
            // 流建立阶段失败直接降级（此时尚无响应体，无法区分 4xx/5xx）
            if (signal?.aborted) return
            if (import.meta.env.DEV) {
                console.warn('[Fallback] Rust 流式后端不可用，降级到 Python 后端:', err)
            }
            usedBackend = 'python'
            stream = await requestChatStream(pythonApi)
        }

        const reader = stream.getReader()
        const decoder = new TextDecoder()
        let buffer = ''

        const onAbort = () => {
            reader.cancel().catch(() => { })
        }
        signal?.addEventListener('abort', onAbort)

        try {
            while (true) {
                const { done, value } = await reader.read()
                if (done) break

                buffer += decoder.decode(value, { stream: true })

                // 手动解析 SSE：按行切分，pop 出的末行可能不完整，留回 buffer 等下一片段拼接
                const lines = buffer.split('\n')
                buffer = lines.pop() || ''

                for (const line of lines) {
                    if (line.startsWith('data: ')) {
                        const data = line.slice(6).trim()
                        // 服务端以 data: [DONE] 作为流结束标记，跳过不 yield
                        if (data && data !== '[DONE]') {
                            yield data
                        }
                    }
                }
            }
        } catch (err) {
            if (signal?.aborted) return
            throw new Error(`流式读取失败 (backend: ${usedBackend}): ${err}`)
        } finally {
            signal?.removeEventListener('abort', onAbort)
            reader.releaseLock()
        }
    }
}
