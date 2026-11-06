<script setup lang="ts">
import { ref, computed, onMounted, provide, readonly } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { BulbFilled, BulbOutlined, SettingOutlined, LogoutOutlined, ThunderboltOutlined } from '@ant-design/icons-vue'
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

// 仅管理员可见的后台管理入口
const isAdmin = computed(() => (userInfo.value?.roles || []).includes('admin'))

const handleLogout = () => {
  clearAuth()
  refreshAuth()
  message.success('已退出登录')
  router.push('/')
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
        <router-link to="/home" class="nav-logo" aria-label="ZYTool 首页">
          <span class="logo-icon"><ThunderboltOutlined /></span>
          <span class="logo-text">ZYTool<span class="logo-dot">.</span></span>
        </router-link>
        <div class="nav-menu">
          <router-link to="/home" class="nav-link">首页</router-link>
          <router-link to="/tools" class="nav-link">工具</router-link>
          <router-link to="/agents/chat" class="nav-link">AI 助手</router-link>
          <router-link to="/finance/ledger" class="nav-link finance-nav-link">财务</router-link>
          <router-link v-if="isAdmin" to="/admin" class="nav-link">管理</router-link>

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
            :aria-label="theme === 'light' ? '切换到深色模式' : '切换到浅色模式'"
          >
            <BulbOutlined class="theme-icon" v-if="theme === 'light'" />
            <BulbFilled class="theme-icon" v-else />
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
  background: var(--workspace-surface);
  border-bottom: 1px solid var(--workspace-line);
  padding: 10px var(--page-gutter);
  position: sticky;
  top: 0;
  z-index: 1000;
  transition: background-color var(--duration-base) var(--ease-out),
    border-color var(--duration-base) var(--ease-out);
}

.nav-container {
  max-width: var(--page-max-width);
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-height: 38px;
  gap: 12px;
  padding: 0;
}

.nav-logo {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  gap: 9px;
  text-decoration: none;
}

.logo-icon {
  display: grid;
  place-items: center;
  width: 32px;
  height: 32px;
  border-radius: 9px;
  background: var(--workspace-accent-soft);
  color: var(--workspace-accent);
  font-size: 19px;
}

.logo-text {
  font-size: 21px;
  font-weight: 650;
  color: var(--workspace-text);
  letter-spacing: -0.5px;
}

.logo-dot { color: var(--workspace-accent); padding-left: 2px; }

.nav-menu {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.nav-link {
  flex-shrink: 0;
  white-space: nowrap;
  color: var(--workspace-muted);
  text-decoration: none;
  padding: 7px 15px;
  border-radius: 8px;
  transition: color var(--duration-fast) var(--ease-out),
    background-color var(--duration-fast) var(--ease-out),
    transform var(--duration-press) var(--ease-out);
  font-weight: 500;
  font-size: 13px;
}

@media (hover: hover) and (pointer: fine) {
  .nav-link:hover { color: var(--workspace-accent); background: var(--workspace-soft); }
}

.nav-link.router-link-active {
  color: var(--workspace-accent);
  background: var(--workspace-accent-soft);
  font-weight: 600;
}

.nav-link-primary, .nav-link-primary.router-link-active {
  background: var(--workspace-accent);
  color: var(--workspace-bg);
  margin-left: 8px;
}

.nav-logo:focus-visible, .nav-link:focus-visible { outline: 2px solid var(--workspace-accent); outline-offset: 3px; }

@media (hover: hover) and (pointer: fine) {
  .nav-link-primary:hover { background: var(--workspace-accent-hover); color: var(--workspace-bg); }
}

.nav-link-primary:active {
  transform: scale(0.97);
}

.user-avatar-btn {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  height: 32px;
  padding: 0 0.5rem;
  margin-left: 0.5rem;
  border-radius: 8px;
  color: var(--workspace-muted) !important;
  background: transparent;
  box-shadow: none;
  transition: color var(--duration-fast) var(--ease-out),
    background-color var(--duration-fast) var(--ease-out);
}

@media (hover: hover) and (pointer: fine) {
  .user-avatar-btn:hover {
    color: var(--workspace-accent) !important;
    background: var(--workspace-soft);
    box-shadow: none;
    transform: none;
  }
}

.user-avatar {
  background: var(--workspace-accent-soft);
  color: var(--workspace-accent);
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
  color: var(--workspace-muted) !important;
  background: transparent;
  box-shadow: none;
  width: 32px;
  height: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 8px;
  margin-left: 8px;
  transition: color var(--duration-fast) var(--ease-out),
    background-color var(--duration-fast) var(--ease-out);
}

.theme-toggle:focus-visible, .user-avatar-btn:focus-visible { outline-color: var(--workspace-accent); }

@media (hover: hover) and (pointer: fine) {
  .theme-toggle:hover {
    color: var(--workspace-accent) !important;
    background: var(--workspace-soft);
    transform: none;
    box-shadow: none;
  }
}

.theme-icon {
  font-size: 18px;
  line-height: 1;
}

@media (max-width: 768px) {
  .navbar { padding: 9px var(--page-gutter); }
  .nav-link { padding: 7px 12px; }
  .nav-link-primary, .user-avatar-btn, .theme-toggle { margin-left: 4px; }
  .user-name { max-width: 60px; }
}

@media (max-width: 480px) {
  .navbar { padding: 9px var(--page-gutter); }
  .nav-container { gap: 8px; }
  .nav-logo { gap: 5px; }
  .logo-icon { width: 27px; height: 29px; font-size: 17px; border-radius: 7px; }
  .logo-text { font-size: 17px; }
  .nav-menu { gap: 0; }
  .nav-link { padding: 7px 6px; font-size: 12px; }
  .user-avatar-btn { padding: 0 4px; }
  .user-name { display: none; }
  .theme-toggle { width: 28px; margin-left: 2px; }
}

@media (max-width: 400px) {
  .nav-logo .logo-text { display: none; }
  .nav-logo { gap: 0; }
}

@media (max-width: 360px) {
  .navbar { padding-left: var(--page-gutter); padding-right: var(--page-gutter); }
  .nav-link { padding-left: 5px; padding-right: 5px; }
  .logo-dot { display: none; }
}
</style>
