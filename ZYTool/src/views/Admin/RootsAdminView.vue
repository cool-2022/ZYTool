<script setup lang="ts">
import {
  ReloadOutlined, PlusOutlined, LockOutlined, InboxOutlined, ScanOutlined, SyncOutlined,
} from '@ant-design/icons-vue'
import GridView from '@/styles/gridView'
import { useRootsAdminView } from './RootsAdminView'

const {
  rows, loading, scanning, keyword, page, pageSize, total, columns, forbidden,
  pageSizeOptions,
  modalOpen, saving, editingId, form,
  typeOptions, isCharType, isNumericType, isTimeType, formTypeExpr, changeType, changePrecision,
  syncOpen, syncing, syncTarget, syncComment, syncType, syncResults, syncTargets,
  rootStatus, formatTime, search, changePage, scan, openCreate, openRegister, openEdit, submit, remove, openSync, doSync,
  refresh,
} = useRootsAdminView()
</script>

<template>
  <div class="roots-panel">
    <div v-if="forbidden" class="forbidden-panel">
      <LockOutlined class="forbidden-icon" />
      <h2>权限不足</h2>
      <p>当前账号没有管理员权限，无法访问字根管理。</p>
    </div>

    <template v-else>
      <div class="panel-toolbar">
        <a-input-search
          v-model:value="keyword"
          class="panel-toolbar-search"
          placeholder="搜索字段名 / 注释 / 备注"
          enter-button="搜索"
          allow-clear
          :loading="loading"
          @search="search"
        />
        <div class="row-actions">
          <a-button :loading="scanning" @click="scan"><ScanOutlined /> 扫描入库</a-button>
          <a-button :loading="loading" @click="refresh"><ReloadOutlined /> 刷新</a-button>
          <a-button type="primary" @click="openCreate"><PlusOutlined /> 新增字根</a-button>
        </div>
      </div>

      <div class="roots-card">
        <GridView
          :columns="columns"
          :data-source="rows"
          :loading="loading"
          row-key="field_name"
          :total="total"
          :page="page"
          :page-size="pageSize"
          :page-size-options="pageSizeOptions"
          :sticky-top="202"
          @change="changePage"
        >
          <template #bodyCell="{ column, record }">
            <template v-if="column.dataIndex === 'field_name'">
              <span class="root-name">{{ record.field_name }}</span>
            </template>
            <template v-else-if="column.dataIndex === 'type_expr'">
              <code v-if="record.type_expr" class="root-code">{{ record.type_expr }}</code>
              <span v-else class="cell-empty">—</span>
            </template>
            <template v-else-if="column.dataIndex === 'refs'">
              <template v-if="record.refs.length">
                <span class="ref-count">{{ record.refs.length }} 张表</span>
                <span
                  v-for="r in record.refs.slice(0, 3)"
                  :key="r.table_name"
                  class="ref-tag"
                  :class="{ diff: !r.synced }"
                  :title="r.diffs.join('；') || '一致'"
                >{{ r.table_name }}</span>
                <span v-if="record.refs.length > 3" class="cell-empty">等</span>
              </template>
              <span v-else class="cell-empty">无引用</span>
            </template>
            <template v-else-if="column.key === 'status'">
              <a-tooltip v-if="rootStatus(record).cls === 'diff'" placement="topLeft">
                <template #title>
                  <div v-for="r in record.refs.filter((x: any) => !x.synced)" :key="r.table_name" class="status-diff-tip">
                    {{ r.table_name }}：{{ r.diffs.join('；') }}
                  </div>
                </template>
                <span class="sync-badge diff">有差异</span>
              </a-tooltip>
              <span v-else class="sync-badge" :class="rootStatus(record).cls">{{ rootStatus(record).text }}</span>
            </template>
            <template v-else-if="column.dataIndex === 'updated_at'">
              <span>{{ formatTime(record.updated_at) }}</span>
            </template>
            <template v-else-if="column.key === 'action'">
              <div class="row-actions">
                <template v-if="record.registered">
                  <a-button size="small" :disabled="!record.refs.length" @click="openSync(record)"><SyncOutlined /> 同步</a-button>
                  <a-button size="small" @click="openEdit(record)">编辑</a-button>
                  <a-popconfirm
                    title="仅删除字根记录，引用表字段不受影响"
                    ok-text="删除"
                    cancel-text="取消"
                    @confirm="remove(record)"
                  >
                    <a-button size="small" danger>删除</a-button>
                  </a-popconfirm>
                </template>
                <a-button v-else size="small" type="primary" @click="openRegister(record)"><PlusOutlined /> 入库</a-button>
              </div>
            </template>
            <template v-else>
              <span v-if="record[column.dataIndex]">{{ record[column.dataIndex] }}</span>
              <span v-else class="cell-empty">—</span>
            </template>
          </template>
          <template #expandedRowRender="{ record }">
            <div class="refs-detail">
              <div class="refs-title">
                引用明细（按字段名「{{ record.field_name }}」自动关联；{{ record.registered ? '差异 = 引用列 ≠ 字根标准' : `未入库，差异 = 引用列 ≠ 首个引用表 ${record.refs[0]?.table_name || ''}` }}）
              </div>
              <div v-for="r in record.refs" :key="r.table_name" class="refs-item">
                <div class="refs-row">
                  <span class="refs-table">{{ r.table_name }}</span>
                  <code class="root-code">{{ r.type_expr }}</code>
                  <span class="refs-comment">{{ r.comment || '—' }}</span>
                  <span class="sync-badge" :class="r.synced ? 'ok' : 'diff'">{{ r.synced ? '一致' : '有差异' }}</span>
                </div>
                <div v-if="r.diffs.length" class="refs-diffs">
                  <span v-for="diff in r.diffs" :key="diff" class="ref-diff">{{ diff }}</span>
                </div>
              </div>
              <div v-if="!record.refs.length" class="refs-empty">当前没有任何表使用该字段名</div>
            </div>
          </template>
          <template #emptyText>
            <div class="roots-empty"><InboxOutlined /><span>{{ keyword ? '没有匹配的字段' : '数据库 public 模式下没有字段（id/created_at/updated_at 已排除）' }}</span></div>
          </template>
        </GridView>
      </div>

      <a-modal
        v-model:open="modalOpen"
        :title="editingId === null ? '字根入库' : '编辑字根'"
        ok-text="保存"
        cancel-text="取消"
        :confirm-loading="saving"
        :width="520"
        wrap-class-name="root-form-modal"
        @ok="submit"
      >
        <div class="root-form">
          <div class="form-item">
            <label><span class="required">*</span>字段名（小写字母/下划线/数字）</label>
            <a-input v-model:value="form.field_name" placeholder="如：user_id" :maxlength="63" />
          </div>
          <div class="form-item">
            <label for="root-type">基础类型（PostgreSQL 短名）</label>
            <a-select
              id="root-type"
              v-model:value="form.data_type"
              :options="typeOptions"
              show-search
              @change="changeType"
            />
          </div>
          <div v-if="isCharType" class="form-item">
            <label for="root-length">字符长度</label>
            <a-input-number id="root-length" v-model:value="form.data_length" :min="1" :max="10485760" :precision="0" placeholder="未指定" />
          </div>
          <div v-if="isNumericType || isTimeType" class="form-item">
            <label for="root-precision">{{ isNumericType ? '总位数（精度）' : '秒的小数位数（精度）' }}</label>
            <a-input-number
              id="root-precision"
              v-model:value="form.data_precision"
              :min="isNumericType ? 1 : 0"
              :max="isNumericType ? 1000 : 6"
              :precision="0"
              placeholder="未指定"
              @change="changePrecision"
            />
          </div>
          <div v-if="isNumericType" class="form-item">
            <label for="root-scale">小数位数</label>
            <a-input-number
              id="root-scale"
              v-model:value="form.data_scale"
              :min="0"
              :max="form.data_precision ?? 1000"
              :precision="0"
              :disabled="form.data_precision === null"
              placeholder="填写精度后默认 0"
            />
          </div>
          <div class="form-item full type-preview">
            <span>完整类型</span>
            <code class="root-code">{{ formTypeExpr || '未指定' }}</code>
            <span class="type-hint">长度/精度留空表示不显式限制，不代表 0。</span>
          </div>
          <div class="form-item full">
            <label>注释</label>
            <a-input v-model:value="form.comment" placeholder="字段中文说明，同步时写入引用表字段注释" />
          </div>
          <div class="form-item full">
            <label>备注</label>
            <a-textarea v-model:value="form.remark" placeholder="补充说明（可选，不同步）" :rows="2" />
          </div>
        </div>
      </a-modal>

      <a-modal
        v-model:open="syncOpen"
        :title="`同步字根「${syncTarget?.field_name || ''}」`"
        :ok-text="syncResults ? '完成' : '开始同步'"
        cancel-text="取消"
        :confirm-loading="syncing"
        :ok-button-props="{ disabled: !syncResults && !syncTargets.length }"
        :width="560"
        wrap-class-name="root-form-modal"
        @ok="syncResults ? (syncOpen = false) : doSync()"
      >
        <template v-if="syncTarget">
          <template v-if="!syncResults">
            <div class="sync-summary">
              将按下方勾选内容，把字根的注释/类型同步到 <b>{{ syncTargets.length }}</b> 张引用表（共引用 {{ syncTarget.refs.length }} 张）：
            </div>
            <div class="sync-options">
              <a-checkbox v-model:checked="syncComment">
                字段注释 → <code class="root-code">{{ syncTarget.comment || '（空）' }}</code>
              </a-checkbox>
              <a-checkbox v-model:checked="syncType" :disabled="!syncTarget.data_type">
                类型 / 长度 / 精度 / 小数位 → <code class="root-code">{{ syncTarget.type_expr || '（未指定）' }}</code>
                <span class="sync-warn">执行 ALTER COLUMN TYPE，可能失败或有数据转换风险；父表类型变更会递归影响继承/分区子表。</span>
              </a-checkbox>
            </div>
            <div class="refs-detail">
              <div class="refs-title">差异值：引用列 ≠ 字根标准</div>
              <div v-for="r in syncTarget.refs" :key="r.table_name" class="refs-item">
                <div class="refs-row">
                  <span class="refs-table">{{ r.table_name }}</span>
                  <code class="root-code">{{ r.type_expr }}</code>
                  <span class="refs-comment">{{ r.comment || '—' }}</span>
                  <span class="sync-badge" :class="r.synced ? 'ok' : 'diff'">{{ r.synced ? '一致' : '有差异' }}</span>
                </div>
                <div v-if="r.diffs.length" class="refs-diffs">
                  <span v-for="diff in r.diffs" :key="diff" class="ref-diff">{{ diff }}</span>
                </div>
              </div>
            </div>
          </template>
          <div v-else class="refs-detail">
            <div class="refs-title">同步结果</div>
            <div v-for="r in syncResults" :key="r.table_name" class="refs-row">
              <span class="refs-table">{{ r.table_name }}</span>
              <span class="refs-comment">{{ r.message }}</span>
              <span class="sync-badge" :class="r.ok ? 'ok' : 'diff'">{{ r.ok ? '成功' : '失败' }}</span>
            </div>
          </div>
        </template>
      </a-modal>
    </template>
  </div>
