import { onMounted, ref } from 'vue'
import { ApiService } from '@/services/api'
import { AppConfig } from '@/config/appConfig'
import type { AdminServiceInfo, ServiceHealth } from '@/services/api'

export function useServicesAdminView() {
    const loading = ref(false)
    const forbidden = ref(false)

    const rust = ref<AdminServiceInfo | null>(null)
    const rustError = ref('')
    const python = ref<ServiceHealth | null>(null)

    const rustUrl = `${AppConfig.rustApiBaseUrl}${AppConfig.apiPrefix}`
    const pythonUrl = `${AppConfig.pythonApiBaseUrl}${AppConfig.apiPrefix}`

    function formatUptime(seconds: number) {
        if (seconds < 60) return `${seconds} 秒`
        const d = Math.floor(seconds / 86400)
        const h = Math.floor((seconds % 86400) / 3600)
        const m = Math.floor((seconds % 3600) / 60)
        const parts: string[] = []
        if (d) parts.push(`${d} 天`)
        if (h) parts.push(`${h} 小时`)
        if (m || !parts.length) parts.push(`${m} 分钟`)
        return parts.join(' ')
    }

    function formatTime(value?: string) {
        if (!value) return '—'
        const d = new Date(value)
        return Number.isNaN(d.getTime()) ? value : d.toLocaleString('zh-CN', { hour12: false })
    }

    async function load() {
        loading.value = true
        rustError.value = ''
        try {
            const res = await ApiService.getAdminServices()
            rust.value = res.service
        } catch (e: any) {
            if (e?.response?.status === 403) {
                forbidden.value = true
            } else {
                rust.value = null
                rustError.value = e?.response?.data?.message || e?.message || 'Rust 后端不可用'
            }
        }
        python.value = await ApiService.checkPythonHealth()
        loading.value = false
    }

    onMounted(load)

    return {
        loading, forbidden, rust, rustError, python, rustUrl, pythonUrl,
        formatUptime, formatTime, refresh: load,
    }
}
