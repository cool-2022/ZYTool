<script setup lang="ts">
import { computed, inject, ref } from 'vue'
import type { Ref } from 'vue'
import { theme as antdTheme } from 'ant-design-vue'
import {
  BookOutlined, PlusOutlined, LeftOutlined, RightOutlined, ArrowUpOutlined, ArrowDownOutlined,
  WalletOutlined, PieChartOutlined, UnorderedListOutlined, AimOutlined, SafetyOutlined,
  SearchOutlined, EditOutlined, DeleteOutlined, ReloadOutlined, CoffeeOutlined, ShoppingOutlined,
  CarOutlined, HomeOutlined, SmileOutlined, MedicineBoxOutlined, EllipsisOutlined, RiseOutlined,
  ExclamationCircleOutlined, CheckCircleOutlined, CalendarOutlined,
} from '@ant-design/icons-vue'
import { useLedgerView } from './LedgerView'

const {
  data, storageError, selectedMonth, activeView, monthLabel, filterType, filterCategory, keyword, page,
  recordModalOpen, budgetModalOpen, resetModalOpen, deleteTarget, editingId, form, formError, budgetDraft, budgetError,
  currency, monthRecords, income, expense, balance, categoryStats, totalBudget, budgetRemaining, budgetUsed,
  overCategories, unbudgetedExpense, donutSegments, dailyExpenses, chartMax, chartPoints, trendPath, trendArea, peakDay,
  categoryOptions, filterCategoryOptions, displayedRecords, filteredRecords, hasFilters,
  changeMonth, selectMonth, openRecord, saveRecord, deleteRecord, openBudgets, saveBudgets, resetDemo, clearFilters, categoryMeta,
} = useLedgerView()

const categoryIcons = {
  food: CoffeeOutlined, shopping: ShoppingOutlined, travel: CarOutlined, home: HomeOutlined,
  fun: SmileOutlined, health: MedicineBoxOutlined, other: EllipsisOutlined, income: RiseOutlined,
}
const pageTheme = inject<Readonly<Ref<'light' | 'dark'>>>('theme', ref<'light' | 'dark'>('light'))
const ledgerTheme = computed(() => {
  const dark = pageTheme.value === 'dark'
  return {
    algorithm: dark ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm,
    token: {
      colorPrimary: dark ? '#ed9b7e' : '#cf654a',
      colorInfo: dark ? '#ed9b7e' : '#cf654a',
      colorSuccess: dark ? '#a0c0a8' : '#5d8d74',
      colorError: dark ? '#ec9a8e' : '#c45646',
      colorWarning: dark ? '#d1b17f' : '#b78239',
      colorBgBase: dark ? '#24221f' : '#f8f6f0',
      colorBgContainer: dark ? '#2e2b27' : '#fffdf9',
      colorBgElevated: dark ? '#2e2b27' : '#fffdf9',
      colorText: dark ? '#eee8df' : '#423e38',
      colorTextSecondary: dark ? '#c0b4a4' : '#81786e',
      colorTextTertiary: dark ? '#aa9c8c' : '#95897b',
      colorTextLightSolid: dark ? '#24221f' : '#fff',
      colorBorder: dark ? '#433c33' : '#e8e2d8',
      colorBorderSecondary: dark ? '#433c33' : '#ece6dc',
      colorPrimaryBg: dark ? '#443027' : '#f3e7dd',
      borderRadius: 9,
      fontFamily: 'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", "PingFang SC", "Microsoft YaHei", sans-serif',
    },
  }
})
const popupContainer = (trigger: HTMLElement) => trigger.parentElement ?? trigger
</script>

