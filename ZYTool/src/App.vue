<script setup lang="ts">
import { ref, onMounted, provide, readonly } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { SettingOutlined, LogoutOutlined } from '@ant-design/icons-vue'
import { isAuthenticated, getUserInfo, clearAuth } from './utils/auth'

const router = useRouter()

// 主题状态
type Theme = 'light' | 'dark'
const theme = ref<Theme>('light')

// 初始化主题
const initTheme = () => {
  const saved = localStorage.getItem('zytool-theme') as Theme | null
  if (saved === 'light' || saved === 'dark') {
    theme.value = saved
  } else if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    theme.value = 'dark'
  }
  applyTheme()
}

// 应用主题到 html 元素
const applyTheme = () => {
  document.documentElement.setAttribute('data-theme', theme.value)
}

// 切换主题
const toggleTheme = () => {
  theme.value = theme.value === 'light' ? 'dark' : 'light'
  applyTheme()
  localStorage.setItem('zytool-theme', theme.value)
}

// 提供给子组件使用
provide('theme', readonly(theme))
provide('toggleTheme', toggleTheme)

// 登录状态
const isLoggedIn = ref(false)
const userInfo = ref<any>(null)

const refreshAuth = () => {
  isLoggedIn.value = isAuthenticated()
  userInfo.value = getUserInfo()
}

const handleLogout = () => {
  clearAuth()
  refreshAuth()
  message.success('已退出登录')
  router.push('/login')
}

const goSettings = () => {
  router.push('/settings')
}

onMounted(() => {
  initTheme()
  refreshAuth()
})

// 路由切换后刷新登录态（例如登录成功后跳转到首页）
router.afterEach(refreshAuth)
</script>

<template>
  <div id="app">
    <!-- 导航栏 -->
    <nav class="navbar">
      <div class="nav-container">
        <router-link to="/home" class="nav-logo">
          <span class="logo-icon">⚡</span>
          <span class="logo-text">ZYTool</span>
        </router-link>
        <div class="nav-menu">
          <router-link to="/home" class="nav-link">首页</router-link>
          <router-link to="/tools" class="nav-link">工具</router-link>

          <!-- 未登录：显示登录按钮 -->
          <router-link
            v-if="!isLoggedIn"
            to="/"
            class="nav-link nav-link-primary"
          >
            登录
          </router-link>

          <!-- 已登录：显示头像下拉菜单 -->
          <a-dropdown
            v-else
            placement="bottomRight"
            :trigger="['hover', 'click']"
          >
            <a-button type="text" class="user-avatar-btn">
              <a-avatar class="user-avatar" size="small">
                {{ (userInfo?.username || 'U').charAt(0).toUpperCase() }}
              </a-avatar>
              <span class="user-name">{{ userInfo?.username || '用户' }}</span>
            </a-button>
            <template #overlay>
              <a-menu>
                <a-menu-item key="settings" @click="goSettings">
                  <SettingOutlined />
                  <span>个人设置</span>
                </a-menu-item>
                <a-menu-divider />
                <a-menu-item key="logout" @click="handleLogout">
                  <LogoutOutlined />
                  <span>退出登录</span>
                </a-menu-item>
              </a-menu>
            </template>
          </a-dropdown>

          <a-button
            type="text"
            class="theme-toggle"
            @click="toggleTheme"
            :title="theme === 'light' ? '切换到深色模式' : '切换到浅色模式'"
          >
            <span class="theme-icon" v-if="theme === 'light'">☀️</span>
            <span class="theme-icon" v-else>🌙</span>
          </a-button>
        </div>
      </div>
    </nav>

    <!-- 路由视图 -->
    <router-view />
  </div>
</template>

<style scoped>
#app {
  min-height: 100vh;
  background: var(--gradient-bg);
}

.navbar {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--glass-border);
  padding: 0.5rem 0;
  position: sticky;
  top: 0;
  z-index: 1000;
  transition: all var(--transition-speed);
}

.nav-container {
  max-width: 1400px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
}

.nav-logo {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  text-decoration: none;
  transition: transform 0.3s ease;
}

.nav-logo:hover {
  transform: scale(1.02);
}

.logo-icon {
  font-size: 1.5rem;
  filter: drop-shadow(0 2px 4px rgba(59, 130, 246, 0.3));
}

.logo-text {
  font-size: 1.4rem;
  font-weight: 800;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -0.5px;
}

.nav-menu {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.nav-link {
  color: var(--text-secondary);
  text-decoration: none;
  padding: 0.4rem 0.9rem;
  border-radius: var(--border-radius);
  transition: all var(--transition-speed);
  font-weight: 500;
  font-size: 0.9rem;
}

.nav-link:hover {
  color: var(--primary-color);
  background: rgba(59, 130, 246, 0.08);
}

.nav-link.router-link-active {
  color: var(--primary-color);
  background: rgba(59, 130, 246, 0.12);
  font-weight: 600;
}

.nav-link-primary {
  background: var(--gradient-primary);
  color: white !important;
  box-shadow: var(--shadow-primary);
  margin-left: 0.5rem;
}

.nav-link-primary:hover {
  background: var(--gradient-primary-hover);
  box-shadow: var(--shadow-primary-hover);
  transform: translateY(-1px);
}

.user-avatar-btn {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  height: 32px;
  padding: 0 0.5rem;
  margin-left: 0.5rem;
  border-radius: var(--border-radius);
  color: var(--text-secondary) !important;
  transition: all var(--transition-speed);
}

.user-avatar-btn:hover {
  color: var(--primary-color) !important;
  background: rgba(59, 130, 246, 0.08);
}

.user-avatar {
  background: var(--gradient-primary);
  color: white;
  font-size: 0.8rem;
  font-weight: 600;
}

.user-name {
  max-width: 80px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 0.85rem;
  font-weight: 500;
}

.theme-toggle {
  color: var(--text-secondary) !important;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--border-radius);
  margin-left: 0.5rem;
  transition: all var(--transition-speed);
}

.theme-toggle:hover {
  color: var(--primary-color) !important;
  background: rgba(59, 130, 246, 0.08);
  transform: rotate(15deg);
}

.theme-icon {
  font-size: 18px;
  line-height: 1;
}

@media (max-width: 768px) {
  .nav-container {
    padding: 0 16px;
  }

  .logo-text {
    font-size: 1.4rem;
  }

  .nav-link {
    padding: 0.45rem 0.8rem;
    font-size: 0.9rem;
  }

  .nav-link-primary {
    margin-left: 0.25rem;
  }

  .user-avatar-btn {
    margin-left: 0.25rem;
    padding: 0 0.5rem;
  }

  .user-name {
    max-width: 60px;
  }

  .theme-toggle {
    margin-left: 0.25rem;
  }
}
</style>