</template>

<style scoped src="./RootsAdminView.css"></style>

<!-- a-modal 传送到 body，弹窗内样式需非 scoped -->
<style>
.root-form-modal .root-form { display: grid; grid-template-columns: 1fr 1fr; gap: 0 14px; margin-top: 8px; }
.root-form-modal .form-item { display: flex; flex-direction: column; gap: 5px; margin-bottom: 12px; }
.root-form-modal .form-item.full { grid-column: 1 / -1; }
.root-form-modal .form-item .ant-input-number { width: 100%; }
.root-form-modal .type-preview { align-items: flex-start; font-size: 12px; }
.root-form-modal .type-hint { color: var(--workspace-caption); }
.root-form-modal .form-item label { font-size: 12px; color: var(--admin-muted, #81786e); }
.root-form-modal .form-item label .required { color: var(--admin-coral, #cf654a); margin-right: 3px; }
.root-form-modal .sync-summary { font-size: 13px; margin: 4px 0 12px; }
.root-form-modal .sync-options { display: flex; flex-direction: column; gap: 10px; margin-bottom: 12px; }
.root-form-modal .sync-warn { display: block; margin: 2px 0 0 24px; font-size: 11px; color: var(--admin-sand, #b78239); }
.root-form-modal .refs-detail { border: 1px solid var(--admin-line, #e8e2d8); border-radius: 9px; padding: 10px 12px; display: flex; flex-direction: column; gap: 6px; max-height: 260px; overflow-y: auto; }
.root-form-modal .refs-title { font-size: 12px; color: var(--admin-muted, #81786e); }
.root-form-modal .refs-item { display: flex; flex-direction: column; gap: 4px; }
.root-form-modal .refs-row { display: flex; align-items: center; flex-wrap: wrap; gap: 10px; font-size: 12px; }
.root-form-modal .refs-diffs { display: flex; flex-wrap: wrap; gap: 4px; }
.root-form-modal .ref-diff { padding: 1px 7px; border-radius: 5px; font-size: 11px; color: var(--workspace-sand); background: var(--workspace-sand-soft); }
.root-form-modal .refs-table { font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; font-weight: 600; }
.root-form-modal .refs-comment { flex: 1; color: var(--admin-muted, #81786e); overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.root-form-modal .root-code { padding: 1px 7px; border-radius: 5px; font-size: 12px; background: var(--admin-code-bg, #f6f1e8); color: var(--admin-muted, #81786e); font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; }
.root-form-modal .sync-badge { padding: 1px 8px; border-radius: 999px; font-size: 11px; background: rgba(0,0,0,0.05); color: #81786e; white-space: nowrap; }
.root-form-modal .sync-badge.ok { background: rgba(93,141,116,0.14); color: #5d8d74; }
.root-form-modal .sync-badge.diff { background: rgba(183,130,57,0.14); color: #b78239; }
@media (max-width: 768px) {
  .root-form-modal .root-form { grid-template-columns: 1fr; }
}
</style>
