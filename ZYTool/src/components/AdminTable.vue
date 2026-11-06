<script setup lang="ts">
import { computed, reactive } from 'vue'

interface Props {
    columns: any[]
    dataSource: any[]
    loading?: boolean
    rowKey: string
    /** null/undefined 不显示分页条（如表结构页） */
    total?: number | null
    page?: number
    pageSize?: number
    pageSizeOptions?: string[]
    /** 表头吸附时的视口顶部偏移（上方固定区总高度） */
    stickyTop?: number
    /** 分页条吸附位置，默认 stickyTop - 46 */
    pagerTop?: number
}

const props = withDefaults(defineProps<Props>(), {
    loading: false,
    total: null,
    page: 1,
    pageSize: 20,
    pageSizeOptions: () => ['10', '20', '50', '100'],
    stickyTop: 0,
})

const emit = defineEmits<{
    (e: 'change', page: number, pageSize: number): void
}>()

// ===================== 列宽拖拽 =====================

const widths = reactive<Record<string, number>>({})

function startResize(e: PointerEvent, key: string) {
    e.preventDefault()
    e.stopPropagation()
    const th = (e.target as HTMLElement).closest('th')
    const startX = e.clientX
    const startW = th?.offsetWidth ?? 100
    const move = (ev: PointerEvent) => {
        widths[key] = Math.max(60, startW + ev.clientX - startX)
    }
    const up = () => {
        window.removeEventListener('pointermove', move)
        window.removeEventListener('pointerup', up)
    }
    window.addEventListener('pointermove', move)
    window.addEventListener('pointerup', up)
}

// ===================== 表头筛选（客户端，仅当前页数据） =====================

const filters = reactive<Record<string, string>>({})
const filterOpen = reactive<Record<string, boolean>>({})

const computedColumns = computed(() =>
    props.columns.map(c => {
        const key = String(c.key ?? c.dataIndex)
        const col: any = { ...c, width: widths[key] ?? c.width }
        if (c.filterable) {
            col.customFilterDropdown = true
            col.filterDropdownOpen = !!filterOpen[key]
            col.onFilterDropdownOpenChange = (open: boolean) => {
                filterOpen[key] = open
            }
            col.filtered = !!(filters[String(c.dataIndex)] || '').trim()
        }
        return col
    })
)

const filteredData = computed(() => {
    const active = Object.entries(filters).filter(([, v]) => v.trim())
    if (!active.length) return props.dataSource
    return props.dataSource.filter(row =>
        active.every(([key, kw]) =>
            String(row[key] ?? '').toLowerCase().includes(kw.trim().toLowerCase())
        )
    )
})

const pagerTopValue = computed(() => props.pagerTop ?? Math.max(0, props.stickyTop - 46))
</script>

<template>
  <div class="admin-table">
    <div v-if="total !== null && total !== undefined" class="admin-table-pager" :style="{ top: pagerTopValue + 'px' }">
      <span class="pager-total">共 {{ total }} 条</span>
      <a-pagination
        :current="page"
        :page-size="pageSize"
        :total="total"
        show-size-changer
        :page-size-options="pageSizeOptions"
        size="small"
        @change="(p: number, s: number) => emit('change', p, s)"
      />
    </div>

    <a-table
      :columns="computedColumns"
      :data-source="filteredData"
      :loading="loading"
      :pagination="false"
      :row-key="rowKey"
      size="middle"
      table-layout="fixed"
      :scroll="{ x: 'max-content' }"
      :sticky="{ offsetHeader: stickyTop }"
    >
      <template #headerCell="{ column }">
        <span class="th-text">{{ column.title }}</span>
        <span
          class="col-resize-handle"
          @pointerdown="startResize($event, String(column.key ?? column.dataIndex))"
          @click.stop
        />
      </template>
      <template #customFilterDropdown="{ column }">
        <div class="col-filter">
          <a-input
            v-model:value="filters[String(column.dataIndex)]"
            :placeholder="`筛选 ${column.title}`"
            allow-clear
            size="small"
          />
        </div>
      </template>
      <template #bodyCell="p"><slot name="bodyCell" v-bind="p" /></template>
      <template v-if="$slots.expandedRowRender" #expandedRowRender="p"><slot name="expandedRowRender" v-bind="p" /></template>
      <template v-if="$slots.emptyText" #emptyText><slot name="emptyText" /></template>
    </a-table>
  </div>
</template>

<style scoped>
/* 分页条固定在表头上方 */
.admin-table-pager {
  position: sticky;
  z-index: 28;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  height: 46px;
  padding: 7px 2px;
  box-sizing: border-box;
  background: var(--admin-surface);
  border-bottom: 1px solid var(--admin-line);
}
.pager-total { color: var(--admin-caption); font-size: 12px; white-space: nowrap; }

/* 表头吸附层级低于工具条 */
.admin-table :deep(.ant-table-sticky-holder) { z-index: 27; }

/* 表头拖拽手柄 */
.admin-table :deep(.ant-table-thead > tr > th) { position: relative; }
.th-text { display: inline-block; max-width: calc(100% - 10px); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.col-resize-handle {
  position: absolute;
  right: -5px;
  top: 0;
  bottom: 0;
  width: 10px;
  cursor: col-resize;
  touch-action: none;
}
.col-resize-handle::after {
  content: '';
  position: absolute;
  right: 4px;
  top: 25%;
  bottom: 25%;
  width: 2px;
  border-radius: 1px;
  background: transparent;
  transition: background-color var(--duration-fast) var(--ease-out);
}
@media (hover: hover) and (pointer: fine) {
  .col-resize-handle:hover::after { background: var(--admin-accent-line); }
}

/* 筛选下拉 */
.col-filter { padding: 8px; width: 200px; }

@media (max-width: 768px) {
  .admin-table-pager { position: static; }
}
</style>
