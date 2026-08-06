export const AppConfig = {
    // Rust 后端（优先使用）
    rustApiBaseUrl: (import.meta.env.VITE_RUST_API_BASE_URL as string) || 'http://localhost:8000',

    // Python 后端（降级兜底）
    pythonApiBaseUrl: (import.meta.env.VITE_PYTHON_API_BASE_URL as string) || 'http://localhost:8001',

    // API 前缀
    apiPrefix: '/api/v1',

    // 高德地图 Web 端 JS API Key（可用 VITE_AMAP_KEY 覆盖）
    amapKey: (import.meta.env.VITE_AMAP_KEY as string) || '82aaaef6e38ad9523d993e795b2fd05c',
}
