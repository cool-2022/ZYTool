<script setup lang="ts">
import { computed, inject, ref } from 'vue'
import type { Ref } from 'vue'
import { theme as antdTheme } from 'ant-design-vue'
import {
  ControlOutlined, DatabaseOutlined, FontSizeOutlined, CloudServerOutlined,
} from '@ant-design/icons-vue'
import DatabaseAdminView from './DatabaseAdminView.vue'
import RootsAdminView from './RootsAdminView.vue'
import ServicesAdminView from './ServicesAdminView.vue'
import { useAdminView } from './AdminView'
import type { AdminTab } from './AdminView'

const { activeTab, selectTab, isVisited } = useAdminView()

const tabs: Array<{ key: AdminTab; label: string; icon: any }> = [
  { key: 'database', label: '数据库管理', icon: DatabaseOutlined },
  { key: 'roots', label: '字根管理', icon: FontSizeOutlined },
  { key: 'services', label: '服务管理', icon: CloudServerOutlined },
]

const pageTheme = inject<Readonly<Ref<'light' | 'dark'>>>('theme', ref<'light' | 'dark'>('light'))
const adminTheme = computed(() => {
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
</script>

<template>
  <a-config-provider :theme="adminTheme">
    <main class="admin-page">
      <div class="admin-shell">
        <header class="admin-header">
          <div class="admin-brand">
            <span class="admin-mark"><ControlOutlined /></span>
            <div><h1>后台管理<span class="brand-dot">.</span></h1><p>数据库 / 字根 / 服务 统一管理</p></div>
          </div>
        </header>

        <nav class="admin-tabs" aria-label="管理分类">
          <button
            v-for="t in tabs"
            :key="t.key"
            type="button"
            class="admin-tab"
            :class="{ selected: activeTab === t.key }"
            :aria-pressed="activeTab === t.key"
            @click="selectTab(t.key)"
          >
            <component :is="t.icon" /><span>{{ t.label }}</span>
          </button>
        </nav>

        <DatabaseAdminView v-if="isVisited('database')" v-show="activeTab === 'database'" />
        <RootsAdminView v-if="isVisited('roots')" v-show="activeTab === 'roots'" />
        <ServicesAdminView v-if="isVisited('services')" v-show="activeTab === 'services'" />
      </div>
    </main>
  </a-config-provider>
</template>

<style scoped src="./AdminView.css"></style>
