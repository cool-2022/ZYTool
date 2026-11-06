import { ref } from 'vue'

export type AdminTab = 'database' | 'roots' | 'services'

export function useAdminView() {
    const activeTab = ref<AdminTab>('database')
    // 首次切换到某分类时才挂载对应面板，避免一次性并发所有请求
    const visited = ref<Set<AdminTab>>(new Set(['database']))

    function selectTab(tab: AdminTab) {
        activeTab.value = tab
        if (!visited.value.has(tab)) {
            visited.value = new Set(visited.value).add(tab)
        }
    }

    function isVisited(tab: AdminTab) {
        return visited.value.has(tab)
    }

    return { activeTab, selectTab, isVisited }
}
