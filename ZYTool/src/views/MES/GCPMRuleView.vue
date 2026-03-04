<template>
    <div class="gcpm-rule-page">
        <div class="container">
            <div class="page-header">
                <a-typography-title :level="2">GCPM规则管理</a-typography-title>
                <a-typography-paragraph>工艺流程规则配置与流程图展示</a-typography-paragraph>
            </div>

            <a-row :gutter="16">
                <!-- 左侧：规则组表格 -->
                <a-col :xs="24" :lg="8">
                    <a-card title="规则组" :bordered="false" class="rule-table-card">
                        <template #extra>
                            <a-space>
                                <a-button type="primary" size="small" @click="handleAdd">
                                    <template #icon>
                                        <PlusOutlined />
                                    </template>
                                    新增
                                </a-button>
                                <a-button size="small" @click="handleRefresh">
                                    <template #icon>
                                        <ReloadOutlined />
                                    </template>
                                    刷新
                                </a-button>
                            </a-space>
                        </template>

                        <a-table :columns="columns" :data-source="ruleData" :pagination="false" size="small"
                            :scroll="{ y: '100%' }" :row-class-name="(record, index) => index % 2 === 1 ? 'table-striped' : ''"
                            @row-click="handleRowClick"
                            :custom-row="(record) => ({
                                onDblclick: () => handleRowDblClick(record)
                            })" class="rule-table-grid">
                            <template #bodyCell="{ column, record }">
                                <template v-if="column.key === 'actions'">
                                    <a-space>
                                        <a-button type="link" size="small" @click.stop="handleEdit(record)">
                                            <EditOutlined />
                                        </a-button>
                                        <a-button type="link" size="small" danger @click.stop="handleDelete(record)">
                                            <DeleteOutlined />
                                        </a-button>
                                    </a-space>
                                </template>
                            </template>
                        </a-table>
                    </a-card>
                </a-col>

                <!-- 中间：流程图展示 -->
                <a-col :xs="24" :lg="10">
                    <a-card title="流程详情" :bordered="false" class="flowchart-card">
                        <a-tabs v-model:activeKey="activeTab" class="flow-tabs">
                            <!-- 流程图 Tab -->
                            <a-tab-pane key="flowchart" tab="流程图">
                                <template #tab>
                                    <span>
                                        <ApartmentOutlined />
                                        流程图
                                    </span>
                                </template>
                                <div class="tab-extra-buttons">
                                    <a-space>
                                        <a-button size="small" @click="handleZoomIn">
                                            <ZoomInOutlined />
                                        </a-button>
                                        <a-button size="small" @click="handleZoomOut">
                                            <ZoomOutOutlined />
                                        </a-button>
                                        <a-button size="small" @click="handleResetZoom">重置</a-button>
                                    </a-space>
                                </div>
                                <div class="flowchart-container" :style="{ transform: `scale(${zoomLevel})` }">
                                    <div v-if="selectedRule" class="flow-content">
                                        <div v-for="(node, index) in flowNodes" :key="index" class="flow-node-wrapper">
                                            <div class="flow-node" :class="node.type">
                                                {{ node.label }}
                                            </div>
                                            <div v-if="index < flowNodes.length - 1" class="flow-arrow">
                                                <ArrowDownOutlined />
                                            </div>
                                        </div>
                                    </div>
                                    <a-empty v-else description="请双击左侧规则查看流程图" />
                                </div>
                            </a-tab-pane>

                            <!-- 流程列表 Tab -->
                            <a-tab-pane key="flowlist" tab="流程列表">
                                <template #tab>
                                    <span>
                                        <UnorderedListOutlined />
                                        流程列表
                                    </span>
                                </template>
                                <div class="flow-list-container">
                                    <a-table v-if="selectedRule && flowList.length" :columns="flowListColumns"
                                        :data-source="flowList" :pagination="false" size="small" bordered>
                                    </a-table>
                                    <a-empty v-else description="请双击左侧规则查看流程列表" />
                                </div>
                            </a-tab-pane>

                            <!-- 流程描述 Tab -->
                            <a-tab-pane key="description" tab="流程描述">
                                <template #tab>
                                    <span>
                                        <FileTextOutlined />
                                        流程描述
                                    </span>
                                </template>
                                <div class="flow-description-container">
                                    <div v-if="selectedRule && flowDescription" class="description-content">
                                        <a-descriptions title="流程信息" bordered :column="1">
                                            <a-descriptions-item label="流程名称">
                                                {{ flowDescription.title }}
                                            </a-descriptions-item>
                                            <a-descriptions-item label="流程描述">
                                                {{ flowDescription.description }}
                                            </a-descriptions-item>
                                            <a-descriptions-item label="输入参数">
                                                <a-tag v-for="(item, idx) in flowDescription.input" :key="idx"
                                                    color="blue">
                                                    {{ item }}
                                                </a-tag>
                                                <span v-if="!flowDescription.input.length">无</span>
                                            </a-descriptions-item>
                                            <a-descriptions-item label="输出结果">
                                                <a-tag v-for="(item, idx) in flowDescription.output" :key="idx"
                                                    color="green">
                                                    {{ item }}
                                                </a-tag>
                                                <span v-if="!flowDescription.output.length">无</span>
                                            </a-descriptions-item>
                                            <a-descriptions-item label="处理逻辑">
                                                <pre class="logic-text">{{ flowDescription.logic || '暂无' }}</pre>
                                            </a-descriptions-item>
                                        </a-descriptions>
                                    </div>
                                    <a-empty v-else description="请双击左侧规则查看流程描述" />
                                </div>
                            </a-tab-pane>
                        </a-tabs>
                    </a-card>
                </a-col>

                <!-- 右侧：主规则树 -->
                <a-col :xs="24" :lg="6">
                    <a-card title="主规则" :bordered="false" class="main-rule-card">
                        <template #extra>
                            <a-space>
                                <a-button type="link" size="small" @click="handleSearch">
                                    <SearchOutlined />
                                    查询
                                </a-button>
                                <a-button type="link" size="small" @click="handleAddMainRule">
                                    <FormOutlined />
                                    附加
                                </a-button>
                            </a-space>
                        </template>

                        <div class="main-rule-content">
                            <a-input v-model:value="searchKeyword" placeholder="输入规则代码进行过滤"
                                @pressEnter="handleSearch" class="search-input">
                                <template #prefix>
                                    <SearchOutlined />
                                </template>
                            </a-input>

                            <a-tree :tree-data="mainRuleTree" 
                                v-model:expanded-keys="expandedKeys"
                                v-model:selected-keys="selectedKeys" 
                                @expand="handleTreeExpand"
                                @select="handleTreeSelect" 
                                class="rule-tree"
                                :show-icon="true" 
                                :show-line="true">
                                <template #icon="{ dataRef }">
                                    <FolderOutlined v-if="dataRef.children && dataRef.children.length > 0" />
                                    <CheckCircleOutlined v-else-if="dataRef.title.includes('KD100') || dataRef.title.includes('KD200') || dataRef.title.includes('KD300')" 
                                        style="color: #52c41a;" />
                                    <CloseCircleOutlined v-else-if="dataRef.isLeaf" style="color: #ff4d4f;" />
                                    <FileOutlined v-else />
                                </template>
                            </a-tree>

                            <div class="tree-footer">
                                <a-typography-text type="secondary" style="font-size: 12px;">
                                    输出代码:K14,输出描述:DT3851D1,输出等级:10组1
                                </a-typography-text>
                                <a-typography-text type="secondary" style="font-size: 12px;">
                                    [CD_O].[炼钢单点] [>] [650]
                                </a-typography-text>
                            </div>
                        </div>
                    </a-card>
                </a-col>
            </a-row>
        </div>
    </div>
</template>

<script setup lang="ts">
import {
    PlusOutlined,
    ReloadOutlined,
    EditOutlined,
    DeleteOutlined,
    ZoomInOutlined,
    ZoomOutOutlined,
    ArrowDownOutlined,
    ApartmentOutlined,
    UnorderedListOutlined,
    FileTextOutlined,
    SearchOutlined,
    FormOutlined,
    FolderOutlined,
    CheckCircleOutlined,
    CloseCircleOutlined,
    FileOutlined
} from '@ant-design/icons-vue'
import { columns, useGCPMRule } from './GCPMRuleView'

// 流程列表表格列定义
const flowListColumns = [
    { title: '步骤', dataIndex: 'step', key: 'step', width: 80, align: 'center' as const },
    { title: '节点代码', dataIndex: 'code', key: 'code', width: 120 },
    { title: '节点名称', dataIndex: 'name', key: 'name' },
    { title: '节点类型', dataIndex: 'type', key: 'type', width: 120 }
]

const {
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
} = useGCPMRule()
</script>

<style scoped src="./GCPMRuleView.css"></style>
