<script setup lang="ts">
import { computed, reactive } from 'vue'

/**
 * GridView 复用表格（插槽式插件）
 * - 分页条固定在表头上方（total 传 null 隐藏分页）
 * - 表头吸附滚动（stickyTop 为上方固定区总高度）
 * - 列宽拖拽、列级筛选（列配置 filterable: true，客户端筛当前页）
 * - bodyCell / expandedRowRender / emptyText 插槽透传
 */
interface Props {
    columns: any[]
    dataSource: any[]
    loading?: boolean
    rowKey: string
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
  <div class="grid-view">
    <div v-if="total !== null && total !== undefined" class="gv-pager" :style="{ top: pagerTopValue + 'px' }">
      <span class="gv-pager-total">共 {{ total }} 条</span>
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
        <span class="gv-th-text">{{ column.title }}</span>
        <span
          class="gv-col-resize"
          @pointerdown="startResize($event, String(column.key ?? column.dataIndex))"
          @click.stop
        />
      </template>
      <template #customFilterDropdown="{ column }">
        <div class="gv-col-filter">
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

<style scoped src="./gridView.css"></style>
