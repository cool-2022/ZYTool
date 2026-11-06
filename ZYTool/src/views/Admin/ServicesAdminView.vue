<script setup lang="ts">
import {
  ReloadOutlined, LockOutlined, CloudServerOutlined, ApiOutlined,
} from '@ant-design/icons-vue'
import { useServicesAdminView } from './ServicesAdminView'

const {
  loading, forbidden, rust, rustError, python, rustUrl, pythonUrl,
  formatUptime, formatTime, refresh,
} = useServicesAdminView()
</script>

<template>
  <div class="services-panel">
    <div v-if="forbidden" class="forbidden-panel">
      <LockOutlined class="forbidden-icon" />
      <h2>权限不足</h2>
      <p>当前账号没有管理员权限，无法访问服务管理。</p>
    </div>

    <template v-else>
      <div class="panel-toolbar">
        <span class="panel-tip">双后端运行状态：Rust 主后端 / Python 兜底后端</span>
        <a-button :loading="loading" @click="refresh"><ReloadOutlined /> 刷新</a-button>
      </div>

      <div class="service-cards">
        <section class="service-card">
          <div class="service-head">
            <div class="service-title">
              <span class="service-icon"><CloudServerOutlined /></span>
              <div><h2>Rust 主后端</h2><p>{{ rustUrl }}</p></div>
            </div>
            <span class="status-badge" :class="rust ? 'ok' : 'down'">{{ rust ? '运行中' : '不可用' }}</span>
          </div>
          <div v-if="rust" class="service-meta">
            <div class="meta-row"><span class="k">服务名称</span><span class="v">{{ rust.name }}</span></div>
            <div class="meta-row"><span class="k">版本</span><span class="v mono">{{ rust.version }}</span></div>
            <div class="meta-row"><span class="k">描述</span><span class="v">{{ rust.description || '—' }}</span></div>
            <div class="meta-row"><span class="k">数据库</span><span class="v">{{ rust.db_connected ? '已连接' : '未连接' }}</span></div>
            <div class="meta-row"><span class="k">运行时长</span><span class="v">{{ formatUptime(rust.uptime_seconds) }}</span></div>
            <div class="meta-row"><span class="k">上报时间</span><span class="v">{{ formatTime(rust.timestamp) }}</span></div>
          </div>
          <div v-else class="service-error">{{ rustError || '正在检测…' }}</div>
        </section>

        <section class="service-card">
          <div class="service-head">
            <div class="service-title">
              <span class="service-icon"><ApiOutlined /></span>
              <div><h2>Python 兜底后端</h2><p>{{ pythonUrl }}</p></div>
            </div>
            <span class="status-badge" :class="python?.ok ? 'ok' : 'down'">{{ python?.ok ? '运行中' : '不可用' }}</span>
          </div>
          <div v-if="python?.ok" class="service-meta">
            <div class="meta-row"><span class="k">状态</span><span class="v">{{ python.status || 'healthy' }}</span></div>
            <div class="meta-row"><span class="k">版本</span><span class="v mono">{{ python.version || '—' }}</span></div>
            <div class="meta-row"><span class="k">上报时间</span><span class="v">{{ formatTime(python.timestamp) }}</span></div>
          </div>
          <div v-else class="service-error">{{ python?.error || '正在检测…' }}</div>
        </section>
      </div>
    </template>
  </div>
</template>

<style scoped src="./ServicesAdminView.css"></style>
