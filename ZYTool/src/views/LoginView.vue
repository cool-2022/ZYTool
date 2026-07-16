<template>
  <div class="login-page">
    <div class="login-background">
      <div class="bg-shape shape-1"></div>
      <div class="bg-shape shape-2"></div>
      <div class="bg-shape shape-3"></div>
    </div>

    <div class="login-container">
      <div class="login-card">
        <div class="login-header">
          <div class="login-logo">
            <span class="logo-icon">⚡</span>
            <span class="logo-text">ZYTool</span>
          </div>
          <h2 class="login-title">{{ isLogin ? '欢迎回来' : '创建账号' }}</h2>
          <p class="login-subtitle">
            {{ isLogin ? '登录以访问所有工具' : '注册开始使用 ZYTool' }}
          </p>
        </div>

        <a-form
          :model="formState"
          @finish="handleSubmit"
          layout="vertical"
          class="login-form"
        >
          <a-form-item
            label="用户名"
            name="username"
            :rules="[{ required: true, message: '请输入用户名' }, { min: 3, message: '用户名至少3个字符' }]"
          >
            <a-input
              v-model:value="formState.username"
              placeholder="请输入用户名"
              size="large"
            >
              <template #prefix>
                <UserOutlined />
              </template>
            </a-input>
          </a-form-item>

          <a-form-item
            label="密码"
            name="password"
            :rules="[{ required: true, message: '请输入密码' }, { min: 6, message: '密码至少6个字符' }]"
          >
            <a-input-password
              v-model:value="formState.password"
              placeholder="请输入密码"
              size="large"
            >
              <template #prefix>
                <LockOutlined />
              </template>
            </a-input-password>
          </a-form-item>

          <a-form-item v-if="!isLogin" label="邮箱" name="email">
            <a-input
              v-model:value="formState.email"
              placeholder="请输入邮箱（可选）"
              size="large"
            >
              <template #prefix>
                <MailOutlined />
              </template>
            </a-input>
          </a-form-item>

          <a-form-item>
            <a-button
              type="primary"
              html-type="submit"
              :loading="loading"
              block
              size="large"
              class="submit-btn"
            >
              {{ isLogin ? '登录' : '注册' }}
            </a-button>
          </a-form-item>
        </a-form>

        <div class="login-footer">
          <a class="toggle-link" @click="toggleMode">
            {{ isLogin ? '还没有账号？去注册' : '已有账号？去登录' }}
          </a>

          <div class="demo-accounts">
            <div class="demo-title">测试账号</div>
            <div class="demo-list">
              <div class="demo-item">
                <span class="demo-label">管理员</span>
                <span class="demo-value">admin / admin123</span>
              </div>
              <div class="demo-item">
                <span class="demo-label">普通用户</span>
                <span class="demo-value">user / user123</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter, useRoute } from 'vue-router'
import { UserOutlined, LockOutlined, MailOutlined } from '@ant-design/icons-vue'
import { ApiService } from '../services/api'
import { setToken, setUserInfo } from '../utils/auth'

const router = useRouter()
const route = useRoute()
const isLogin = ref(true)
const loading = ref(false)

const formState = reactive({
  username: '',
  password: '',
  email: ''
})

const toggleMode = () => {
  isLogin.value = !isLogin.value
  formState.username = ''
  formState.password = ''
  formState.email = ''
}

