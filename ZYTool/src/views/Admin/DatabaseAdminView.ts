import { computed, onMounted, ref } from 'vue'
import { message } from 'ant-design-vue'
import { ApiService } from '@/services/api'
import type { AdminColumnInfo, AdminRowsData, AdminTableInfo } from '@/services/api'

const PAGE_SIZE_OPTIONS = ['10', '20', '50', '100']

export function useDatabaseAdminView() {
    const tables = ref<AdminTableInfo[]>([])
    const tablesLoading = ref(false)
    const tableKeyword = ref('')

    const activeTable = ref('')
    const activeTab = ref<'rows' | 'columns'>('rows')

    const columns = ref<AdminColumnInfo[]>([])
    const columnsLoading = ref(false)

    const rowsData = ref<AdminRowsData>({ table: '', columns: [], rows: [], total: 0, page: 1, page_size: 20 })
    const rowsLoading = ref(false)
    const keyword = ref('')
    const pageSize = ref(20)

    // 无管理员权限时整页提示，不再重复请求
    const forbidden = ref(false)

    const filteredTables = computed(() => {
        const kw = tableKeyword.value.trim().toLowerCase()
        if (!kw) return tables.value
        return tables.value.filter(
            t => t.name.toLowerCase().includes(kw) || (t.comment || '').toLowerCase().includes(kw)
        )
    })

    const activeTableInfo = computed(() => tables.value.find(t => t.name === activeTable.value))

    const rowTableColumns = computed(() =>
        rowsData.value.columns.map(name => ({ title: name, dataIndex: name, key: name, ellipsis: true }))
    )

    const tableRows = computed(() =>
        rowsData.value.rows.map((row, index) => ({ ...row, __row_key: index }))
    )

    const structureColumns = [
        { title: '字段名', dataIndex: 'name', key: 'name', filterable: true },
        { title: '类型', dataIndex: 'data_type', key: 'data_type', width: 170, filterable: true },
        { title: '可空', dataIndex: 'nullable', key: 'nullable', width: 70 },
        { title: '默认值', dataIndex: 'column_default', key: 'column_default', filterable: true },
        { title: '注释', dataIndex: 'comment', key: 'comment', filterable: true },
    ]

    function extractError(e: any): string {
        return e?.response?.data?.message || e?.message || '请求失败'
    }

    function handleError(e: any, fallback: string) {
        if (e?.response?.status === 403) {
            forbidden.value = true
            return
        }
        message.error(extractError(e) || fallback)
    }

    async function loadTables() {
        tablesLoading.value = true
        try {
            const res = await ApiService.getAdminTables()
            tables.value = res.tables || []
            const first = tables.value[0]
            if (!activeTable.value && first) {
                await selectTable(first.name)
            }
        } catch (e) {
            handleError(e, '获取表列表失败')
        } finally {
            tablesLoading.value = false
        }
    }

    async function selectTable(name: string) {
        if (rowsLoading.value || columnsLoading.value) return
        activeTable.value = name
        keyword.value = ''
        await Promise.all([loadColumns(), loadRows(1)])
    }

    async function loadColumns() {
        if (!activeTable.value) return
        columnsLoading.value = true
        try {
            const res = await ApiService.getAdminTableColumns(activeTable.value)
            columns.value = res.columns || []
        } catch (e) {
            handleError(e, '获取表结构失败')
        } finally {
            columnsLoading.value = false
        }
    }

    async function loadRows(page: number) {
        if (!activeTable.value) return
        rowsLoading.value = true
        try {
            rowsData.value = await ApiService.getAdminTableRows(
                activeTable.value,
                page,
                pageSize.value,
                keyword.value.trim() || undefined
            )
        } catch (e) {
            handleError(e, '获取表数据失败')
        } finally {
            rowsLoading.value = false
        }
    }

    function searchRows() {
        loadRows(1)
    }

    function changePage(page: number, size: number) {
        // 每页条数变化时回到第一页
        if (size !== pageSize.value) {
            pageSize.value = size
            loadRows(1)
        } else {
            loadRows(page)
        }
    }

    async function refresh() {
        if (activeTable.value) {
            await Promise.all([loadColumns(), loadRows(rowsData.value.page)])
        }
        await loadTables()
    }

    onMounted(loadTables)

    return {
        tables, tablesLoading, tableKeyword, filteredTables,
        activeTable, activeTableInfo, activeTab,
        columns, columnsLoading, structureColumns,
        rowsData, rowsLoading, keyword, rowTableColumns, tableRows,
        pageSize, pageSizeOptions: PAGE_SIZE_OPTIONS,
        forbidden,
        selectTable, searchRows, changePage, refresh,
    }
}
