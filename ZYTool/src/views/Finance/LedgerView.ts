import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from 'vue'
import { message } from 'ant-design-vue'

export type RecordType = 'expense' | 'income'
export interface LedgerRecord {
  id: string
  amount: number
  type: RecordType
  category: string
  date: string
  note: string
}
interface LedgerData {
  version: 1
  demo: boolean
  records: LedgerRecord[]
  budgets: Record<string, Record<string, number>>
}

const STORAGE_KEY = 'zytool-little-ledger-v1'
const MAX_AMOUNT = 999999999
export const expenseCategories = [
  { name: '餐饮美食', color: '#ed896c', icon: 'food', budget: 150000 },
  { name: '购物消费', color: '#e6b561', icon: 'shopping', budget: 100000 },
  { name: '交通出行', color: '#87a99c', icon: 'travel', budget: 50000 },
  { name: '居家生活', color: '#9a9abd', icon: 'home', budget: 300000 },
  { name: '休闲娱乐', color: '#c99ab0', icon: 'fun', budget: 60000 },
  { name: '医疗健康', color: '#7ca9b7', icon: 'health', budget: 40000 },
  { name: '其他支出', color: '#aaa59b', icon: 'other', budget: 30000 },
]
const incomeCategories = ['工资薪酬', '兼职收入', '投资收益', '其他收入']

function localDate(date = new Date()) {
  return `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`
}
function monthAt(month: string, offset: number) {
  const [year, value] = month.split('-').map(Number) as [number, number]
  const date = new Date(year, value - 1 + offset, 1)
  return localDate(date).slice(0, 7)
}
function validDate(value: unknown): value is string {
  if (typeof value !== 'string' || !/^\d{4}-\d{2}-\d{2}$/.test(value)) return false
  const date = new Date(`${value}T12:00:00`)
  return Number.isFinite(date.getTime()) && localDate(date) === value && value >= '1900-01-01' && value <= '2100-12-31'
}
function validCents(value: unknown): value is number {
  return typeof value === 'number' && Number.isSafeInteger(value) && value >= 0 && value <= MAX_AMOUNT
}
function parseData(raw: string): LedgerData {
  const data = JSON.parse(raw)
  if (data?.version !== 1 || typeof data.demo !== 'boolean' || !Array.isArray(data.records) || !data.budgets || typeof data.budgets !== 'object' || Array.isArray(data.budgets)) throw new Error('Invalid ledger')
  const ids = new Set<string>()
  for (const record of data.records) {
    const categories = record?.type === 'expense' ? expenseCategories.map(item => item.name) : record?.type === 'income' ? incomeCategories : []
    if (typeof record?.id !== 'string' || !record.id || ids.has(record.id) || !validCents(record.amount) || record.amount === 0 || !categories.includes(record.category) || !validDate(record.date) || typeof record.note !== 'string' || record.note.length > 200) throw new Error('Invalid record')
    ids.add(record.id)
  }
  for (const [month, budgets] of Object.entries(data.budgets)) {
    if (!/^\d{4}-(0[1-9]|1[0-2])$/.test(month) || !budgets || typeof budgets !== 'object' || Array.isArray(budgets)) throw new Error('Invalid budgets')
    for (const [category, amount] of Object.entries(budgets)) {
      if (!expenseCategories.some(item => item.name === category) || !validCents(amount)) throw new Error('Invalid budget')
    }
  }
  return data
}
function createDemo(): LedgerData {
  const current = localDate().slice(0, 7)
  const records: LedgerRecord[] = []
  const budgets: LedgerData['budgets'] = {}
  for (const [index, month] of [current, monthAt(current, -1)].entries()) {
    budgets[month] = Object.fromEntries(expenseCategories.map(item => [item.name, item.budget]))
    const rows: [number, RecordType, string, number, string][] = [
      [5, 'income', '工资薪酬', 1280000, '每个月的努力，都有回报'],
      [1, 'expense', '居家生活', 260000, '这个月的房租'],
      [3, 'expense', '餐饮美食', 16800, '和朋友吃一顿火锅'],
      [6, 'expense', '交通出行', 5800, '地铁交通卡充值'],
      [8, 'expense', '购物消费', 32900, '换一双舒服的运动鞋'],
      [10, 'expense', '医疗健康', 8600, '日常体检'],
      [12, 'expense', '餐饮美食', 3280, '午餐和一杯咖啡'],
      [15, 'income', '兼职收入', 150000, '设计项目结款'],
      [16, 'expense', '购物消费', 79900, '买了心仪的小家电'],
      [18, 'expense', '居家生活', 13800, '水电燃气费'],
      [20, 'expense', '休闲娱乐', 12800, '周末看场电影'],
      [22, 'expense', '餐饮美食', 6800, '下班后的美味晚餐'],
      [24, 'expense', '交通出行', 2600, '周末出行打车'],
      [25, 'expense', '其他支出', 5000, '给朋友准备小礼物'],
      [27, 'expense', '餐饮美食', 4850, '早餐、午餐和水果'],
      [28, 'expense', '休闲娱乐', 19900, '喜欢的音乐现场'],
    ]
    for (const [day, type, category, amount, note] of rows) {
      if (index === 0 && day > new Date().getDate()) continue
      records.push({ id: crypto.randomUUID(), amount: Math.round(amount * (index === 0 ? 1 : 0.9)), type, category, date: `${month}-${String(day).padStart(2, '0')}`, note })
    }
  }
  return { version: 1, demo: true, records, budgets }
}

