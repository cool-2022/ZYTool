import { ref, computed } from 'vue'
import { message } from 'ant-design-vue'
import {
    ruleData as mockRuleData,
    flowDataMap,
    flowListMap,
    flowDescriptionMap,
    mainRuleTree as mockMainRuleTree,
    type RuleRecord,
    type FlowNode,
    type FlowListItem,
    type FlowDescription,
    type TreeNode
} from '@/Mock/GCPMRuleData'

export type { RuleRecord, FlowNode, FlowListItem, FlowDescription, TreeNode }

// 表格列定义
export const columns = [
    { title: '编号', dataIndex: 'code', key: 'code', width: 80 },
    { title: '短描述程序', dataIndex: 'shortDesc', key: 'shortDesc', width: 120 },
    { title: '备名或规则描述', dataIndex: 'description', key: 'description', ellipsis: true },
    { title: '规则', dataIndex: 'ruleCount', key: 'ruleCount', width: 60, align: 'center' as const },
    { title: '操作', key: 'actions', width: 80, align: 'center' as const }
]

export function useGCPMRule() {
    // 模拟数据
    const ruleData = mockRuleData

    // 选中的规则
    const selectedRule = ref<RuleRecord | null>(null)

    // 缩放级别
    const zoomLevel = ref(1)

    // 流程节点数据
    const flowNodes = computed<FlowNode[]>(() => {
        if (!selectedRule.value) return []

        // 根据选中的规则返回对应的流程
        return flowDataMap[selectedRule.value.code] || [
            { label: selectedRule.value.code, type: 'start' },
            { label: selectedRule.value.shortDesc || '处理中', type: 'process' },
            { label: 'END', type: 'end' }
        ]
    })

    // 流程列表数据
    const flowList = computed<FlowListItem[]>(() => {
        if (!selectedRule.value) return []
        return flowListMap[selectedRule.value.code] || []
    })

    // 流程描述数据
    const flowDescription = computed<FlowDescription | null>(() => {
        if (!selectedRule.value) return null
        return flowDescriptionMap[selectedRule.value.code] || {
            title: selectedRule.value.shortDesc,
            description: '暂无描述信息',
            input: [],
            output: [],
            logic: ''
        }
    })

    // 当前激活的 Tab
    const activeTab = ref('flowchart')

    // 主规则树形数据
    const mainRuleTree = mockMainRuleTree

    // 展开的树节点
    const expandedKeys = ref<string[]>([])

    // 选中的树节点
    const selectedKeys = ref<string[]>([])

    // 搜索关键词
    const searchKeyword = ref('')

    // 处理树节点展开/折叠
    function handleTreeExpand(keys: string[]) {
        expandedKeys.value = keys
    }

    // 处理树节点选择
    function handleTreeSelect(keys: string[]) {
        selectedKeys.value = keys
    }

    // 处理搜索
    function handleSearch() {
        if (searchKeyword.value) {
            message.info(`搜索: ${searchKeyword.value}`)
        }
    }

    // 处理添加主规则
    function handleAddMainRule() {
        message.info('添加主规则')
    }

    // 处理行点击
    function handleRowClick(record: RuleRecord) {
        selectedRule.value = record
    }

    // 处理行双击
    function handleRowDblClick(record: RuleRecord) {
        selectedRule.value = record
        message.info(`已加载规则: ${record.code} - ${record.shortDesc}`)
    }

    // 新增
    function handleAdd() {
        message.info('新增规则')
    }

    // 编辑
    function handleEdit(record: RuleRecord) {
        message.info(`编辑规则: ${record.code}`)
    }

    // 删除
    function handleDelete(record: RuleRecord) {
        message.warning(`删除规则: ${record.code}`)
    }

    // 刷新
    function handleRefresh() {
        message.success('刷新成功')
    }

    // 放大
    function handleZoomIn() {
        if (zoomLevel.value < 2) {
            zoomLevel.value += 0.1
        }
    }

    // 缩小
    function handleZoomOut() {
        if (zoomLevel.value > 0.5) {
            zoomLevel.value -= 0.1
        }
    }

    // 重置缩放
    function handleResetZoom() {
        zoomLevel.value = 1
    }

    return {
        ruleData,
        selectedRule,
        zoomLevel,
        flowNodes,
        flowList,
        flowDescription,
        activeTab,
        mainRuleTree,
        expandedKeys,
        selectedKeys,
        searchKeyword,
        handleRowClick,
        handleRowDblClick,
        handleAdd,
        handleEdit,
        handleDelete,
        handleRefresh,
        handleZoomIn,
        handleZoomOut,
        handleResetZoom,
        handleTreeExpand,
        handleTreeSelect,
        handleSearch,
        handleAddMainRule
    }
}
