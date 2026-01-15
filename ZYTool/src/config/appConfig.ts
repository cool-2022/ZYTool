export const AppConfig = {
    // 前端调用后端的基础地址（优先使用环境变量 VITE_API_BASE_URL）
    apiBaseUrl: (import.meta.env.VITE_API_BASE_URL as string) || 'http://localhost:8000',

    // 可在这里添加其它前端配置，例如默认分页大小、主题开关等
    defaults: {
        pageSize: 20,
    },
}

export default AppConfig
