import { ref } from 'vue'

export interface RuleRecord {
    key: string
    code: string
    shortDesc: string
    description: string
    ruleCount: number
}

export interface FlowNode {
    label: string
    type: 'start' | 'process' | 'decision' | 'end'
}

export interface FlowListItem {
    step: number
    code: string
    name: string
    type: string
}

export interface FlowDescription {
    title: string
    description: string
    input: string[]
    output: string[]
    logic: string
}

export interface TreeNode {
    title: string
    key: string
    children?: TreeNode[]
    isLeaf?: boolean
}

// 规则组表格数据
export const ruleData = ref<RuleRecord[]>([
    { key: 'A10002', code: 'A10002', shortDesc: '获取铁环实绩', description: '', ruleCount: 0 },
    { key: 'A10001', code: 'A10001', shortDesc: '获取炉次实绩', description: '', ruleCount: 0 },
    { key: 'B10001', code: 'B10001', shortDesc: '钢水纯净度判定', description: '', ruleCount: 0 },
    { key: 'C10001', code: 'C10001', shortDesc: '游离氧判定', description: '', ruleCount: 0 },
    { key: 'D10001', code: 'D10001', shortDesc: '炉次温度判定', description: '', ruleCount: 0 },
    { key: 'E10001', code: 'E10001', shortDesc: '大OB判定', description: '', ruleCount: 0 },
    { key: 'F020000', code: 'F020000', shortDesc: '低拉速', description: '', ruleCount: 0 },
    { key: 'F010000', code: 'F010000', shortDesc: '液面波动', description: '', ruleCount: 0 },
    { key: 'FA0001', code: 'FA0001', shortDesc: '铸坯等级-特征值计算', description: '', ruleCount: 0 },
    { key: 'FC0001', code: 'FC0001', shortDesc: '铸坯等级判定-最终等级', description: '', ruleCount: 0 },
    { key: 'FB0001', code: 'FB0001', shortDesc: '铸坯等级判定-异常等级', description: '', ruleCount: 0 },
    { key: 'G10001', code: 'G10001', shortDesc: '铸坯品质异常代码判定 - 板坯', description: '', ruleCount: 0 },
    { key: 'G10002', code: 'G10002', shortDesc: '铸坯品质异常代码判定 - 炉次', description: '', ruleCount: 0 },
    { key: 'H10000', code: 'H10000', shortDesc: 'CP在线监控判定', description: '', ruleCount: 0 },
    { key: 'J10000', code: 'J10000', shortDesc: '炉次过程监控', description: '', ruleCount: 0 },
    { key: 'K10000', code: 'K10000', shortDesc: '弹簧DQ1451H5', description: '', ruleCount: 0 },
    { key: 'L10000', code: 'L10000', shortDesc: '炉次品质异常判定', description: '', ruleCount: 0 }
])

// 流程图数据映射
export const flowDataMap: Record<string, FlowNode[]> = {
    'A10001': [
        { label: 'A10001', type: 'start' },
        { label: 'AA0001', type: 'process' },
        { label: 'AB0001', type: 'process' },
        { label: 'AC0001', type: 'process' },
        { label: 'END', type: 'end' }
    ],
    'A10002': [
        { label: 'A10002', type: 'start' },
        { label: '获取铁环实绩', type: 'process' },
        { label: 'END', type: 'end' }
    ],
    'B10001': [
        { label: 'B10001', type: 'start' },
        { label: '钢水纯净度判定', type: 'process' },
        { label: 'END', type: 'end' }
    ],
    'C10001': [
        { label: 'C10001', type: 'start' },
        { label: '游离氧判定', type: 'process' },
        { label: 'END', type: 'end' }
    ],
    'D10001': [
        { label: 'D10001', type: 'start' },
        { label: '炉次温度判定', type: 'process' },
        { label: 'END', type: 'end' }
    ]
}

// 流程列表数据映射
export const flowListMap: Record<string, FlowListItem[]> = {
    'A10001': [
        { step: 1, code: 'A10001', name: '获取炉次实绩', type: '开始节点' },
        { step: 2, code: 'AA0001', name: '数据验证', type: '处理节点' },
        { step: 3, code: 'AB0001', name: '数据转换', type: '处理节点' },
        { step: 4, code: 'AC0001', name: '结果输出', type: '处理节点' },
        { step: 5, code: 'END', name: '流程结束', type: '结束节点' }
    ],
    'A10002': [
        { step: 1, code: 'A10002', name: '获取铁环实绩', type: '开始节点' },
        { step: 2, code: '获取铁环实绩', name: '查询铁环数据', type: '处理节点' },
        { step: 3, code: 'END', name: '流程结束', type: '结束节点' }
    ]
}

