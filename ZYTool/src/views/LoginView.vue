<template>
  <div class="login-page warm-form-page">
    <div class="login-container">
      <div class="login-card warm-form-card">
        <div class="login-header">
          <div class="login-logo">
            <span class="logo-icon"><ThunderboltOutlined /></span>
            <span class="logo-text">ZYTool</span>
          </div>
          <div class="login-eyebrow">ZYTOOL WORKSPACE</div>
          <h1 class="login-title">{{ isLogin ? '欢迎回来' : '创建账号' }}</h1>
          <p class="login-subtitle">
            {{ isLogin ? '登录后继续你的创作与高效工作' : '注册账号，开始探索 ZYTool 的全部能力' }}
          </p>
        </div>

        <a-form :model="formState" @finish="handleSubmit" layout="vertical" class="login-form warm-form">
          <a-form-item label="用户名" name="username"
            :rules="[{ required: true, message: '请输入用户名' }, { min: 3, message: '用户名至少3个字符' }]">
            <a-input v-model:value="formState.username" placeholder="请输入用户名" size="large">
              <template #prefix>
                <UserOutlined />
              </template>
            </a-input>
          </a-form-item>

          <a-form-item label="密码" name="password"
            :rules="[{ required: true, message: '请输入密码' }, { min: 6, message: '密码至少6个字符' }]">
            <a-input-password v-model:value="formState.password" placeholder="请输入密码" size="large">
              <template #prefix>
                <LockOutlined />
              </template>
            </a-input-password>
          </a-form-item>

          <a-form-item v-if="!isLogin" label="邮箱" name="email">
            <a-input v-model:value="formState.email" placeholder="请输入邮箱（可选）" size="large">
              <template #prefix>
                <MailOutlined />
              </template>
            </a-input>
          </a-form-item>

          <a-form-item class="submit-item">
            <a-button type="primary" html-type="submit" :loading="loading" block size="large" class="submit-btn warm-form-submit">
              {{ isLogin ? '登录' : '注册' }}
            </a-button>
          </a-form-item>

          <div class="social-login">
            <div class="social-divider">
              <span class="divider-line"></span>
              <span class="divider-text">或使用</span>
              <span class="divider-line"></span>
            </div>
            <a-button type="default" block size="large" class="qq-login-btn warm-form-secondary" @click="handleQQLogin"
              :loading="qqLoading">
              <QqOutlined class="qq-icon" />
              <span>QQ 关联登录</span>
            </a-button>
          </div>
        </a-form>

        <div class="login-footer">
          <a class="toggle-link warm-form-link" @click="toggleMode">
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
      <div class="login-note">安全登录 · 高效工具 · 智能助手</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, onMounted } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter, useRoute } from 'vue-router'
import { UserOutlined, LockOutlined, MailOutlined, ThunderboltOutlined, QqOutlined } from '@ant-design/icons-vue'
import { ApiService } from '../services/api'
import { setToken, setUserInfo } from '../utils/auth'

const router = useRouter()
const route = useRoute()
const isLogin = ref(true)
const loading = ref(false)
const qqLoading = ref(false)

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
    const tokenData = isLogin.value
      ? await ApiService.login(formState.username, formState.password)
      : await ApiService.register(formState.username, formState.password, formState.email)

    setToken(tokenData.access_token)
    const userInfo = await ApiService.getCurrentUser()
    setUserInfo(userInfo)

    message.success(isLogin.value ? '登录成功' : '注册成功')
    const redirect = route.query.redirect as string || '/home'
    router.push(redirect)
  } catch (error: any) {
    console.error('Auth error:', error)
    message.error(error.response?.data?.message || error.response?.data?.detail || '操作失败')
  } finally {
    loading.value = false
  }
}

const handleQQLogin = async () => {
  qqLoading.value = true

  try {
    const { url } = await ApiService.getQQAuthUrl()
    window.location.href = url
  } catch (error: any) {
    console.error('QQ login error:', error)
    message.error('获取QQ授权链接失败')
  } finally {
    qqLoading.value = false
  }
}