<template>
  <a-config-provider :theme="ledgerTheme">
    <main class="ledger-page">
      <div class="ledger-shell">
        <header class="ledger-header">
          <div class="ledger-brand">
            <span class="ledger-mark"><BookOutlined /></span>
            <div><h1>小账本<span class="brand-dot">.</span></h1><p>记下日常，让生活心里有数</p></div>
          </div>
          <div class="ledger-header-actions">
            <span class="local-badge"><SafetyOutlined /> 仅保存在本机</span>
            <a-button type="primary" size="large" class="add-record" @click="openRecord()"><PlusOutlined /> 记一笔</a-button>
          </div>
        </header>

        <div class="ledger-toolbar">
          <nav class="ledger-tabs" aria-label="账本视图">
            <a-button :class="{ selected: activeView === 'overview' }" :aria-pressed="activeView === 'overview'" @click="activeView = 'overview'"><PieChartOutlined /> 收支概览</a-button>
            <a-button :class="{ selected: activeView === 'records' }" :aria-pressed="activeView === 'records'" @click="activeView = 'records'"><UnorderedListOutlined /> 全部账单</a-button>
            <a-button :class="{ selected: activeView === 'budgets' }" :aria-pressed="activeView === 'budgets'" @click="activeView = 'budgets'"><AimOutlined /> 预算管理</a-button>
          </nav>
          <div class="month-switcher">
            <a-button type="text" aria-label="上个月" :disabled="selectedMonth === '1900-01'" @click="changeMonth(-1)"><LeftOutlined /></a-button>
            <a-date-picker :value="selectedMonth" picker="month" value-format="YYYY-MM" format="YYYY年M月" :allow-clear="false" :input-read-only="true" :get-popup-container="popupContainer" aria-label="选择月份" @update:value="selectMonth" />
            <a-button type="text" aria-label="下个月" :disabled="selectedMonth === '2100-12'" @click="changeMonth(1)"><RightOutlined /></a-button>
          </div>
        </div>

        <a-alert v-if="storageError" :message="storageError" type="error" show-icon class="ledger-alert" />
        <div class="month-heading"><div><span class="eyebrow">MY LITTLE LEDGER</span><h2>{{ monthLabel }}，收支一目了然</h2></div><span v-if="data.demo" class="demo-badge">演示数据 · 可随时重置</span></div>

        <section class="summary-grid" aria-label="本月收支汇总" aria-live="polite">
          <article class="summary-card income-card">
            <div class="summary-label">本月收入<span class="metric-icon"><ArrowDownOutlined /></span></div>
            <div class="metric income-text"><span class="currency-symbol">¥</span>{{ currency(income) }}</div>
            <p><span class="small-dot green"></span>每一份努力，都值得记录</p>
          </article>
          <article class="summary-card">
            <div class="summary-label">本月支出<span class="metric-icon orange"><ArrowUpOutlined /></span></div>
            <div class="metric"><span class="currency-symbol">¥</span>{{ currency(expense) }}</div>
            <p>共 {{ monthRecords.filter(item => item.type === 'expense').length }} 笔消费 · 好好生活的痕迹</p>
          </article>
          <article class="summary-card">
            <div class="summary-label">本月结余<span class="metric-icon tan"><WalletOutlined /></span></div>
            <div class="metric" :class="{ 'danger-text': balance < 0 }"><span class="currency-symbol">¥</span>{{ currency(balance) }}</div>
            <p>{{ balance < 0 ? '本月支出已超过收入，留意一下哦' : '收入减去支出，给未来留一点余地' }}</p>
          </article>
          <article class="summary-card budget-summary" :class="{ exceeded: totalBudget > 0 && budgetRemaining < 0 }">
            <div class="summary-label">预算剩余<span class="metric-icon orange"><AimOutlined /></span></div>
            <div v-if="totalBudget > 0" class="metric" :class="{ 'danger-text': budgetRemaining < 0 }"><span class="currency-symbol">¥</span>{{ currency(budgetRemaining) }}</div>
            <div v-else class="metric unset-budget">尚未设置</div>
            <div class="mini-progress"><span :style="{ transform: `scaleX(${Math.min(budgetUsed, 100) / 100})` }"></span></div>
            <p>{{ totalBudget > 0 ? `总预算 ¥${currency(totalBudget)} · 已用 ${budgetUsed}%` : '设置分类预算，让消费更从容' }}</p>
          </article>
        </section>

        <div v-if="overCategories.length" class="budget-warning" role="status"><ExclamationCircleOutlined /><span><strong>{{ overCategories.map(item => item.name).join('、') }}已超出预算</strong>，一起留意接下来的花费。</span><a-button type="text" @click="activeView = 'budgets'">查看预算 <RightOutlined /></a-button></div>

        <section v-if="activeView === 'overview'" class="charts-grid">
          <article class="ledger-panel category-panel">
            <div class="panel-heading"><div><h3>钱都花在哪儿了</h3><p>本月消费分类</p></div><span class="panel-mini-label">支出分布</span></div>
            <div v-if="expense > 0" class="donut-layout">
              <div class="donut-wrap">
                <svg viewBox="0 0 200 200" role="img" :aria-label="`本月支出 ${currency(expense)} 元，分类明细见右侧`">
                  <circle cx="100" cy="100" r="78" fill="none" stroke="var(--ledger-chart-track)" stroke-width="24" />
                  <circle v-for="segment in donutSegments" :key="segment.name" cx="100" cy="100" r="78" fill="none" :stroke="segment.color" stroke-width="24" pathLength="100" :stroke-dasharray="segment.dash" :stroke-dashoffset="segment.offset" transform="rotate(-90 100 100)"><title>{{ segment.name }}：¥{{ currency(segment.spent) }}，{{ segment.share }}%</title></circle>
                </svg>
                <div class="donut-center"><span>本月总支出</span><strong>¥{{ currency(expense) }}</strong><small>{{ donutSegments.length }} 个消费分类</small></div>
              </div>
              <ul class="category-legend"><li v-for="segment in donutSegments" :key="segment.name"><span class="legend-dot" :style="{ background: segment.color }"></span><span class="legend-name">{{ segment.name }}</span><span class="legend-share">{{ segment.share }}%</span><strong>¥{{ currency(segment.spent) }}</strong></li></ul>
            </div>
            <div v-else class="chart-empty"><PieChartOutlined /><h4>这个月还没有支出</h4><p>记下一笔消费，这里就会呈现分类分布</p><a-button @click="openRecord()">记下第一笔</a-button></div>
          </article>
          <article class="ledger-panel trend-panel">
            <div class="panel-heading"><div><h3>每天的小小花费</h3><p>每日支出趋势</p></div><span class="trend-key"><span class="small-dot coral"></span>支出</span></div>
            <div class="trend-chart">
              <svg viewBox="0 0 660 220" role="img" :aria-label="`${monthLabel}每日支出趋势，最高为${peakDay.day}日 ${currency(peakDay.amount)} 元`">
                <defs><linearGradient id="ledger-trend-fill" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stop-color="var(--ledger-chart-fill)" stop-opacity="0.24" /><stop offset="100%" stop-color="var(--ledger-chart-fill)" stop-opacity="0.01" /></linearGradient></defs>
                <g v-for="level in [0, 1, 2, 3]" :key="level"><line x1="46" x2="640" :y1="184 - level * 148 / 3" :y2="184 - level * 148 / 3" stroke="var(--ledger-chart-grid)" stroke-dasharray="4 5" /><text x="35" :y="188 - level * 148 / 3" text-anchor="end">{{ Math.round(chartMax / 100 * level / 3).toLocaleString('zh-CN') }}</text></g>
                <path :d="trendArea" fill="url(#ledger-trend-fill)" />
                <path :d="trendPath" fill="none" stroke="var(--ledger-chart-stroke)" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" />
                <circle v-for="point in chartPoints" :key="point.day" :cx="point.x" :cy="point.y" r="4" fill="var(--ledger-surface)" stroke="var(--ledger-chart-stroke)" stroke-width="2"><title>{{ point.day }}日：¥{{ currency(point.amount) }}</title></circle>
                <text v-for="day in [1, 5, 10, 15, 20, 25, dailyExpenses.length]" :key="day" :x="46 + (day - 1) / (dailyExpenses.length - 1) * 594" y="211" text-anchor="middle">{{ day }}日</text>
              </svg>
            </div>
            <div class="trend-footer"><span>日均支出 <strong>¥{{ currency(Math.round(expense / dailyExpenses.length)) }}</strong></span><span v-if="expense > 0">最高支出 <strong>{{ peakDay.day }}日 · ¥{{ currency(peakDay.amount) }}</strong></span><span v-else>暂无支出，享受轻松的一天</span></div>
          </article>
        </section>

        <div class="details-grid" :class="{ 'full-width': activeView !== 'overview' }">
          <section v-if="activeView !== 'budgets'" class="ledger-panel records-panel">
            <div class="panel-heading"><div><h3>{{ activeView === 'overview' ? '最近账单' : '本月全部账单' }}<span class="count-badge">{{ filteredRecords.length }} 笔</span></h3><p>每一笔，都是生活的一部分</p></div><a-button v-if="activeView === 'overview'" type="text" class="text-link" @click="activeView = 'records'">查看全部 <RightOutlined /></a-button><a-button v-else @click="openRecord()"><PlusOutlined /> 新增账单</a-button></div>
            <div class="record-filters">
              <a-input v-model:value="keyword" placeholder="搜索备注、分类" aria-label="搜索账单" allow-clear><template #prefix><SearchOutlined /></template></a-input>
              <a-select v-model:value="filterType" aria-label="筛选收支类型" :get-popup-container="popupContainer" :options="[{ label: '全部类型', value: 'all' }, { label: '支出', value: 'expense' }, { label: '收入', value: 'income' }]" />
              <a-select v-model:value="filterCategory" aria-label="筛选分类" :get-popup-container="popupContainer" :options="filterCategoryOptions" />
            </div>
            <div v-if="displayedRecords.length" class="records-list">
              <div class="record-list-head"><span>账单 / 分类</span><span>日期</span><span>金额</span><span>操作</span></div>
              <article v-for="record in displayedRecords" :key="record.id" class="record-row">
                <div class="record-description"><span class="category-icon" :style="{ color: categoryMeta(record).color, background: `${categoryMeta(record).color}18` }"><component :is="categoryIcons[categoryMeta(record).icon as keyof typeof categoryIcons]" /></span><div class="record-copy"><h4>{{ record.note || record.category }}</h4><span>{{ record.category }}<span class="record-type-dot">·</span>{{ record.type === 'income' ? '收入' : '支出' }}<span class="mobile-date"> · {{ record.date.slice(5).replace('-', '/') }}</span></span></div></div>
                <time :datetime="record.date" class="record-date">{{ record.date.slice(5).replace('-', '/') }}</time>
                <strong class="record-amount" :class="record.type === 'income' ? 'income-text' : ''">{{ record.type === 'income' ? '+' : '−' }}{{ currency(record.amount) }}</strong>
                <div class="record-actions"><a-button type="text" :aria-label="`编辑账单 ${record.note || record.category}`" @click="openRecord(record)"><EditOutlined /></a-button><a-button type="text" danger :aria-label="`删除账单 ${record.note || record.category}`" @click="deleteTarget = record"><DeleteOutlined /></a-button></div>
              </article>
            </div>
            <div v-else class="records-empty"><UnorderedListOutlined /><h4>{{ hasFilters ? '没有找到匹配的账单' : '这个月的账本，等待你的第一笔' }}</h4><p>{{ hasFilters ? '换个关键词或筛选条件试试看' : '从一顿午餐、一杯咖啡开始记录吧' }}</p><a-button v-if="hasFilters" @click="clearFilters">清除筛选</a-button><a-button v-else type="primary" @click="openRecord()"><PlusOutlined /> 记一笔</a-button></div>
            <div v-if="activeView === 'records' && filteredRecords.length > 8" class="record-pagination"><a-pagination v-model:current="page" :total="filteredRecords.length" :page-size="8" :show-size-changer="false" /></div>
          </section>

          <section v-if="activeView !== 'records'" class="ledger-panel budgets-panel" :class="{ 'budgets-expanded': activeView === 'budgets' }">
            <div class="panel-heading"><div><h3>花得刚刚好</h3><p>{{ monthLabel }}分类预算</p></div><a-button type="text" class="text-link" @click="openBudgets"><EditOutlined /> 设置预算</a-button></div>
            <div v-if="activeView === 'budgets'" class="budget-explanation"><AimOutlined /><div><strong>给每一类消费，留一份计划</strong><p>预算按月份单独保存。填 0 表示未设置；进度和超额金额随账单实时更新。</p></div></div>
            <div class="budget-list">
              <article v-for="category in categoryStats" :key="category.name" class="category-budget" :class="{ 'is-over': category.over }">
                <div class="budget-row-heading"><span><span class="legend-dot" :style="{ background: category.color }"></span>{{ category.name }}</span><strong v-if="category.over" class="danger-text">超出 ¥{{ currency(category.spent - category.budget) }}</strong><span v-else-if="category.budget === 0" class="muted">未设预算</span><span v-else>{{ category.percent }}%</span></div>
                <div class="budget-bar" role="progressbar" :aria-label="`${category.name}预算使用`" :aria-valuenow="Math.min(category.percent, 100)" :aria-valuemin="0" :aria-valuemax="100" :aria-valuetext="category.budget ? `已花 ${currency(category.spent)} 元，预算 ${currency(category.budget)} 元${category.over ? '，已超额' : ''}` : '未设置预算'"><span :style="{ background: category.over ? 'var(--ledger-danger)' : category.color, transform: `scaleX(${Math.min(category.percent, 100) / 100})` }"></span></div>
                <div class="budget-row-amount"><span>已花 ¥{{ currency(category.spent) }}</span><span>{{ category.budget ? `预算 ¥${currency(category.budget)}` : '设置后显示进度' }}</span></div>
              </article>
            </div>
            <div class="budget-note"><ExclamationCircleOutlined v-if="overCategories.length" /><CheckCircleOutlined v-else /><span>{{ overCategories.length ? `${overCategories.length} 个分类超额，调整计划也没关系` : totalBudget > 0 ? '量入为出，慢慢靠近心里的小目标' : '先定一个小预算，从容安排每一天' }}</span></div>
            <p v-if="unbudgetedExpense > 0" class="unbudgeted-note">未设预算分类的 ¥{{ currency(unbudgetedExpense) }} 支出也计入预算剩余。</p>
          </section>
        </div>

        <footer class="ledger-footer"><p><SafetyOutlined /> 数据仅保存在当前浏览器，不会连接银行账户，不会上传账单。<span>清除浏览器数据会丢失账本。</span></p><a-button type="text" @click="resetModalOpen = true"><ReloadOutlined /> 重置演示数据</a-button></footer>
      </div>
      <a-button type="primary" class="mobile-add-record" @click="openRecord()"><PlusOutlined /> 记一笔</a-button>

      <a-modal v-model:open="recordModalOpen" :title="editingId ? '编辑这笔账单' : '记下生活的一笔'" :width="480" :footer="null" :get-container="false" :mask-closable="false" class="ledger-modal">
        <p class="modal-subtitle">不用记得很完美，开始记录就很好。</p>
        <a-form layout="vertical" @finish="saveRecord">
          <a-form-item label="收支类型"><a-radio-group v-model:value="form.type" button-style="solid" class="record-type-toggle"><a-radio-button value="expense"><ArrowUpOutlined /> 支出</a-radio-button><a-radio-button value="income"><ArrowDownOutlined /> 收入</a-radio-button></a-radio-group></a-form-item>
          <a-form-item label="金额（元）" required><a-input-number v-model:value="form.amount" :controls="false" :step="0.01" placeholder="0.00" size="large" inputmode="decimal" aria-label="金额（元）" class="amount-input" addon-before="¥" /></a-form-item>
          <div class="form-two-columns"><a-form-item label="分类" required><a-select v-model:value="form.category" :options="categoryOptions" :get-popup-container="popupContainer" aria-label="账单分类" /></a-form-item><a-form-item label="日期" required><a-date-picker v-model:value="form.date" value-format="YYYY-MM-DD" format="YYYY/MM/DD" :allow-clear="false" :get-popup-container="popupContainer" aria-label="账单日期"><template #suffixIcon><CalendarOutlined /></template></a-date-picker></a-form-item></div>
          <a-form-item label="备注（选填）"><a-textarea v-model:value="form.note" :maxlength="200" show-count :rows="3" placeholder="这笔钱，花在了什么小美好上？" aria-label="账单备注" /></a-form-item>
          <a-alert v-if="formError" :message="formError" type="error" show-icon role="alert" />
          <div class="modal-actions"><a-button @click="recordModalOpen = false">取消</a-button><a-button type="primary" html-type="submit">{{ editingId ? '保存修改' : '保存账单' }}</a-button></div>
        </a-form>
      </a-modal>

      <a-modal v-model:open="budgetModalOpen" :title="`${monthLabel} · 预算设置`" :width="480" :footer="null" :get-container="false" :mask-closable="false" class="ledger-modal">
        <p class="modal-subtitle">仅修改这个月的预算，填 0 表示未设置。</p>
        <a-form layout="vertical" @finish="saveBudgets"><div class="budget-form-grid"><a-form-item v-for="category in categoryStats" :key="category.name" :label="category.name"><a-input-number v-model:value="budgetDraft[category.name]" :controls="false" :step="0.01" addon-before="¥" inputmode="decimal" :aria-label="`${category.name}月度预算`" /></a-form-item></div><a-alert v-if="budgetError" :message="budgetError" type="error" show-icon role="alert" /><div class="modal-actions"><a-button @click="budgetModalOpen = false">取消</a-button><a-button type="primary" html-type="submit">保存预算</a-button></div></a-form>
      </a-modal>

      <a-modal :open="!!deleteTarget" title="确定删除这笔账单吗？" ok-text="确认删除" cancel-text="保留账单" :ok-button-props="{ danger: true }" :get-container="false" class="ledger-modal" @ok="deleteRecord" @cancel="deleteTarget = null"><div v-if="deleteTarget" class="delete-preview"><strong>{{ deleteTarget.note || deleteTarget.category }}</strong><span>{{ deleteTarget.date }} · {{ deleteTarget.category }} · {{ deleteTarget.type === 'income' ? '收入' : '支出' }} ¥{{ currency(deleteTarget.amount) }}</span></div><p class="modal-subtitle">删除后无法恢复，汇总、图表和预算会同步更新。</p></a-modal>
      <a-modal v-model:open="resetModalOpen" title="重置为演示账本？" ok-text="确认重置" cancel-text="取消" :ok-button-props="{ danger: true }" :get-container="false" class="ledger-modal" @ok="resetDemo"><p class="reset-warning">这会覆盖所有月份的账单和预算，包括你自己添加的记录，且无法恢复。</p><p class="modal-subtitle">重置后会生成本月和上月的示例收支与预算，方便重新体验。</p></a-modal>
    </main>
  </a-config-provider>
</template>

<style scoped src="./LedgerView.css"></style>
