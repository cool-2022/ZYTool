<template>
    <div class="tool-page warm-theme">
        <div class="container">
            <div class="page-header has-tool-badge">
                <div>
                    <a-typography-title :level="2">SQL 可执行性判断</a-typography-title>
                    <a-typography-paragraph>通过前端启发式规则检查 SQL，辅助识别高风险语句</a-typography-paragraph>
                </div>
                <span class="tool-badge static"><SafetyOutlined /> 静态检查</span>
            </div>

            <div class="tool-action-bar">
                <div class="tool-action-group">
                    <a-button type="primary" @click="analyzeSql" :loading="loading"><CheckCircleOutlined /> 检查语句</a-button>
                    <a-button @click="fillExample"><FileTextOutlined /> 填入示例</a-button>
                </div>
                <div class="tool-action-group">
                    <a-button :disabled="!sqlText && !results.length" @click="clearAll"><ClearOutlined /> 清空</a-button>
                </div>
            </div>

            <a-row :gutter="{ xs: 16, sm: 20 }" class="tool-panel-row">
                <a-col :xs="24" :lg="12" class="tool-panel-col">
                    <a-card :bordered="false" class="tool-panel-card text-editor-card">
                        <template #title><span class="tool-card-title"><span class="tool-status-dot"></span>输入 SQL</span></template>
                        <template #extra><span class="tool-card-meta">{{ sqlText.length }} 字符</span></template>
                        <a-textarea class="sql-insert-textarea text-tool-input" v-model:value="sqlText"
                            placeholder="请输入或粘贴 SQL 语句，支持多条分号分隔" :rows="16" />
                    </a-card>
                </a-col>
                <a-col :xs="24" :lg="12" class="tool-panel-col">
                    <a-card :bordered="false" class="tool-panel-card text-editor-card">
                        <template #title><span class="tool-card-title"><span class="tool-status-dot output"></span>检查结果</span></template>
                        <template #extra><span class="tool-card-meta">{{ results.length }} 条语句</span></template>
                        <a-spin :spinning="loading" tip="检查中...">
                            <div class="text-tool-result" :class="{ empty: !results.length }">
                                <div v-if="results.length" class="result-list">
                                    <div v-for="(r, idx) in results" :key="idx" class="result-item">
                                        <div class="result-header">
                                            <a-typography-text strong>语句 {{ idx + 1 }}</a-typography-text>
                                            <a-tag :color="r.severity === 'safe' ? 'green' : r.severity === 'warn' ? 'orange' : 'red'">{{ r.severityLabel }}</a-tag>
                                        </div>
                                        <pre class="sql-snippet">{{ r.statement }}</pre>
                                        <div class="advice"><div v-for="(m, i) in r.messages" :key="i">- {{ m }}</div></div>
                                    </div>
                                </div>
                                <a-empty v-else description="检查完成后，结果会显示在这里" />
                            </div>
                        </a-spin>
                    </a-card>
                </a-col>
            </a-row>

            <a-card title="配置区域" :bordered="false" class="tool-help-card">
                <div class="db-panel">
                    <div class="db-col db-tables">
                        <div class="col-title">表名</div>
                        <div class="col-body">
                            <div v-for="t in tables" :key="t.name" class="item" :class="{ active: selectedTable === t.name }" @click="selectTable(t)">{{ t.name }}</div>
                        </div>
                    </div>
                    <div class="db-col db-columns">
                        <div class="col-title">列名</div>
                        <div class="col-body">
                            <div v-if="selectedColumns.length" class="col-cards">
                                <a-card v-for="c in selectedColumns" :key="c" class="col-card" hoverable @click="insertColumn(c)">
                                    <div class="col-card-content"><span class="col-label">{{ c }}</span></div>
                                </a-card>
                            </div>
                            <div v-else class="empty">请选择左侧表以显示列</div>
                        </div>
                    </div>
                    <div class="db-col db-ops">
                        <div class="col-title">运算符</div>
                        <div class="col-body"><div v-for="op in operators" :key="op" class="item op-item" @click="insertOperator(op)">{{ op }}</div></div>
                    </div>
                </div>
            </a-card>

            <a-card title="检查说明" :bordered="false" class="tool-help-card">
                <p>该工具在前端做静态启发式检查：</p>
                <ul>
                    <li>识别 SELECT、INSERT、UPDATE、DELETE、DROP、ALTER、CREATE 等语句类型</li>
                    <li>检查 UPDATE / DELETE 是否带有 WHERE，提示全表修改风险</li>
                    <li>标记 DDL 等高风险操作，建议先在测试环境验证</li>
                </ul>
                <p>该工具不连接数据库，也不能替代严格的 SQL 解析与数据库安全审查。</p>
            </a-card>
            <div class="tool-page-note"><SafetyOutlined /> 静态分析仅在浏览器中完成，不会执行输入的 SQL</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { SafetyOutlined, CheckCircleOutlined, FileTextOutlined, ClearOutlined } from '@ant-design/icons-vue'

