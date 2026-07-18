<script setup lang="ts">
import { inject } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { LogoutOutlined, MoonOutlined, SunOutlined } from '@ant-design/icons-vue'
import { getUserInfo, clearAuth } from '@/utils/auth'

const router = useRouter()
const theme = inject<{ value: string }>('theme')
const toggleTheme = inject<() => void>('toggleTheme')

const userInfo = getUserInfo()

const handleLogout = () => {
  clearAuth()
  message.success('已退出登录')
  router.push('/login')
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-card">
      <h1 class="settings-title">个人设置</h1>

      <div class="settings-section">
        <h2 class="section-title">账号信息</h2>
        <div class="info-item">
          <span class="info-label">用户名</span>
          <span class="info-value">{{ userInfo?.username || '-' }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">角色</span>
          <span class="info-value">{{ (userInfo?.roles || []).join(', ') || '-' }}</span>
        </div>
      </div>

      <div class="settings-section">
        <h2 class="section-title">外观</h2>
        <div class="info-item">
          <span class="info-label">主题模式</span>
          <a-button type="primary" class="theme-btn" @click="toggleTheme">
            <template #icon>
              <SunOutlined v-if="theme?.value === 'dark'" />
              <MoonOutlined v-else />
            </template>
            {{ theme?.value === 'light' ? '切换深色模式' : '切换浅色模式' }}
          </a-button>
        </div>
      </div>

      <div class="settings-actions">
        <a-button danger block size="large" @click="handleLogout">
          <LogoutOutlined />
          <span>退出登录</span>
        </a-button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  min-height: calc(100vh - 70px);
  padding: 48px 24px;
  display: flex;
  justify-content: center;
  align-items: flex-start;
}

.settings-card {
  width: 100%;
  max-width: 520px;
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--border-radius-xl);
  padding: 2rem;
  box-shadow: var(--shadow-xl);
}

.settings-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 1.5rem;
}

.settings-section {
  margin-bottom: 1.5rem;
}

.section-title {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 1rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid var(--border-color);
}

.info-item:last-child {
  border-bottom: none;
}

.info-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.info-value {
  color: var(--text-primary);
  font-weight: 600;
}

.theme-btn {
  background: var(--gradient-primary);
  border: none;
}

.settings-actions {
  margin-top: 2rem;
}

@media (max-width: 480px) {
  .settings-card {
    padding: 1.5rem;
  }

  .info-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.25rem;
  }
}
</style>