const handleSubmit = async () => {
  loading.value = true

  try {
    let response

    if (isLogin.value) {
      response = await ApiService.login(formState.username, formState.password)
    } else {
      response = await ApiService.register(formState.username, formState.password, formState.email)
    }

    if (response.success && response.data) {
      setToken(response.data.access_token)
      const userInfo = await ApiService.getCurrentUser()
      setUserInfo(userInfo)

      message.success(response.message)
      const redirect = route.query.redirect as string || '/home'
      router.push(redirect)
    } else {
      message.error(response.message || '操作失败')
    }
  } catch (error: any) {
    console.error('Auth error:', error)
    message.error(error.response?.data?.detail || error.response?.data?.message || '操作失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-bg);
  position: relative;
  overflow: hidden;
  padding: 24px;
}

.login-background {
  position: absolute;
  inset: 0;
  pointer-events: none;
  overflow: hidden;
}

.bg-shape {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.5;
}

.shape-1 {
  width: 500px;
  height: 500px;
  top: -200px;
  right: -100px;
  background: var(--primary-color);
  animation: float 10s ease-in-out infinite;
}

.shape-2 {
  width: 400px;
  height: 400px;
  bottom: -150px;
  left: -100px;
  background: var(--accent-color);
  animation: float 12s ease-in-out infinite reverse;
}

.shape-3 {
  width: 300px;
  height: 300px;
  top: 50%;
  left: 60%;
  background: linear-gradient(135deg, var(--primary-light), var(--accent-light));
  animation: float 8s ease-in-out infinite 2s;
}

.login-container {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 440px;
}

.login-card {
  background: var(--glass-bg);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--glass-border);
  border-radius: var(--border-radius-xl);
  padding: 2.5rem;
  box-shadow: var(--shadow-xl);
}

.login-header {
  text-align: center;
  margin-bottom: 2rem;
}

.login-logo {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1.25rem;
}

.logo-icon {
  font-size: 1.5rem;
}

.logo-text {
  font-size: 1.5rem;
  font-weight: 800;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.login-title {
  font-size: 1.75rem;
  font-weight: 800;
  color: var(--text-primary);
  margin: 0 0 0.5rem;
}

.login-subtitle {
  font-size: 0.95rem;
  color: var(--text-secondary);
  margin: 0;
}

.login-form :deep(.ant-form-item-label > label) {
  color: var(--text-secondary);
  font-weight: 500;
}

.login-form :deep(.ant-input),
.login-form :deep(.ant-input-affix-wrapper) {
  background: var(--bg-primary);
  border-color: var(--border-color);
  color: var(--text-primary);
  border-radius: var(--border-radius);
}

.login-form :deep(.ant-input:hover),
.login-form :deep(.ant-input-affix-wrapper:hover) {
  border-color: var(--primary-light);
}

.login-form :deep(.ant-input:focus),
.login-form :deep(.ant-input-affix-wrapper-focused) {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.login-form :deep(.ant-input-prefix) {
  color: var(--text-tertiary);
  margin-right: 0.5rem;
}

.submit-btn {
  background: var(--gradient-primary);
  border: none;
  border-radius: var(--border-radius);
  height: 46px;
  font-weight: 600;
  font-size: 1rem;
  box-shadow: var(--shadow-primary);
  transition: all 0.3s ease;
}

.submit-btn:hover {
  background: var(--gradient-primary-hover);
  box-shadow: var(--shadow-primary-hover);
  transform: translateY(-1px);
}

.login-footer {
  margin-top: 1.5rem;
  text-align: center;
}

.toggle-link {
  color: var(--primary-color);
  font-weight: 500;
  cursor: pointer;
  transition: color 0.3s ease;
}

.toggle-link:hover {
  color: var(--primary-dark);
  text-decoration: underline;
}

.demo-accounts {
  margin-top: 1.5rem;
  padding: 1rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-lg);
  text-align: left;
}

.demo-title {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 0.75rem;
}

.demo-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.demo-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 0.85rem;
}

.demo-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.demo-value {
  color: var(--text-primary);
  font-family: 'SF Mono', monospace;
  background: var(--bg-tertiary);
  padding: 0.2rem 0.5rem;
  border-radius: 4px;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-20px) scale(1.05);
  }
}

@media (max-width: 480px) {
  .login-card {
    padding: 1.75rem;
  }

  .login-title {
    font-size: 1.5rem;
  }

  .demo-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.25rem;
  }
}
</style>
