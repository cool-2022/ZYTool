<template>
  <div class="login-container">
    <!-- 登录卡片 -->
    <div class="login-card">
      <div class="login-header">
        <h2>账号登录</h2>
        <p>请使用QQ或微信账号登录系统</p>
      </div>

      <div class="login-methods">
        <!-- QQ登录按钮 -->
        <button
            class="login-btn qq-login"
            @click="handleQQLogin"
        >
          <i class="icon-qq"></i>
          <span>QQ登录</span>
        </button>

        <!-- 微信登录按钮 -->
        <button
            class="login-btn wechat-login"
            @click="handleWechatLogin"
        >
          <i class="icon-wechat"></i>
          <span>微信登录</span>
        </button>
      </div>

      <div class="login-footer">
        <p>登录即表示同意<a href="/terms" target="_blank">用户协议</a>和<a href="/privacy" target="_blank">隐私政策</a></p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'

const router = useRouter()
const isLoading = ref(false)

// QQ登录处理
const handleQQLogin = async () => {
  if (isLoading.value) return
  isLoading.value = true

  try {
    // 实际项目中需要对接QQ开放平台的登录接口
    // 这里仅做示例
    const qqLoginUrl = import.meta.env.VITE_QQ_LOGIN_URL || '/api/qq/login'

    // 1. 跳转到QQ授权页面或调用SDK
    // 对于PC端通常是跳转到授权页面
    window.location.href = qqLoginUrl

    // 2. 授权成功后会回调到指定的回调页面(如/login/callback/qq)
    // 在回调页面中处理token获取和用户信息获取
  } catch (error) {
    message.error('QQ登录失败，请稍后重试')
    console.error('QQ登录错误:', error)
  } finally {
    isLoading.value = false
  }
}

// 微信登录处理
const handleWechatLogin = async () => {
  if (isLoading.value) return
  isLoading.value = true

  try {
    // 实际项目中需要对接微信开放平台的登录接口
    const wechatLoginUrl = import.meta.env.VITE_WECHAT_LOGIN_URL || '/api/wechat/login'

    // 微信登录分PC端和移动端
    if (isMobile()) {
      // 移动端可以使用微信SDK调起微信客户端
      if (window.WeixinJSBridge) {
        // 这里是微信JS-SDK的登录逻辑示例
        window.WeixinJSBridge.invoke('getLoginCode', {}, (res) => {
          if (res.code) {
            // 使用code获取token
            getWechatToken(res.code)
          } else {
            message.error('获取微信授权失败')
          }
        })
      } else {
        // 如果没有WeixinJSBridge，监听事件
        document.addEventListener('WeixinJSBridgeReady', () => {
          handleWechatLogin()
        }, { once: true })
      }
    } else {
      // PC端通常显示二维码登录
      router.push('/login/wechat-qrcode')
    }
  } catch (error) {
    message.error('微信登录失败，请稍后重试')
    console.error('微信登录错误:', error)
  } finally {
    isLoading.value = false
  }
}

// 判断是否为移动端
const isMobile = () => {
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent)
}

// 使用微信code获取token
const getWechatToken = async (code) => {
  try {
    const response = await fetch('/api/wechat/token', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ code })
    })

    const data = await response.json()
    if (data.success && data.token) {
      // 存储token
      localStorage.setItem('token', data.token)
      // 存储用户信息
      if (data.userInfo) {
        localStorage.setItem('userInfo', JSON.stringify(data.userInfo))
      }
      // 跳转到首页
      router.push('/')
      message.success('登录成功')
    } else {
      message.error(data.message || '微信登录失败')
    }
  } catch (error) {
    console.error('获取微信token失败:', error)
    message.error('服务器异常，请稍后重试')
  }
}
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  align-items: center;
  background: #f5f7fa;
  padding: 20px;
}

.login-card {
  width: 100%;
  max-width: 400px;
  background: #fff;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  padding: 30px;
  box-sizing: border-box;
}

.login-header {
  text-align: center;
  margin-bottom: 30px;
}

.login-header h2 {
  font-size: 24px;
  color: #333;
  margin-bottom: 8px;
}

.login-header p {
  color: #666;
  font-size: 14px;
}

.login-methods {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.login-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  width: 100%;
  padding: 14px 0;
  border: none;
  border-radius: 8px;
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.login-btn:hover {
  opacity: 0.9;
  transform: translateY(-2px);
}

.qq-login {
  background-color: #12B7F5;
  color: #fff;
}

.wechat-login {
  background-color: #07C160;
  color: #fff;
}

.icon-qq, .icon-wechat {
  font-size: 20px;
}

/* 可以使用字体图标或背景图替代 */
.icon-qq::before {
  content: "🐧";
}

.icon-wechat::before {
  content: "💬";
}

.login-footer {
  margin-top: 30px;
  text-align: center;
  font-size: 12px;
  color: #999;
}

.login-footer a {
  color: #1677ff;
  text-decoration: none;
  margin: 0 4px;
}

.login-footer a:hover {
  text-decoration: underline;
}
</style>