const sqlText = ref('')
const loading = ref(false)
const results = ref<Array<{ statement: string; severity: string; severityLabel: string; messages: string[] }>>([])

// 从 Mock 中导入表结构与运算符数据
import { tables as mockTables, operators as mockOperators } from '@/Mock/SqlRation'

const tables = mockTables
const operators = mockOperators

const selectedTable = ref<string>(tables.value[0]?.name ?? '')
const selectedColumns = ref<string[]>(tables.value[0]?.columns ?? [])

function selectTable(t: { name: string; columns: string[] }) {
    selectedTable.value = t.name
    selectedColumns.value = t.columns
}

function insertAtCursor(insertText: string) {
    // 找到实际的 textarea 元素（ant-design-vue 的 a-textarea 内部包含一个 textarea）
    const ta = document.querySelector('.sql-insert-textarea textarea') as HTMLTextAreaElement | null
    if (!ta) return

    const start = ta.selectionStart ?? ta.value.length
    const end = ta.selectionEnd ?? start
    const newValue = ta.value.slice(0, start) + insertText + ta.value.slice(end)
    ta.value = newValue
    sqlText.value = newValue

    const pos = start + insertText.length
    ta.setSelectionRange(pos, pos)
    ta.focus()
}

function insertColumn(col: string) {
    const prefix = selectedTable.value ? `${selectedTable.value}.` : ''
    insertAtCursor(prefix + col)
}

function insertOperator(op: string) {
    insertAtCursor(` ${op} `)
}

// 简单的启发式分析函数
function analyzeSql() {
    if (!sqlText.value.trim()) {
        message.warning('请输入 SQL 语句')
        return
    }

    loading.value = true
    results.value = []

    setTimeout(() => {
        const raw = sqlText.value

        // 粗略按分号拆分（不会处理引号内的分号）
        const statements = raw.split(';').map(s => s.trim()).filter(Boolean)

        for (const stmt of statements) {
            const normalized = stmt.replace(/\s+/g, ' ').trim()
            const firstWordMatch = normalized.match(/^([a-zA-Z]+)/)
            const firstWord = (firstWordMatch?.[1] ?? '').toUpperCase()

            const entry = { statement: stmt, severity: 'safe', severityLabel: '安全', messages: [] as string[] }

            if (!firstWord) {
                entry.severity = 'unknown'
                entry.severityLabel = '未知'
                entry.messages.push('无法识别语句类型')
                results.value.push(entry)
                continue
            }

            // 高风险 DDL
            const ddl = ['DROP', 'TRUNCATE', 'ALTER', 'CREATE', 'RENAME']
            const dml = ['INSERT', 'UPDATE', 'DELETE']
            const safe = ['SELECT', 'SHOW', 'DESCRIBE', 'EXPLAIN']

            if (ddl.includes(firstWord)) {
                entry.severity = 'danger'
                entry.severityLabel = '高风险（DDL）'
                entry.messages.push('包含 DDL 操作，可能修改或删除表结构或数据，慎重执行')
            } else if (dml.includes(firstWord)) {
                // 对 UPDATE/DELETE 进一步检查 WHERE
                const upper = normalized.toUpperCase()
                if ((firstWord === 'UPDATE' || firstWord === 'DELETE')) {
                    // 检查是否包含 WHERE 子句（简单检测，不考虑子查询等复杂场景）
                    if (!/\bWHERE\b/.test(upper)) {
                        entry.severity = 'danger'
                        entry.severityLabel = '高风险（无 WHERE）'
                        entry.messages.push('UPDATE/DELETE 未检测到 WHERE 子句，可能会影响全表，请补充条件或在事务中执行并先做备份')
                    } else {
                        entry.severity = 'warn'
                        entry.severityLabel = '警告（有 WHERE）'
                        entry.messages.push('检测到 WHERE，但仍建议在备份/事务中执行，并先在测试库验证')
                    }
                } else {
                    // INSERT
                    entry.severity = 'warn'
                    entry.severityLabel = '警告（INSERT）'
                    entry.messages.push('INSERT 会写入数据，请确认目标表及字段是否正确')
                }
            } else if (safe.includes(firstWord)) {
                entry.severity = 'safe'
                entry.severityLabel = '安全（只读）'
                entry.messages.push('只读语句，一般可以安全执行（仍需注意权限与资源）')
            } else {
                // 默认谨慎处理
                entry.severity = 'warn'
                entry.severityLabel = '未知/中性'
                entry.messages.push('无法完全判断语句是否安全，建议在测试环境验证')
            }

            // 额外检查：是否以分号结尾（用户输入多条时我们已拆分），提示写事务
            if (/\bBEGIN\b|\bCOMMIT\b|\bROLLBACK\b/i.test(stmt)) {
                entry.messages.push('检测到事务控制语句，请确认事务边界是否正确')
            }

            results.value.push(entry)
        }

        loading.value = false
    }, 200)
}

