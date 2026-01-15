import axios from 'axios'

// 创建axios实例
const api = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000/api/v1/tools',
    timeout: 10000,
    headers: {
        'Content-Type': 'application/json',
    },
})

// 请求拦截器
api.interceptors.request.use(
    (config) => {
        if (import.meta.env.DEV) console.log('发送请求:', config.method?.toUpperCase(), config.url)
        return config
    },
    (error) => {
        if (import.meta.env.DEV) console.error('请求错误:', error)
        return Promise.reject(error)
    }
)

// 响应拦截器
api.interceptors.response.use(
    (response) => {
        if (import.meta.env.DEV) console.log('收到响应:', response.status, response.data)
        return response
    },
    (error) => {
        if (import.meta.env.DEV) console.error('响应错误:', error.response?.status, error.response?.data)
        return Promise.reject(error)
    }
)

// API接口定义
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
    type?: 'frontend' | 'backend'  // 工具类型：前端处理或后端处理
}

export interface TextProcessRequest {
    text: string
    action: 'json_format' | 'base64_encode' | 'base64_decode' | 'url_encode' | 'url_decode'
}

export interface TextCompareRequest {
    text1: string
    text2: string
}

export interface RegexTestRequest {
    pattern: string
    text: string
}

export interface PasswordGenerateRequest {
    length?: number
    include_symbols?: boolean
    include_numbers?: boolean
    include_uppercase?: boolean
    include_lowercase?: boolean
}

export interface TimestampConvertRequest {
    timestamp: number
    action: 'to_datetime' | 'to_timestamp'
}

// 文件比对相关接口
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

// API服务类
export class ApiService {
    // 获取工具分类
    static async getCategories(): Promise<{ categories: Category[] }> {
        // const response = await api.get('/categories')
        const response = await api.get('/categories')
        // const response = await fetch('http://localhost:8000/api/categories')
        // if (!response.ok) {
        //     throw new Error(`HTTP error! status: ${response.status}`)
        // }
        return await response.data
    }

    // 文本处理
    static async processText(request: TextProcessRequest): Promise<{ result: string; success: boolean }> {
        const response = await api.post('/text/process', request)
        return response.data
    }

    // 文本对比
    static async compareText(request: TextCompareRequest): Promise<{
        differences: Array<{
            line: number
            text1: string
            text2: string
            type: 'modified' | 'added' | 'removed'
        }>
        summary: {
            total_lines: number
            different_lines: number
            identical: boolean
        }
    }> {
        const response = await api.post('/text/compare', request)
        return response.data
    }

    // 正则表达式测试
    static async testRegex(request: RegexTestRequest): Promise<{
        matches: string[]
        match_details: Array<{
            match: string
            start: number
            end: number
            groups: string[]
        }>
        success: boolean
    }> {
        const response = await api.post('/regex/test', request)
        return response.data
    }

    // 密码生成
    static async generatePassword(request: PasswordGenerateRequest): Promise<{
        password: string
        length: number
        character_types: {
            lowercase: boolean
            uppercase: boolean
            numbers: boolean
            symbols: boolean
        }
    }> {
        const response = await api.post('/password/generate', request)
        return response.data
    }

    // 时间戳转换
    static async convertTimestamp(request: TimestampConvertRequest): Promise<{
        datetime: string
        timestamp: number
        action: string
    }> {
        const response = await api.post('/timestamp/convert', request)
        return response.data
    }

    // 文件比对
    static async compareFiles(request: TextCompareRequest): Promise<FileDiffResult> {
        const response = await api.post('/diff/compare', request)
        return response.data
    }

    // 健康检查
    static async healthCheck(): Promise<{ status: string; message: string }> {
        const response = await api.get('/health')
        return response.data
    }
}

export default api