export function useLedgerView() {
  const storageError = ref('')
  const storageLocked = ref(false)
  let initial: LedgerData
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    initial = raw === null ? createDemo() : parseData(raw)
    if (raw === null) localStorage.setItem(STORAGE_KEY, JSON.stringify(initial))
  } catch {
    initial = { version: 1, demo: false, records: [], budgets: {} }
    storageLocked.value = true
    storageError.value = '无法读取或保存本地账本，已暂停写入以保护原数据。请检查浏览器存储权限，或明确确认重置演示数据后覆盖原账本。'
  }
  const data = ref(initial)
  const selectedMonth = ref(localDate().slice(0, 7))
  const activeView = ref('overview')
  const filterType = ref('all')
  const filterCategory = ref('all')
  const keyword = ref('')
  const page = ref(1)
  const recordModalOpen = ref(false)
  const budgetModalOpen = ref(false)
  const resetModalOpen = ref(false)
  const deleteTarget = ref<LedgerRecord | null>(null)
  const editingId = ref<string | null>(null)
  const formError = ref('')
  const budgetError = ref('')
  const form = reactive({ amount: null as number | null, type: 'expense' as RecordType, category: '餐饮美食', date: localDate(), note: '' })
  const budgetDraft = reactive<Record<string, number | null>>({})
  const previousTitle = document.title

  onMounted(() => { document.title = '小账本 · 个人收支管理' })
  onBeforeUnmount(() => { document.title = previousTitle })

  const currency = (cents: number) => (cents / 100).toLocaleString('zh-CN', { minimumFractionDigits: 2, maximumFractionDigits: 2 })
  const monthLabel = computed(() => `${Number(selectedMonth.value.slice(0, 4))}年${Number(selectedMonth.value.slice(5))}月`)
  const daysInMonth = computed(() => {
    const [year, month] = selectedMonth.value.split('-').map(Number) as [number, number]
    return new Date(year, month, 0).getDate()
  })
  const monthRecords = computed(() => data.value.records.filter(record => record.date.startsWith(selectedMonth.value)).sort((a, b) => b.date.localeCompare(a.date) || b.id.localeCompare(a.id)))
  const income = computed(() => monthRecords.value.filter(record => record.type === 'income').reduce((sum, record) => sum + record.amount, 0))
  const expense = computed(() => monthRecords.value.filter(record => record.type === 'expense').reduce((sum, record) => sum + record.amount, 0))
  const balance = computed(() => income.value - expense.value)
  const categoryStats = computed(() => expenseCategories.map(category => {
    const spent = monthRecords.value.filter(record => record.type === 'expense' && record.category === category.name).reduce((sum, record) => sum + record.amount, 0)
    const budget = data.value.budgets[selectedMonth.value]?.[category.name] ?? 0
    return { ...category, spent, budget, percent: budget > 0 ? Math.round(spent / budget * 100) : 0, over: budget > 0 && spent > budget }
  }))
  const totalBudget = computed(() => categoryStats.value.reduce((sum, category) => sum + category.budget, 0))
  const budgetRemaining = computed(() => totalBudget.value - expense.value)
  const budgetUsed = computed(() => totalBudget.value > 0 ? Math.round(expense.value / totalBudget.value * 100) : 0)
  const overCategories = computed(() => categoryStats.value.filter(category => category.over))
  const unbudgetedExpense = computed(() => categoryStats.value.filter(category => category.budget === 0).reduce((sum, category) => sum + category.spent, 0))
  const donutSegments = computed(() => {
    let offset = 0
    return categoryStats.value.filter(category => category.spent > 0).sort((a, b) => b.spent - a.spent).map(category => {
      const share = category.spent / expense.value
      const result = { ...category, share: Math.round(share * 100), dash: `${share * 100} ${100 - share * 100}`, offset: -offset }
      offset += share * 100
      return result
    })
  })
  const dailyExpenses = computed(() => Array.from({ length: daysInMonth.value }, (_, index) => {
    const date = `${selectedMonth.value}-${String(index + 1).padStart(2, '0')}`
    return { day: index + 1, amount: monthRecords.value.filter(record => record.type === 'expense' && record.date === date).reduce((sum, record) => sum + record.amount, 0) }
  }))
  const chartMax = computed(() => Math.max(10000, Math.ceil(Math.max(...dailyExpenses.value.map(day => day.amount)) / 10000) * 10000))
  const chartPoints = computed(() => dailyExpenses.value.map((day, index) => ({ ...day, x: 46 + index / (daysInMonth.value - 1) * 594, y: 184 - day.amount / chartMax.value * 148 })))
  const trendPath = computed(() => chartPoints.value.map((point, index) => `${index === 0 ? 'M' : 'L'}${point.x},${point.y}`).join(' '))
  const trendArea = computed(() => `${trendPath.value} L640,184 L46,184 Z`)
  const peakDay = computed(() => dailyExpenses.value.reduce((peak, day) => day.amount > peak.amount ? day : peak, { day: 1, amount: 0 }))
  const categoryOptions = computed(() => (form.type === 'expense' ? expenseCategories.map(category => category.name) : incomeCategories).map(name => ({ label: name, value: name })))
  const filterCategoryOptions = computed(() => [
    { label: '全部分类', value: 'all' },
    ...(filterType.value === 'income' ? incomeCategories : filterType.value === 'expense' ? expenseCategories.map(category => category.name) : [...expenseCategories.map(category => category.name), ...incomeCategories]).map(name => ({ label: name, value: name })),
  ])
  const filteredRecords = computed(() => monthRecords.value.filter(record =>
    (filterType.value === 'all' || record.type === filterType.value) &&
    (filterCategory.value === 'all' || record.category === filterCategory.value) &&
    `${record.note} ${record.category}`.toLowerCase().includes(keyword.value.trim().toLowerCase()),
  ))
  const displayedRecords = computed(() => activeView.value === 'overview' ? filteredRecords.value.slice(0, 5) : filteredRecords.value.slice((page.value - 1) * 8, page.value * 8))
  const hasFilters = computed(() => filterType.value !== 'all' || filterCategory.value !== 'all' || keyword.value.trim() !== '')

  watch(() => form.type, () => { form.category = categoryOptions.value[0]!.value }, { flush: 'sync' })
  watch(filterType, () => { filterCategory.value = 'all' })
  watch([selectedMonth, filterType, filterCategory, keyword, activeView], () => { page.value = 1 })
  watch(() => filteredRecords.value.length, count => { page.value = Math.min(page.value, Math.max(1, Math.ceil(count / 8))) })

  function persist(next: LedgerData, overwrite = false) {
    if (storageLocked.value && !overwrite) {
      message.error('原账本读取失败，已暂停写入。请先检查存储，或确认重置演示数据。')
      return false
    }
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(next))
      data.value = next
      storageLocked.value = false
      storageError.value = ''
      return true
    } catch {
      storageError.value = '保存失败，请检查浏览器存储权限或剩余空间。本次修改未保存，原账本仍保留。'
      message.error('保存失败，本次修改未生效')
      return false
    }
  }
  function changeMonth(offset: number) {
    const next = monthAt(selectedMonth.value, offset)
    if (next >= '1900-01' && next <= '2100-12') selectedMonth.value = next
  }
  function selectMonth(value: string | null) {
    if (value && /^\d{4}-(0[1-9]|1[0-2])$/.test(value) && value >= '1900-01' && value <= '2100-12') selectedMonth.value = value
  }
  function openRecord(record?: LedgerRecord) {
    editingId.value = record?.id ?? null
    formError.value = ''
    Object.assign(form, { amount: record ? record.amount / 100 : null, type: record?.type ?? 'expense', category: record?.category ?? '餐饮美食', date: record?.date ?? (selectedMonth.value === localDate().slice(0, 7) ? localDate() : `${selectedMonth.value}-01`), note: record?.note ?? '' })
    recordModalOpen.value = true
  }
  function saveRecord() {
    const amount = form.amount === null ? NaN : Math.round(form.amount * 100)
    if (!Number.isFinite(form.amount) || !validCents(amount) || amount <= 0 || Math.abs(form.amount! * 100 - amount) > 0.00001) {
      formError.value = '请输入 0.01～9,999,999.99 元的金额，最多两位小数。'
      return
    }
    if (!validDate(form.date)) { formError.value = '请选择 1900～2100 年之间的有效日期。'; return }
    if (!categoryOptions.value.some(category => category.value === form.category)) { formError.value = '请选择对应收支类型的分类。'; return }
    if (form.note.length > 200) { formError.value = '备注最多填写 200 个字。'; return }
    const record: LedgerRecord = { id: editingId.value ?? crypto.randomUUID(), amount, type: form.type, category: form.category, date: form.date, note: form.note.trim() }
    const records = editingId.value ? data.value.records.map(item => item.id === editingId.value ? record : item) : [...data.value.records, record]
    if (!persist({ ...data.value, records })) return
    selectedMonth.value = record.date.slice(0, 7)
    recordModalOpen.value = false
    message.success(editingId.value ? '账单已更新' : '记好啦，又是认真生活的一天')
  }
  function deleteRecord() {
    if (!deleteTarget.value) return
    if (persist({ ...data.value, records: data.value.records.filter(record => record.id !== deleteTarget.value!.id) })) {
      deleteTarget.value = null
      message.success('账单已删除')
    }
  }
  function openBudgets() {
    budgetError.value = ''
    for (const category of categoryStats.value) budgetDraft[category.name] = category.budget / 100
    budgetModalOpen.value = true
  }
  function saveBudgets() {
    const budgets: Record<string, number> = {}
    for (const category of expenseCategories) {
      const value = budgetDraft[category.name]
      const cents = value === null || value === undefined ? NaN : Math.round(value * 100)
      if (!Number.isFinite(value) || !validCents(cents) || Math.abs(value! * 100 - cents) > 0.00001) {
        budgetError.value = `${category.name}：请输入 0～9,999,999.99 元，最多两位小数。`
        return
      }
      budgets[category.name] = cents
    }
    if (persist({ ...data.value, budgets: { ...data.value.budgets, [selectedMonth.value]: budgets } })) {
      budgetModalOpen.value = false
      message.success(`${monthLabel.value}预算已保存`)
    }
  }
  function resetDemo() {
    if (persist(createDemo())) {
      selectedMonth.value = localDate().slice(0, 7)
      clearFilters()
      resetModalOpen.value = false
      message.success('演示账本已重置')
    }
  }
  function clearFilters() { filterType.value = 'all'; filterCategory.value = 'all'; keyword.value = '' }
  function categoryMeta(record: LedgerRecord) {
    return expenseCategories.find(category => category.name === record.category) ?? { color: '#76a18a', icon: 'income' }
  }

  return {
    data, storageError, selectedMonth, activeView, monthLabel, filterType, filterCategory, keyword, page,
    recordModalOpen, budgetModalOpen, resetModalOpen, deleteTarget, editingId, form, formError, budgetDraft, budgetError,
    currency, monthRecords, income, expense, balance, categoryStats, totalBudget, budgetRemaining, budgetUsed,
    overCategories, unbudgetedExpense, donutSegments, dailyExpenses, chartMax, chartPoints, trendPath, trendArea, peakDay,
    categoryOptions, filterCategoryOptions, displayedRecords, filteredRecords, hasFilters,
    changeMonth, selectMonth, openRecord, saveRecord, deleteRecord, openBudgets, saveBudgets, resetDemo, clearFilters, categoryMeta,
  }
}
