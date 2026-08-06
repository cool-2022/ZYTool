import axios, { type AxiosError, type AxiosInstance, type AxiosRequestConfig } from 'axios'
import { getToken, clearAuth, getBaseInfo, buildDefaultBaseInfo } from '../utils/auth'
import { AppConfig } from '../config/appConfig'

// ===================== 双后端 axios 实例 =====================

function createApiInstance(baseURL: string): AxiosInstance {
    const instance = axios.create({
        baseURL,
        timeout: 60000,
        headers: {
            'Content-Type': 'application/json',
        },
    })

    // 请求拦截器 - 添加 token 和基础信息头
    instance.interceptors.request.use(
        (config) => {
            if (import.meta.env.DEV) console.log('发送请求:', config.method?.toUpperCase(), config.url)

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

    // 响应拦截器 - 处理 token 过期
    instance.interceptors.response.use(
        (response) => {
            if (import.meta.env.DEV) console.log('收到响应:', response.status, response.data)
            return response
        },
        (error) => {
            if (import.meta.env.DEV) console.error('响应错误:', error.response?.status, error.response?.data)

            // 处理 401 未授权错误（token 过期或无效）
            if (error.response?.status === 401) {
                clearAuth()
                // 可以在这里跳转到登录页
                // window.location.href = '/login'
            }

            return Promise.reject(error)
        }
    )

    return instance
}

export const rustApi = createApiInstance(`${AppConfig.rustApiBaseUrl}${AppConfig.apiPrefix}`)
export const pythonApi = createApiInstance(`${AppConfig.pythonApiBaseUrl}${AppConfig.apiPrefix}`)

// ===================== 统一响应结构 =====================

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

    // AI 聊天 - 流式响应（使用 axios + fetch adapter，支持双后端兜底）
    static async *chatStream(message: string, sessionId?: string) {
        const body = { message, session_id: sessionId }
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
            if (import.meta.env.DEV) {
                console.warn('[Fallback] Rust 流式后端不可用，降级到 Python 后端:', err)
            }
            usedBackend = 'python'
            stream = await requestChatStream(pythonApi)
        }

        const reader = stream.getReader()
        const decoder = new TextDecoder()
        let buffer = ''

        try {
            while (true) {
                const { done, value } = await reader.read()
                if (done) break

                buffer += decoder.decode(value, { stream: true })

                // 处理 SSE 格式的数据
                const lines = buffer.split('\n')
                buffer = lines.pop() || ''

                for (const line of lines) {
                    if (line.startsWith('data: ')) {
                        const data = line.slice(6).trim()
                        if (data && data !== '[DONE]') {
                            yield data
                        }
                    }
                }
            }
        } catch (err) {
            throw new Error(`流式读取失败 (backend: ${usedBackend}): ${err}`)
        } finally {
            reader.releaseLock()
        }
    }
}