const handleQQCallback = async (code: string) => {
  qqLoading.value = true

  try {
    const tokenData = await ApiService.loginWithQQ(code)
    setToken(tokenData.access_token)
    const userInfo = await ApiService.getCurrentUser()
    setUserInfo(userInfo)

    message.success('QQ登录成功')
    const redirect = route.query.redirect as string || '/home'
    router.push(redirect)
  } catch (error: any) {
    console.error('QQ callback error:', error)
    message.error(error.response?.data?.message || 'QQ登录失败')
  } finally {
    qqLoading.value = false
  }
}

onMounted(() => {
  const qqCode = route.query.code as string
  if (qqCode) {
    handleQQCallback(qqCode)
  }
})
</script>

<style scoped>
.login-page {
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.login-container {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 440px;
}

.login-card {
  animation: loginCardIn var(--duration-slow) var(--ease-out) both;
}

@keyframes loginCardIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-header {
  margin-bottom: 24px;
  text-align: center;
}

.login-logo {
  display: inline-flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 16px;
}

.logo-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  color: #fff;
  font-size: 20px;
  background: var(--gradient-primary);
  border-radius: 12px;
  box-shadow: 0 6px 16px color-mix(in srgb, var(--workspace-accent) 22%, transparent);
}

.logo-text {
  color: var(--text-primary);
  font-size: 23px;
  font-weight: 750;
  letter-spacing: -0.035em;
}

.login-eyebrow {
  margin-bottom: 8px;
  color: var(--primary-color);
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.18em;
}

.login-title {
  margin: 0;
  color: var(--text-primary);
  font-size: 28px;
  font-weight: 720;
  line-height: 1.3;
  letter-spacing: -0.035em;
}

.login-subtitle {
  margin: 9px 0 0;
  color: var(--text-secondary);
  font-size: 13px;
  line-height: 1.7;
}

.login-form :deep(.submit-item) {
  margin-top: 22px;
  margin-bottom: 0;
}

.social-login {
  margin-top: 20px;
}

.social-divider {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-bottom: 14px;
}

.divider-line {
  flex: 1;
  height: 1px;
  background: var(--border-color);
}

.divider-text {
  color: var(--text-tertiary);
  font-size: 11px;
}

.qq-login-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
}

.qq-icon {
  color: var(--primary-color);
  font-size: 17px;
}

.login-footer {
  margin-top: 20px;
  text-align: center;
}

.toggle-link {
  font-size: 12px;
}

.demo-accounts {
  margin-top: 20px;
  padding: 13px 14px;
  text-align: left;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 14px;
}

.demo-title {
  margin-bottom: 9px;
  color: var(--text-tertiary);
  font-size: 10px;
  font-weight: 650;
  letter-spacing: 0.08em;
}

.demo-list {
  display: flex;
  flex-direction: column;
  gap: 7px;
}

.demo-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  font-size: 12px;
}

.demo-label {
  color: var(--text-secondary);
  font-weight: 500;
}

.demo-value {
  padding: 2px 7px;
  color: var(--text-primary);
  font-family: 'SF Mono', Consolas, monospace;
  font-size: 11px;
  background: var(--bg-primary);
  border: 1px solid var(--border-light);
  border-radius: 6px;
}

.login-note {
  margin-top: 16px;
  color: var(--text-tertiary);
  font-size: 10px;
  letter-spacing: 0.08em;
  text-align: center;
}

:global(:root[data-theme='dark']) .demo-value,
:global(:root[data-theme='dark']) .qq-login-btn {
  background: var(--bg-tertiary);
}

@media (prefers-reduced-motion: reduce) {
  .login-card {
    animation-name: none;
    opacity: 1;
    transform: none;
  }
}

@media (max-width: 480px) {
  .login-header {
    margin-bottom: 22px;
  }

  .login-title {
    font-size: 25px;
  }

  .demo-item {
    gap: 8px;
  }
}
</style>