function fillExample() {
    sqlText.value = `-- 示例：安全查询\nSELECT id, name FROM users WHERE status = 'active';\n\n-- 示例：危险（无 WHERE）\nDELETE FROM orders;\n\n-- 示例：DDL（高风险）\nDROP TABLE audit_log;`
}

function clearAll() {
    sqlText.value = ''
    results.value = []
}
</script>

<style scoped>
.tool-page {
    min-height: 100vh;
    background: var(--gradient-bg);
    padding: 2rem var(--page-gutter);
}

.container {
    max-width: var(--page-max-width);
    margin: 0 auto;
    padding: 0;
}

.page-header {
    /* 配置区域三列布局：10% / 80% / 10% */
    margin-bottom: 2rem;
}

.result-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
}

.result-item {
    background: var(--bg-primary);
    border-radius: 8px;
    padding: 12px;
    border: 1px solid var(--border-color);
}

.result-header {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 8px;
}

.sql-snippet {
    margin: 0;
    font-family: 'Courier New', monospace;
    background: var(--bg-secondary);
    padding: 8px;
    border-radius: 6px;
    overflow: auto;
}

.advice {
    margin-top: 8px;
    color: var(--text-primary);
}



/* 配置区域三列布局：10% / 80% / 10% */
.db-panel {
    display: flex;
    gap: 12px;
    align-items: flex-start;
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
}

.db-col {
    background: var(--bg-primary);
    border-radius: 6px;
    padding: 8px;
    box-sizing: border-box;
}

.db-tables {
    flex: 0 0 9%;
    max-width: 9%;
    min-width: 78px;
}

.db-columns {
    flex: 0 0 76%;
    max-width: 76%;
    min-width: 225px;
}

.db-ops {
    flex: 0 0 12%;
    max-width: 12%;
    min-width: 98px;
}

.col-title {
    font-weight: 600;
    margin-bottom: 8px;
}

.col-body {
    max-height: 160px;
    overflow: auto;
}

.item {
    padding: 6px 8px;
    border-radius: 4px;
    cursor: pointer;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.item:hover {
    background: rgba(0, 0, 0, 0.04);
}

.item.active {
    background: var(--primary-color);
    color: white;
}

/* 列名卡片样式 */
.col-cards {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 8px;
}

.col-card {
    padding: 8px;
    cursor: pointer;
    text-align: left;
    border-radius: 6px;
    transition: transform var(--duration-press) var(--ease-out), box-shadow var(--duration-press) var(--ease-out);
    background: var(--bg-secondary);
}

.col-card:hover {
    transform: translateY(-6px);
    box-shadow: var(--shadow-md);
}

.col-card-content {
    font-family: 'Courier New', monospace;
    font-size: 13px;
    color: var(--text-primary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    display: flex;
    align-items: center;
}

.col-icon {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 18px;
    height: 18px;
    margin-right: 8px;
    font-size: 14px;
    opacity: 0.9;
}

.col-label {
    font-weight: 700;
    overflow: hidden;
    text-overflow: ellipsis;
}

/* 保持三列显示，允许横向滚动以适配窄屏 */
</style>