// 流程描述数据映射
export const flowDescriptionMap: Record<string, FlowDescription> = {
    'A10001': {
        title: '获取炉次实绩',
        description: '该流程用于获取和处理炉次生产实绩数据，包括数据验证、转换和输出等步骤。',
        input: ['炉次号', '生产时间', '钢种代码'],
        output: ['炉次实绩数据', '验证结果', '处理状态'],
        logic: '1. 接收炉次号和相关参数\n2. 验证输入数据的完整性和有效性\n3. 从数据库查询炉次实绩信息\n4. 对数据进行标准化转换\n5. 输出处理结果'
    },
    'A10002': {
        title: '获取铁环实绩',
        description: '该流程用于获取铁环生产实绩数据。',
        input: ['铁环编号', '查询时间范围'],
        output: ['铁环实绩数据', '查询状态'],
        logic: '1. 接收铁环编号\n2. 查询铁环生产数据\n3. 返回查询结果'
    }
}

// 主规则树形数据
export const mainRuleTree = ref<TreeNode[]>([
    {
        title: '铸坯过程条件判定',
        key: '0-0',
        children: [
            { title: 'KD700（中间包吹位波动>10t或中间包吹氩时间<15min,改AP1120E1）', key: '0-0-0', isLeaf: true },
            { title: 'KD800（上水口+塞棒氩气均值>10L/min,改AP1120E1）', key: '0-0-1', isLeaf: true },
            { title: 'KD900（两侧拔热量差>150kcal/m2.h℃,改AP1120E1）', key: '0-0-2', isLeaf: true },
            { title: 'KDA01（转炉废吹游离氧>650,改DT3851D1）', key: '0-0-3', isLeaf: true },
            { title: 'KDC00（未开结晶器电磁搅拌,改AP1120E1）', key: '0-0-4', isLeaf: true },
            { title: 'KDD00（板坯存在调宽,改AP1120E1）', key: '0-0-5', isLeaf: true },
            { title: 'KDE01（RH前游离氧>590,改DQ199D1）', key: '0-0-6', isLeaf: true },
            { title: 'KDF01（中间包连浇炉数>5,改DT3851D1）', key: '0-0-7', isLeaf: true },
            { title: 'KD100（镇静时间<15min,改AP1120E1）', key: '0-0-8', isLeaf: true },
            { title: 'KD104（电池壳钢,镇静时间<15min,并>5min,改AP1120E1）', key: '0-0-9', isLeaf: true },
            { title: 'KD200（纯脱气时间<10min,改AP1120E1）', key: '0-0-10', isLeaf: true },
            { title: 'KD201（纯脱气时间<10min且>5min,改AP1120E1）', key: '0-0-11', isLeaf: true },
            { title: 'KD251（高真空时间<9.5min且>5min,改AP1120E1）', key: '0-0-12', isLeaf: true },
            { title: 'KD280（RH极限真空度>266Pa,改AQ1020E1）', key: '0-0-13', isLeaf: true },
            { title: 'KD300（OB>100Nm3,改AP1120E1）', key: '0-0-14', isLeaf: true },
            { title: 'KD301（OB>0且<500Nm3且纯脱气时间<10min,改AP1120E1）', key: '0-0-15', isLeaf: true },
            { title: 'KD400（中间包过热度<15℃,改AP1120E1）', key: '0-0-16', isLeaf: true },
            { title: 'KD406（中间包过热度<18℃,按A001决策树判定）', key: '0-0-17', isLeaf: true },
            { title: 'KD500（液面波动均值>10或极差大于13,改AP1120E1）', key: '0-0-18', isLeaf: true },
            { title: 'KD600（拉速<0.9m/min或拉速波动>0.4m/min,改AP1120E1）', key: '0-0-19', isLeaf: true }
        ]
    },
    {
        title: '炼钢纯水纯净度判定',
        key: '0-1',
        children: []
    },
    {
        title: '铸坯等级判定-特征值计算',
        key: '0-2',
        children: []
    },
    {
        title: '铸坯等级判定-液面波动',
        key: '0-3',
        children: []
    },
    {
        title: '铸坯等级判定-低拉速',
        key: '0-4',
        children: []
    },
    {
        title: '铸坯等级判定-异常等级',
        key: '0-5',
        children: []
    },
    {
        title: '铸坯等级判定-最终等级',
        key: '0-6',
        children: []
    },
    {
        title: '炉次过程监控',
        key: '0-7',
        children: []
    },
    {
        title: '铸坯位置条件判定',
        key: '0-8',
        children: []
    },
    {
        title: '铸坯铸机条件判定',
        key: '0-9',
        children: []
    },
    {
        title: '铸坯成分条件判定',
        key: '0-10',
        children: []
    },
    {
        title: 'END(结束节点)',
        key: '0-11',
        isLeaf: true
    }
])
