// API 地址默认跟随当前访问的 hostname：
// localhost 打开 → localhost:8000/8001；局域网 IP 打开 → 同 IP:8000/8001（IP 变化不受影响）
// 如需固定地址，用 VITE_RUST_API_BASE_URL / VITE_PYTHON_API_BASE_URL 覆盖
const currentHost = window.location.hostname || 'localhost'

export const AppConfig = {
    // Rust 后端（优先使用）
    rustApiBaseUrl: (import.meta.env.VITE_RUST_API_BASE_URL as string) || `http://${currentHost}:8000`,

    // Python 后端（降级兜底）
    pythonApiBaseUrl: (import.meta.env.VITE_PYTHON_API_BASE_URL as string) || `http://${currentHost}:8001`,

    // API 前缀
    apiPrefix: '/api/v1',

    // 高德地图 Web 端 JS API Key（可用 VITE_AMAP_KEY 覆盖）
    amapKey: (import.meta.env.VITE_AMAP_KEY as string) || '82aaaef6e38ad9523d993e795b2fd05c',
}
