import { computed, onMounted, reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { ApiService } from '@/services/api'
import type { FieldRootInfo, FieldRootSyncResult } from '@/services/api'

const PAGE_SIZE_OPTIONS = ['10', '20', '50', '100']
const BASE_TYPES = [
    'varchar', 'bpchar', 'numeric', 'text', 'int2', 'int4', 'int8', 'float4', 'float8',
    'bool', 'date', 'time', 'timetz', 'timestamp', 'timestamptz', 'interval',
    'uuid', 'json', 'jsonb', 'inet', 'bytea',
]
const TYPE_OPTIONS = [
    { label: '未指定', value: '' },
    ...BASE_TYPES.flatMap(value => [
        { label: value, value },
        { label: `${value}[]`, value: `${value}[]` },
    ]),
]

export function useRootsAdminView() {
    const rows = ref<FieldRootInfo[]>([])
    const loading = ref(false)
    const scanning = ref(false)
    const keyword = ref('')
    const page = ref(1)
    const pageSize = ref(20)
    const total = ref(0)

    // 无管理员权限时整页提示，不再重复请求
    const forbidden = ref(false)

    const modalOpen = ref(false)
    const saving = ref(false)
    const editingId = ref<number | null>(null)
    const form = reactive({
        field_name: '',
        data_type: '',
        data_length: null as number | null,
        data_precision: null as number | null,
        data_scale: null as number | null,
        comment: '',
        remark: '',
    })
    const baseType = computed(() => form.data_type.replace(/\[\]$/, ''))
    const isCharType = computed(() => ['varchar', 'bpchar'].includes(baseType.value))
    const isNumericType = computed(() => baseType.value === 'numeric')
    const isTimeType = computed(() => ['time', 'timetz', 'timestamp', 'timestamptz', 'interval'].includes(baseType.value))
    const formTypeExpr = computed(() => {
        let modifier = ''
        if (isCharType.value && form.data_length !== null) {
            modifier = `(${form.data_length})`
        } else if (isNumericType.value && form.data_precision !== null) {
            modifier = `(${form.data_precision},${form.data_scale ?? 0})`
        } else if (isTimeType.value && form.data_precision !== null) {
            modifier = `(${form.data_precision})`
        }
        return `${baseType.value}${modifier}${form.data_type.endsWith('[]') ? '[]' : ''}`
    })

    function changeType() {
        form.data_length = null
        form.data_precision = null
        form.data_scale = null
    }

    function changePrecision() {
        if (form.data_precision === null) form.data_scale = null
    }

    const syncOpen = ref(false)
    const syncing = ref(false)
    const syncTarget = ref<FieldRootInfo | null>(null)
    const syncComment = ref(true)
    const syncType = ref(false)
    const syncResults = ref<FieldRootSyncResult[] | null>(null)

    const columns = [
        { title: '字段名', dataIndex: 'field_name', key: 'field_name', width: 160, filterable: true },
        { title: '标准类型', dataIndex: 'type_expr', key: 'type_expr', width: 170 },
        { title: '注释', dataIndex: 'comment', key: 'comment', filterable: true },
        { title: '引用表', dataIndex: 'refs', key: 'refs', width: 220 },
        { title: '状态', key: 'status', width: 90 },
        { title: '更新时间', dataIndex: 'updated_at', key: 'updated_at', width: 170 },
        { title: '操作', key: 'action', width: 190 },
    ]

    function rootStatus(record: FieldRootInfo): { text: string; cls: string } {
        if (!record.registered) return { text: '未入库', cls: 'none' }
        if (!record.refs.length) return { text: '未引用', cls: 'none' }
        return record.refs.every(r => r.synced)
            ? { text: '一致', cls: 'ok' }
            : { text: '有差异', cls: 'diff' }
    }

    const syncTargets = computed(() => {
        const t = syncTarget.value
        if (!t) return []
        return t.refs.filter(r => {
            const commentDiff = syncComment.value && !r.comment_synced
            const typeDiff = syncType.value && !r.type_synced
            return commentDiff || typeDiff
        })
    })

    function formatTime(value: string | null) {
        if (!value) return '—'
        const d = new Date(value)
        return Number.isNaN(d.getTime()) ? value : d.toLocaleString('zh-CN', { hour12: false })
    }

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

    async function load() {
        loading.value = true
        try {
            const res = await ApiService.getAdminRoots(page.value, pageSize.value, keyword.value.trim() || undefined)
            rows.value = res.rows || []
            total.value = res.total || 0
        } catch (e) {
            handleError(e, '获取字根列表失败')
        } finally {
            loading.value = false
        }
    }

    function search() {
        page.value = 1
        load()
    }

    function changePage(p: number, size: number) {
        // 每页条数变化时回到第一页
        if (size !== pageSize.value) {
            pageSize.value = size
            page.value = 1
        } else {
            page.value = p
        }
        load()
    }

    async function scan() {
        scanning.value = true
        try {
            const res = await ApiService.scanAdminRoots()
            message.success(`扫描完成：新入库 ${res.inserted} 个字根，共 ${res.total_roots} 个`)
            page.value = 1
            await load()
        } catch (e) {
            handleError(e, '扫描字段入库失败')
        } finally {
            scanning.value = false
        }
    }

    function openCreate() {
        editingId.value = null
        Object.assign(form, {
            field_name: '', data_type: '', data_length: null, data_precision: null, data_scale: null,
            comment: '', remark: '',
        })
        modalOpen.value = true
    }

    // 未入库字段：以实时引用信息预填表单，提交即入库
    function openRegister(record: FieldRootInfo) {
        editingId.value = null
        Object.assign(form, {
            field_name: record.field_name,
            data_type: record.data_type,
            data_length: record.data_length,
            data_precision: record.data_precision,
            data_scale: record.data_scale,
            comment: record.comment,
            remark: record.remark,
        })
        modalOpen.value = true
    }

    function openEdit(record: FieldRootInfo) {
        if (record.id === null) return
        editingId.value = record.id
        Object.assign(form, {
            field_name: record.field_name,
            data_type: record.data_type,
            data_length: record.data_length,
            data_precision: record.data_precision,
            data_scale: record.data_scale,
            comment: record.comment,
            remark: record.remark,
        })
        modalOpen.value = true
    }

    async function submit() {
        if (!form.field_name.trim()) {
            message.warning('字段名不能为空')
            return
        }
        if ([form.data_length, form.data_precision, form.data_scale].some(v => v !== null && !Number.isInteger(v))) {
            message.warning('长度、精度和小数位必须为整数')
            return
        }
        if (isCharType.value && form.data_length !== null && (form.data_length < 1 || form.data_length > 10485760)) {
            message.warning('字符长度须在 1–10485760 之间')
            return
        }
        if (isNumericType.value) {
            if (form.data_precision !== null && (form.data_precision < 1 || form.data_precision > 1000)) {
                message.warning('numeric 精度须在 1–1000 之间')
                return
            }
            if (form.data_scale !== null && (form.data_precision === null || form.data_scale < 0 || form.data_scale > form.data_precision)) {
                message.warning('小数位须在 0–精度之间，且必须先填写精度')
                return
            }
        }
        if (isTimeType.value && form.data_precision !== null && (form.data_precision < 0 || form.data_precision > 6)) {
            message.warning('时间精度须在 0–6 之间')
            return
        }
        saving.value = true
        const payload = {
            field_name: form.field_name.trim(),
            data_type: form.data_type.trim(),
            data_length: isCharType.value ? form.data_length : null,
            data_precision: isNumericType.value || isTimeType.value ? form.data_precision : null,
            data_scale: isNumericType.value && form.data_precision !== null ? (form.data_scale ?? 0) : null,
            comment: form.comment.trim(),
            remark: form.remark.trim(),
        }
        try {
            if (editingId.value === null) {
                await ApiService.createAdminRoot(payload)
                message.success('字根已新增')
            } else {
                await ApiService.updateAdminRoot(editingId.value, payload)
                message.success('字根已更新，引用表字段需手动同步')
            }
            modalOpen.value = false
            await load()
        } catch (e) {
            handleError(e, '保存字根失败')
        } finally {
            saving.value = false
        }
    }

    async function remove(record: FieldRootInfo) {
        if (record.id === null) return
        try {
            await ApiService.deleteAdminRoot(record.id)
            message.success('字根已删除（引用表字段不受影响）')
            if (rows.value.length === 1 && page.value > 1) {
                page.value -= 1
            }
            await load()
        } catch (e) {
            handleError(e, '删除字根失败')
        }
    }

    function openSync(record: FieldRootInfo) {
        if (record.id === null) return
        syncTarget.value = record
        syncComment.value = true
        syncType.value = false
        syncResults.value = null
        syncOpen.value = true
    }

    async function doSync() {
        const t = syncTarget.value
        if (!t || t.id === null) return
        if (!syncComment.value && !syncType.value) {
            message.warning('请至少选择一项同步内容')
            return
        }
        if (syncType.value && !t.data_type) {
            message.warning('字根未指定标准类型，不能同步类型')
            return
        }
        if (!syncTargets.value.length) {
            message.info('所选属性已一致，无需同步')
            return
        }
        syncing.value = true
        try {
            const res = await ApiService.syncAdminRoot(t.id, syncComment.value, syncType.value)
            syncResults.value = res.results || []
            const failed = syncResults.value.filter(r => !r.ok).length
            if (failed) {
                message.warning(`同步完成：${failed} 张表失败，请查看明细`)
            } else {
                message.success('全部引用表同步成功')
            }
            await load()
        } catch (e) {
            handleError(e, '同步失败')
        } finally {
            syncing.value = false
        }
    }

    onMounted(load)

    return {
        rows, loading, scanning, keyword, page, pageSize, total, columns, forbidden,
        pageSizeOptions: PAGE_SIZE_OPTIONS,
        modalOpen, saving, editingId, form,
        typeOptions: TYPE_OPTIONS, isCharType, isNumericType, isTimeType, formTypeExpr, changeType, changePrecision,
        syncOpen, syncing, syncTarget, syncComment, syncType, syncResults, syncTargets,
        rootStatus, formatTime, search, changePage, scan, openCreate, openRegister, openEdit, submit, remove, openSync, doSync,
        refresh: load,
    }
}
