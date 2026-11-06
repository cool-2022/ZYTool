<script setup lang="ts">
import {
  SearchOutlined, ReloadOutlined, TableOutlined, ProfileOutlined,
  KeyOutlined, LockOutlined, InboxOutlined,
} from '@ant-design/icons-vue'
import GridView from '@/styles/gridView'
import { useDatabaseAdminView } from './DatabaseAdminView'

const {
  tablesLoading, tableKeyword, filteredTables,
  activeTable, activeTableInfo, activeTab,
  columns, columnsLoading, structureColumns,
  rowsData, rowsLoading, keyword, rowTableColumns, tableRows,
  pageSize, pageSizeOptions,
  forbidden,
  selectTable, searchRows, changePage, refresh,
} = useDatabaseAdminView()
</script>

<template>
  <div class="db-panel">
    <div v-if="forbidden" class="forbidden-panel">
      <LockOutlined class="forbidden-icon" />
      <h2>权限不足</h2>
      <p>当前账号没有管理员权限，无法访问数据库表管理。</p>
    </div>

    <template v-else>
      <div class="panel-toolbar">
        <span class="panel-tip">选择左侧数据表，浏览结构与数据</span>
        <a-button :loading="tablesLoading || rowsLoading || columnsLoading" @click="refresh">
          <ReloadOutlined /> 刷新
        </a-button>
      </div>

      <div class="admin-workspace">
        <aside class="table-sidebar">
          <div class="sidebar-search">
            <a-input v-model:value="tableKeyword" placeholder="搜索表名或注释" allow-clear>
              <template #prefix>
                <SearchOutlined />
              </template>
            </a-input>
          </div>
          <div class="table-list" role="listbox" aria-label="数据表列表">
            <a-spin v-if="tablesLoading" class="list-spin" />
            <button v-for="t in filteredTables" :key="t.name" type="button" class="table-item"
              :class="{ selected: t.name === activeTable }" role="option" :aria-selected="t.name === activeTable"
              @click="selectTable(t.name)">
              <span class="table-name">{{ t.name }}</span>
              <span class="table-comment">{{ t.comment || '暂无注释' }}</span>
            </button>
            <div v-if="!tablesLoading && !filteredTables.length" class="list-empty">
              <InboxOutlined /><span>没有匹配的数据表</span>
            </div>
          </div>
        </aside>

        <section class="table-content">
          <div v-if="!activeTable" class="content-placeholder">
            <TableOutlined class="placeholder-icon" />
            <p>从左侧选择一张数据表开始浏览</p>
          </div>

          <template v-else>
            <div class="content-heading">
              <div class="heading-text">
                <h2>{{ activeTable }}<span v-if="activeTableInfo?.comment" class="heading-comment">{{
                    activeTableInfo.comment }}</span></h2>
                <p>约 {{ activeTableInfo?.row_estimate ?? 0 }} 行 · 共 {{ rowsData.total }} 条匹配记录</p>
              </div>
              <nav class="content-tabs" aria-label="表信息视图">
                <a-button :class="{ selected: activeTab === 'rows' }" :aria-pressed="activeTab === 'rows'"
                  @click="activeTab = 'rows'">
                  <TableOutlined /> 数据浏览
                </a-button>
                <a-button :class="{ selected: activeTab === 'columns' }" :aria-pressed="activeTab === 'columns'"
                  @click="activeTab = 'columns'">
                  <ProfileOutlined /> 表结构
                </a-button>
              </nav>
            </div>

            <div v-show="activeTab === 'rows'" class="rows-panel">
              <!-- <div class="rows-tools">
                  <a-input-search
                    v-model:value="keyword"
                    class="rows-search"
                    placeholder="关键词：整行内容模糊匹配"
                    enter-button="搜索"
                    allow-clear
                    :loading="rowsLoading"
                    @search="searchRows"
                  />
                </div> -->
              <GridView :columns="rowTableColumns" :data-source="tableRows" :loading="rowsLoading" row-key="__row_key"
                :total="rowsData.total" :page="rowsData.page" :page-size="pageSize" :page-size-options="pageSizeOptions"
                :sticky-top="202" @change="changePage">
                <template #bodyCell="{ column, record }">
                  <template v-if="record[column.dataIndex] === null || record[column.dataIndex] === undefined">
                    <span class="cell-null">NULL</span>
                  </template>
                  <span v-else-if="typeof record[column.dataIndex] === 'boolean'" class="cell-bool"
                    :class="{ on: record[column.dataIndex] }">{{ record[column.dataIndex] ? 'true' : 'false' }}</span>
                  <span v-else-if="typeof record[column.dataIndex] === 'object'" class="cell-json"
                    :title="JSON.stringify(record[column.dataIndex])">{{ JSON.stringify(record[column.dataIndex])
                    }}</span>
                  <span v-else>{{ record[column.dataIndex] }}</span>
                </template>
                <template #emptyText>
                  <div class="table-empty">
                    <InboxOutlined /><span>{{ keyword ? '没有匹配的记录' : '该表暂无数据' }}</span>
                  </div>
                </template>
              </GridView>
            </div>

            <div v-show="activeTab === 'columns'" class="columns-panel">
              <GridView :columns="structureColumns" :data-source="columns" :loading="columnsLoading" row-key="name"
                :sticky-top="156">
                <template #bodyCell="{ column, record }">
                  <template v-if="column.dataIndex === 'name'">
                    <span class="col-name">{{ record.name }}</span>
                    <span v-if="record.is_primary" class="pk-badge">
                      <KeyOutlined /> 主键
                    </span>
                  </template>
                  <template v-else-if="column.dataIndex === 'data_type'">
                    <code class="col-type">{{ record.data_type }}</code>
                  </template>
                  <template v-else-if="column.dataIndex === 'nullable'">
                    <span :class="record.nullable ? 'cell-null' : 'col-not-null'">{{ record.nullable ? '是' : '否'
                      }}</span>
                  </template>
                  <template v-else>
                    <span v-if="record[column.dataIndex]">{{ record[column.dataIndex] }}</span>
                    <span v-else class="cell-null">—</span>
                  </template>
                </template>
              </GridView>
            </div>
          </template>
        </section>
      </div>
    </template>
  </div>
</template>

<style scoped src="./DatabaseAdminView.css"></style>
