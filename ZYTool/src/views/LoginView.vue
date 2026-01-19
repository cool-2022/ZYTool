<template>
  <div class="login-container">
    <div class="login-box">
      <h2>{{ isLogin ? '登录' : '注册' }}</h2>
      
      <a-form
        :model="formState"
        @finish="handleSubmit"
        layout="vertical"
      >
        <a-form-item
          label="用户名"
          name="username"
          :rules="[{ required: true, message: '请输入用户名' }, { min: 3, message: '用户名至少3个字符' }]"
        >
          <a-input v-model:value="formState.username" placeholder="请输入用户名" />
        </a-form-item>

        <a-form-item
          label="密码"
          name="password"
          :rules="[{ required: true, message: '请输入密码' }, { min: 6, message: '密码至少6个字符' }]"
        >
          <a-input-password v-model:value="formState.password" placeholder="请输入密码" />
        </a-form-item>

        <a-form-item v-if="!isLogin" label="邮箱" name="email">
          <a-input v-model:value="formState.email" placeholder="请输入邮箱（可选）" />
        </a-form-item>

        <a-form-item>
          <a-button type="primary" html-type="submit" :loading="loading" block>
            {{ isLogin ? '登录' : '注册' }}
          </a-button>
        </a-form-item>
      </a-form>

      <div class="switch-mode">
        <a @click="toggleMode">
          {{ isLogin ? '没有账号？去注册' : '已有账号？去登录' }}
        </a>
      </div>

      <div class="demo-accounts">
        <p>测试账号：</p>
        <p>管理员 - 用户名: admin, 密码: admin123</p>
        <p>普通用户 - 用户名: user, 密码: user123</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { message } from 'ant-design-vue'
import { useRouter, useRoute } from 'vue-router'
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
      // 登录
      response = await ApiService.login(formState.username, formState.password)
    } else {
      // 注册
      response = await ApiService.register(formState.username, formState.password, formState.email)
    }
    
    if (response.success && response.data) {
      // 保存 token
      setToken(response.data.access_token)
      
      // 获取用户信息
      const userInfo = await ApiService.getCurrentUser()
      setUserInfo(userInfo)
      
      message.success(response.message)
      
      // 获取重定向路径，如果没有则跳转到首页
      const redirect = route.query.redirect as string || '/home'
      
      // 跳转到目标页面
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
.login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-box {
  background: white;
  padding: 40px;
  border-radius: 10px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  width: 100%;
  max-width: 400px;
}

.login-box h2 {
  text-align: center;
  margin-bottom: 30px;
  color: #333;
}

.switch-mode {
  text-align: center;
  margin-top: 20px;
}

.switch-mode a {
  color: #667eea;
  cursor: pointer;
}

.switch-mode a:hover {
  text-decoration: underline;
}

.demo-accounts {
  margin-top: 30px;
  padding: 15px;
  background: #f5f5f5;
  border-radius: 5px;
  font-size: 12px;
  color: #666;
}

.demo-accounts p {
  margin: 5px 0;
}
</style>
