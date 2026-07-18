<script setup lang="ts">
import { reactive, onMounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import {
  LogoutOutlined,
  MobileOutlined,
  QqOutlined,
  WechatOutlined,
} from '@ant-design/icons-vue'
import { getUserInfo, clearAuth } from '@/utils/auth'
import { ApiService } from '@/services/api'

const router = useRouter()
const theme = inject<{ value: string }>('theme')
const toggleTheme = inject<() => void>('toggleTheme')

const userInfo = getUserInfo()

// 绑定信息
const bindings = reactive({
  phone: '',
  phone_verified: false,
  email: '',
  providers: [] as Array<{
    provider: string
    open_id: string
    union_id?: string
    nickname: string
  }>,
  loading: false,
})

const bindForms = reactive({
  phone: '',
  qqOpenId: '',
  qqNickname: '',
  wxOpenId: '',
  wxUnionId: '',
  wxNickname: '',
})

const bindLoading = reactive({
  phone: false,
  qq: false,
  wx: false,
})

const qqBound = computed(() =>
  bindings.providers.some((p) => p.provider === 'qq')
)
const wechatBound = computed(() =>
  bindings.providers.some((p) => p.provider === 'wechat')
)

const loadBindings = async () => {
  bindings.loading = true
  try {
    const data = await ApiService.getBindings()
    bindings.phone = data.phone || ''
    bindings.phone_verified = data.phone_verified
    bindings.email = data.email || ''
    bindings.providers = data.providers || []
  } catch (err: any) {
    console.error('获取绑定信息失败', err)
    message.error('获取绑定信息失败')
  } finally {
    bindings.loading = false
  }
}

const isValidPhone = (phone: string) => /^1[3-9]\d{9}$/.test(phone)

const handleBindPhone = async () => {
  if (!isValidPhone(bindForms.phone)) {
    message.error('请输入正确的 11 位手机号')
    return
  }
  bindLoading.phone = true
  try {
    const res = await ApiService.bindPhone(bindForms.phone)
    if (res.success) {
      message.success(res.message)
      bindForms.phone = ''
      await loadBindings()
    } else {
      message.error(res.message)
    }
  } catch (err: any) {
    message.error(err.response?.data?.message || '绑定失败')
  } finally {
    bindLoading.phone = false
  }
}

const handleBindQQ = async () => {
  if (!bindForms.qqOpenId.trim()) {
    message.error('请输入 QQ openid')
    return
  }
  bindLoading.qq = true
  try {
    const res = await ApiService.bindThirdParty(
      'qq',
      bindForms.qqOpenId.trim(),
      bindForms.qqNickname.trim() || undefined
    )
    if (res.success) {
      message.success(res.message)
      bindForms.qqOpenId = ''
      bindForms.qqNickname = ''
      await loadBindings()
    } else {
      message.error(res.message)
    }
  } catch (err: any) {
    message.error(err.response?.data?.message || '绑定失败')
  } finally {
    bindLoading.qq = false
  }
}

const handleBindWechat = async () => {
  if (!bindForms.wxOpenId.trim()) {
    message.error('请输入微信 openid')
    return
  }
  bindLoading.wx = true
  try {
    const res = await ApiService.bindThirdParty(
      'wechat',
      bindForms.wxOpenId.trim(),
      bindForms.wxNickname.trim() || undefined,
      bindForms.wxUnionId.trim() || undefined
    )
    if (res.success) {
      message.success(res.message)
      bindForms.wxOpenId = ''
      bindForms.wxUnionId = ''
      bindForms.wxNickname = ''
      await loadBindings()
    } else {
      message.error(res.message)
    }
  } catch (err: any) {
    message.error(err.response?.data?.message || '绑定失败')
  } finally {
    bindLoading.wx = false
  }
}

const handleLogout = () => {
  clearAuth()
  message.success('已退出登录')
  router.push('/login')
}

onMounted(loadBindings)
</script>

<template>
  <div class="settings-page">
    <div class="settings-card">
      <h1 class="settings-title">个人设置</h1>

      <!-- 账号信息 -->
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
        <div class="info-item">
          <span class="info-label">手机号</span>
          <span class="info-value">
            {{ bindings.phone || '未绑定' }}
            <a-tag v-if="bindings.phone && bindings.phone_verified" color="success" class="bind-tag">
              已验证
            </a-tag>
          </span>
        </div>
        <div class="info-item">
          <span class="info-label">邮箱</span>
          <span class="info-value">{{ bindings.email || '未绑定' }}</span>
        </div>
      </div>

      <!-- 账号绑定 -->
      <div class="settings-section">
        <h2 class="section-title">账号绑定</h2>

        <!-- 手机绑定 -->
        <div class="bind-card">
          <div class="bind-header">
            <MobileOutlined class="bind-icon phone-icon" />
            <div class="bind-meta">
              <div class="bind-name">手机绑定</div>
              <div class="bind-status">
                {{ bindings.phone ? `已绑定：${bindings.phone}` : '未绑定' }}
              </div>
            </div>
          </div>
          <div class="bind-form">
            <a-input
              v-model:value="bindForms.phone"
              placeholder="请输入手机号"
              maxlength="11"
              :disabled="bindLoading.phone"
            />
            <a-button
              type="primary"
              :loading="bindLoading.phone"
              @click="handleBindPhone"
            >
              {{ bindings.phone ? '更换手机号' : '立即绑定' }}
            </a-button>
          </div>
        </div>

        <!-- QQ 绑定 -->
        <div class="bind-card">
          <div class="bind-header">
            <QqOutlined class="bind-icon qq-icon" />
            <div class="bind-meta">
              <div class="bind-name">QQ 绑定</div>
              <div class="bind-status">
                {{ qqBound ? '已绑定' : '未绑定' }}
              </div>
            </div>
          </div>
          <div class="bind-form">
            <a-input
              v-model:value="bindForms.qqOpenId"
              placeholder="QQ openid"
              :disabled="bindLoading.qq"
            />
            <a-input
              v-model:value="bindForms.qqNickname"
              placeholder="QQ 昵称（可选）"
              :disabled="bindLoading.qq"
            />
            <a-button
              type="primary"
              :loading="bindLoading.qq"
              @click="handleBindQQ"
            >
              {{ qqBound ? '更新绑定' : '立即绑定' }}
            </a-button>
          </div>
        </div>

        <!-- 微信绑定 -->
        <div class="bind-card">
          <div class="bind-header">
            <WechatOutlined class="bind-icon wechat-icon" />
            <div class="bind-meta">
              <div class="bind-name">微信绑定</div>
              <div class="bind-status">
                {{ wechatBound ? '已绑定' : '未绑定' }}
              </div>
            </div>
          </div>
          <div class="bind-form">
            <a-input
              v-model:value="bindForms.wxOpenId"
              placeholder="微信 openid"
              :disabled="bindLoading.wx"
            />
            <a-input
              v-model:value="bindForms.wxUnionId"
              placeholder="微信 unionid（可选）"
              :disabled="bindLoading.wx"
            />
            <a-input
              v-model:value="bindForms.wxNickname"
              placeholder="微信昵称（可选）"
              :disabled="bindLoading.wx"
            />
            <a-button
              type="primary"
              :loading="bindLoading.wx"
              @click="handleBindWechat"
            >
              {{ wechatBound ? '更新绑定' : '立即绑定' }}
            </a-button>
          </div>
        </div>
      </div>

      <!-- 外观 -->
      <div class="settings-section">
        <h2 class="section-title">外观</h2>
        <div class="info-item">
          <span class="info-label">主题模式</span>
          <a-button type="primary" class="theme-btn" @click="toggleTheme">
            {{ theme?.value === 'light' ? '🌙 切换深色模式' : '☀️ 切换浅色模式' }}
          </a-button>
        </div>
      </div>

      <!-- 退出登录 -->
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
  max-width: 640px;
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
  margin-bottom: 1.75rem;
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
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.bind-tag {
  font-size: 0.75rem;
}

.bind-card {
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-lg);
  padding: 1rem;
  margin-bottom: 1rem;
}

.bind-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.75rem;
}

.bind-icon {
  font-size: 1.5rem;
}

.phone-icon {
  color: var(--primary-color);
}

.qq-icon {
  color: #12b7f5;
}

.wechat-icon {
  color: #07c160;
}

.bind-meta {
  flex: 1;
}

.bind-name {
  font-weight: 600;
  color: var(--text-primary);
}

.bind-status {
  font-size: 0.85rem;
  color: var(--text-secondary);
}

.bind-form {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.bind-form .ant-input {
  flex: 1;
  min-width: 140px;
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
    padding: 1.25rem;
  }

  .info-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.25rem;
  }

  .bind-form {
    flex-direction: column;
  }

  .bind-form .ant-input,
  .bind-form .ant-btn {
    width: 100%;
  }
}
</style>
