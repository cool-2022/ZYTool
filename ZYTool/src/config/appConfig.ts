export const AppConfig = {
    // Rust 后端（优先使用）
    rustApiBaseUrl: (import.meta.env.VITE_RUST_API_BASE_URL as string) || 'http://localhost:8000',

    // Python 后端（降级兜底）
    pythonApiBaseUrl: (import.meta.env.VITE_PYTHON_API_BASE_URL as string) || 'http://localhost:8001',

    // API 前缀
    apiPrefix: '/api/v1',

    // 可在这里添加其它前端配置，例如默认分页大小、主题开关等
    defaults: {
        pageSize: 20,
    },
}

export default AppConfig
