<script setup lang="ts">
import { reactive, onMounted, computed, inject, ref } from 'vue'
import { useRouter } from 'vue-router'
import { message, theme as antdTheme } from 'ant-design-vue'
import {
  LogoutOutlined,
  MobileOutlined,
  QqOutlined,
  WechatOutlined,
  SettingOutlined,
  UserOutlined,
  LinkOutlined,
  BulbOutlined,
  BulbFilled,
  CheckCircleOutlined,
} from '@ant-design/icons-vue'
import { getUserInfo, clearAuth } from '@/utils/auth'
import { ApiService } from '@/services/api'

const router = useRouter()
const theme = inject<{ value: string }>('theme')
const toggleTheme = inject<() => void>('toggleTheme')
const activeTab = ref<'account' | 'bindings' | 'appearance'>('account')
const isDark = computed(() => theme?.value === 'dark')
const settingsTheme = computed(() => ({
  algorithm: isDark.value ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm,
  token: {
    colorPrimary: isDark.value ? '#ed9b7e' : '#cf654a',
    colorInfo: isDark.value ? '#ed9b7e' : '#cf654a',
    colorSuccess: isDark.value ? '#a0c0a8' : '#5d8d74',
    colorBgBase: isDark.value ? '#24221f' : '#f8f6f0',
    colorBgContainer: isDark.value ? '#2e2b27' : '#fffdf9',
    colorBgElevated: isDark.value ? '#2e2b27' : '#fffdf9',
    colorText: isDark.value ? '#eee8df' : '#423e38',
    colorTextSecondary: isDark.value ? '#c0b4a4' : '#81786e',
    colorBorder: isDark.value ? '#433c33' : '#ece6dc',
    colorPrimaryBg: isDark.value ? '#443027' : '#f3e7dd',
    borderRadius: 8,
  },
}))

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
    await ApiService.bindPhone(bindForms.phone)
    message.success('手机号绑定成功')
    bindForms.phone = ''
    await loadBindings()
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
    await ApiService.bindThirdParty(
      'qq',
      bindForms.qqOpenId.trim(),
      bindForms.qqNickname.trim() || undefined
    )
    message.success('QQ 绑定成功')
    bindForms.qqOpenId = ''
    bindForms.qqNickname = ''
    await loadBindings()
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
    await ApiService.bindThirdParty(
      'wechat',
      bindForms.wxOpenId.trim(),
      bindForms.wxNickname.trim() || undefined,
      bindForms.wxUnionId.trim() || undefined
    )
    message.success('微信绑定成功')
    bindForms.wxOpenId = ''
    bindForms.wxUnionId = ''
    bindForms.wxNickname = ''
    await loadBindings()
  } catch (err: any) {
    message.error(err.response?.data?.message || '绑定失败')
  } finally {
    bindLoading.wx = false
  }
}

const handleLogout = () => {
  clearAuth()
  message.success('已退出登录')
  router.push('/')
}

onMounted(loadBindings)
</script>

<template>
  <a-config-provider :theme="settingsTheme">
    <main class="settings-page warm-form-page">
      <div class="settings-shell">
        <header class="settings-header">
          <div class="settings-brand">
            <span class="settings-mark" aria-hidden="true"><SettingOutlined /></span>
            <div class="settings-heading">
              <h1>个人设置<span class="title-dot">.</span></h1>
              <p>管理账号，调整你的使用偏好</p>
            </div>
          </div>
          <span class="settings-badge"><UserOutlined /> {{ userInfo?.username || '我的账号' }}</span>
        </header>

        <nav class="settings-toolbar" aria-label="设置分类">
          <div class="settings-menu">
            <button type="button" class="settings-tab" :class="{ selected: activeTab === 'account' }"
              :aria-pressed="activeTab === 'account'" aria-controls="settings-account" @click="activeTab = 'account'">
              <UserOutlined /><span>账号信息</span>
            </button>
            <button type="button" class="settings-tab" :class="{ selected: activeTab === 'bindings' }"
              :aria-pressed="activeTab === 'bindings'" aria-controls="settings-bindings" @click="activeTab = 'bindings'">
              <LinkOutlined /><span>账号绑定</span>
            </button>
            <button type="button" class="settings-tab" :class="{ selected: activeTab === 'appearance' }"
              :aria-pressed="activeTab === 'appearance'" aria-controls="settings-appearance" @click="activeTab = 'appearance'">
              <BulbOutlined /><span>外观</span>
            </button>
          </div>
        </nav>

        <a-spin :spinning="bindings.loading && activeTab !== 'appearance'">
          <section id="settings-account" v-show="activeTab === 'account'" class="settings-content" aria-labelledby="account-title">
            <div class="section-header">
              <div>
                <h2 id="account-title">账号信息</h2>
                <p>查看你的基本资料与联系方式</p>
              </div>
              <a-button class="secondary-button warm-form-secondary" @click="activeTab = 'bindings'"><LinkOutlined /> 管理绑定</a-button>
            </div>
            <div class="profile-summary">
              <a-avatar :size="48" class="profile-avatar">
                {{ (userInfo?.username || 'U').charAt(0).toUpperCase() }}
              </a-avatar>
              <div class="profile-copy">
                <strong>{{ userInfo?.username || '未登录' }}</strong>
                <span>ZYTool 个人账号</span>
              </div>
            </div>
            <dl class="info-grid">
              <div class="info-item">
                <dt class="info-label">用户名</dt>
                <dd class="info-value">{{ userInfo?.username || '-' }}</dd>
              </div>
              <div class="info-item">
                <dt class="info-label">角色</dt>
                <dd class="info-value">{{ (userInfo?.roles || []).join(', ') || '-' }}</dd>
              </div>
              <div class="info-item">
                <dt class="info-label">手机号</dt>
                <dd class="info-value">
                  {{ bindings.phone || '未绑定' }}
                  <a-tag v-if="bindings.phone && bindings.phone_verified" class="bind-tag"><CheckCircleOutlined /> 已验证</a-tag>
                </dd>
              </div>
              <div class="info-item">
                <dt class="info-label">邮箱</dt>
                <dd class="info-value">{{ bindings.email || '未绑定' }}</dd>
              </div>
            </dl>
          </section>

          <section id="settings-bindings" v-show="activeTab === 'bindings'" class="settings-content" aria-labelledby="bindings-title">
            <div class="section-header">
              <div>
                <h2 id="bindings-title">账号绑定</h2>
                <p>管理手机号、QQ 和微信的关联信息</p>
              </div>
            </div>
            <div class="binding-grid">
              <article class="bind-card">
                <div class="bind-header">
                  <span class="bind-icon phone-icon" aria-hidden="true"><MobileOutlined /></span>
                  <div class="bind-meta">
                    <h3>手机号</h3>
                    <span class="bind-status" :class="{ bound: !!bindings.phone }">{{ bindings.phone ? '已绑定' : '未绑定' }}</span>
                  </div>
                </div>
                <p class="bind-description">{{ bindings.phone ? `当前号码：${bindings.phone}` : '绑定常用手机号，方便管理账号信息。' }}</p>
                <div class="bind-form warm-form">
                  <div class="bind-field">
                    <label for="settings-phone">手机号</label>
                    <a-input id="settings-phone" v-model:value="bindForms.phone" placeholder="请输入 11 位手机号"
                      maxlength="11" inputmode="tel" autocomplete="tel" :disabled="bindLoading.phone" />
                  </div>
                  <p class="field-note">填写新号码后可更换现有绑定。</p>
                  <a-button type="primary" class="save-button warm-form-submit" :loading="bindLoading.phone" @click="handleBindPhone">
                    {{ bindings.phone ? '更换手机号' : '立即绑定' }}
                  </a-button>
                </div>
              </article>

              <article class="bind-card">
                <div class="bind-header">
                  <span class="bind-icon qq-icon" aria-hidden="true"><QqOutlined /></span>
                  <div class="bind-meta">
                    <h3>QQ</h3>
                    <span class="bind-status" :class="{ bound: qqBound }">{{ qqBound ? '已绑定' : '未绑定' }}</span>
                  </div>
                </div>
                <p class="bind-description">填写 QQ 标识完成关联，不会跳转授权。</p>
                <div class="bind-form warm-form">
                  <div class="bind-field">
                    <label for="settings-qq-openid">OpenID</label>
                    <a-input id="settings-qq-openid" v-model:value="bindForms.qqOpenId" placeholder="QQ openid" :disabled="bindLoading.qq" />
                  </div>
                  <div class="bind-field">
                    <label for="settings-qq-nickname">昵称<span>可选</span></label>
                    <a-input id="settings-qq-nickname" v-model:value="bindForms.qqNickname" placeholder="QQ 昵称" :disabled="bindLoading.qq" />
                  </div>
                  <a-button type="primary" class="save-button warm-form-submit" :loading="bindLoading.qq" @click="handleBindQQ">
                    {{ qqBound ? '更新绑定' : '立即绑定' }}
                  </a-button>
                </div>
              </article>

              <article class="bind-card">
                <div class="bind-header">
                  <span class="bind-icon wechat-icon" aria-hidden="true"><WechatOutlined /></span>
                  <div class="bind-meta">
                    <h3>微信</h3>
                    <span class="bind-status" :class="{ bound: wechatBound }">{{ wechatBound ? '已绑定' : '未绑定' }}</span>
                  </div>
                </div>
                <p class="bind-description">填写微信标识完成关联，不会跳转授权。</p>
                <div class="bind-form warm-form">
                  <div class="bind-field">
                    <label for="settings-wx-openid">OpenID</label>
                    <a-input id="settings-wx-openid" v-model:value="bindForms.wxOpenId" placeholder="微信 openid" :disabled="bindLoading.wx" />
                  </div>
                  <div class="bind-field">
                    <label for="settings-wx-unionid">UnionID<span>可选</span></label>
                    <a-input id="settings-wx-unionid" v-model:value="bindForms.wxUnionId" placeholder="微信 unionid" :disabled="bindLoading.wx" />
                  </div>
                  <div class="bind-field">
                    <label for="settings-wx-nickname">昵称<span>可选</span></label>
                    <a-input id="settings-wx-nickname" v-model:value="bindForms.wxNickname" placeholder="微信昵称" :disabled="bindLoading.wx" />
                  </div>
                  <a-button type="primary" class="save-button warm-form-submit" :loading="bindLoading.wx" @click="handleBindWechat">
                    {{ wechatBound ? '更新绑定' : '立即绑定' }}
                  </a-button>
                </div>
              </article>
            </div>
          </section>

          <section id="settings-appearance" v-show="activeTab === 'appearance'" class="settings-content" aria-labelledby="appearance-title">
            <div class="section-header">
              <div>
                <h2 id="appearance-title">外观</h2>
                <p>选择你喜欢的明暗，让日常使用更舒适</p>
              </div>
              <span class="appearance-current">当前：{{ isDark ? '深色模式' : '浅色模式' }}</span>
            </div>
            <div class="appearance-options">
              <button type="button" class="appearance-option" :class="{ selected: !isDark }"
                :aria-pressed="!isDark" @click="isDark && toggleTheme?.()">
                <span class="theme-preview light-preview" aria-hidden="true">
                  <span class="preview-top"><i></i><i></i><i></i></span>
                  <span class="preview-body"><span class="preview-menu"></span><span class="preview-cards"><i></i><i></i></span></span>
                </span>
                <span class="appearance-copy">
                  <span class="appearance-name"><BulbOutlined /> 浅色模式</span>
                  <span class="appearance-description">奶油白背景，轻松明亮</span>
                </span>
                <CheckCircleOutlined v-if="!isDark" class="appearance-check" />
              </button>
              <button type="button" class="appearance-option" :class="{ selected: isDark }"
                :aria-pressed="isDark" @click="!isDark && toggleTheme?.()">
                <span class="theme-preview dark-preview" aria-hidden="true">
                  <span class="preview-top"><i></i><i></i><i></i></span>
                  <span class="preview-body"><span class="preview-menu"></span><span class="preview-cards"><i></i><i></i></span></span>
                </span>
                <span class="appearance-copy">
                  <span class="appearance-name"><BulbFilled /> 深色模式</span>
                  <span class="appearance-description">暖灰背景，柔和安静</span>
                </span>
                <CheckCircleOutlined v-if="isDark" class="appearance-check" />
              </button>
            </div>
            <p class="appearance-note">主题选择与顶栏开关联动，并保存在当前浏览器中。</p>
          </section>
        </a-spin>

        <footer class="settings-footer">
          <div><strong>需要换个账号？</strong><p>退出后可以重新登录</p></div>
          <a-button danger class="logout-button" @click="handleLogout"><LogoutOutlined /><span>退出登录</span></a-button>
        </footer>
      </div>
    </main>
  </a-config-provider>
</template>

<style scoped>
.settings-page {
  min-height: calc(100vh - 59px);
  padding: 20px var(--page-gutter) 40px;
  color: var(--workspace-text);
  background: var(--workspace-bg);
}

.settings-shell { width: 100%; max-width: var(--page-max-width); margin: 0 auto; }

.settings-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--workspace-line);
}

.settings-brand { display: flex; align-items: center; gap: 12px; min-width: 0; }
.settings-heading { min-width: 0; }

.settings-mark {
  display: grid;
  flex-shrink: 0;
  place-items: center;
  width: 42px;
  height: 46px;
  color: var(--workspace-accent);
  font-size: 25px;
  background: var(--workspace-accent-soft);
  border-radius: 11px;
  transform: rotate(-5deg);
}

.settings-heading h1 {
  margin: 0;
  color: var(--workspace-text);
  font-size: 22px;
  font-weight: 600;
  line-height: 1.4;
  letter-spacing: 0.5px;
}

.title-dot { margin-left: 3px; color: var(--workspace-accent); }
.settings-heading p { margin: 4px 0 0; color: var(--workspace-muted); font-size: 12px; line-height: 1.5; }

.settings-badge {
  display: inline-flex;
  flex-shrink: 0;
  align-items: center;
  gap: 6px;
  max-width: 200px;
  padding: 5px 9px;
  overflow: hidden;
  color: var(--workspace-muted);
  font-size: 11px;
  white-space: nowrap;
  background: var(--workspace-surface);
  border: 1px solid var(--workspace-line);
  border-radius: 7px;
}

.settings-toolbar { min-width: 0; margin-bottom: 20px; padding: 9px 0; border-bottom: 1px solid var(--workspace-line); }
.settings-menu { display: flex; flex-wrap: nowrap; gap: 5px; min-width: 0; padding: 4px; overflow-x: auto; }

.settings-tab {
  display: inline-flex;
  flex: 0 0 auto;
  align-items: center;
  justify-content: center;
  gap: 8px;
  height: 38px;
  padding: 0 17px;
  color: var(--workspace-muted);
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  background: transparent;
  border: 0;
  border-radius: 8px;
  box-shadow: none;
  transition:
    color var(--duration-fast) var(--ease-out),
    background-color var(--duration-fast) var(--ease-out),
    transform var(--duration-press) var(--ease-out);
}

.settings-tab.selected { color: var(--workspace-accent); font-weight: 600; background: var(--workspace-accent-soft); }
.settings-tab :deep(.anticon) { font-size: 15px; }

.settings-content {
  min-width: 0;
  padding: 24px;
  background: var(--workspace-surface);
  border: 1px solid var(--workspace-line);
  border-radius: var(--workspace-radius);
  box-shadow: var(--workspace-shadow);
}

.section-header { display: flex; align-items: center; justify-content: space-between; gap: 16px; margin-bottom: 22px; }
.section-header h2 { margin: 0; color: var(--workspace-text); font-size: 19px; font-weight: 600; line-height: 1.4; }
.section-header p { margin: 5px 0 0; color: var(--workspace-muted); font-size: 12px; line-height: 1.6; }
.secondary-button { display: inline-flex !important; flex-shrink: 0; align-items: center; gap: 6px; height: 34px; color: var(--workspace-accent) !important; font-size: 12px; background: var(--workspace-accent-soft) !important; border-color: var(--workspace-accent-line) !important; border-radius: 8px; box-shadow: none !important; }

.profile-summary { display: flex; align-items: center; gap: 13px; margin-bottom: 20px; padding: 18px; background: var(--workspace-soft); border: 1px solid var(--workspace-line); border-radius: 11px; }
.profile-avatar { flex-shrink: 0; color: var(--workspace-accent); font-size: 20px; font-weight: 600; background: var(--workspace-accent-soft); border: 1px solid var(--workspace-accent-line); border-radius: 12px; }
.profile-copy { display: flex; flex-direction: column; gap: 3px; min-width: 0; }
.profile-copy strong { color: var(--workspace-text); font-size: 16px; font-weight: 600; overflow-wrap: anywhere; }
.profile-copy > span { color: var(--workspace-muted); font-size: 11px; }
.info-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 0 28px; margin: 0; }
.info-item { min-width: 0; padding: 16px 0; border-bottom: 1px solid var(--workspace-line); }
.info-item:nth-last-child(-n + 2) { border-bottom: 0; padding-bottom: 0; }
.info-label { margin-bottom: 7px; color: var(--workspace-muted); font-size: 12px; }
.info-value { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; margin: 0; color: var(--workspace-text); font-size: 14px; font-weight: 500; overflow-wrap: anywhere; }
.bind-tag { margin: 0; padding: 1px 6px; color: var(--workspace-green); font-size: 10px; background: var(--workspace-green-soft); border: 0; border-radius: 5px; }

.binding-grid { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 16px; }

.bind-card {
  display: flex;
  flex-direction: column;
  min-width: 0;
  padding: 20px;
  background: var(--workspace-bg);
  border: 1px solid var(--workspace-line);
  border-radius: var(--workspace-radius);
}

.bind-header { display: flex; align-items: center; gap: 11px; }
.bind-icon { display: grid; flex-shrink: 0; place-items: center; width: 40px; height: 42px; font-size: 22px; border-radius: 10px; }
.phone-icon { color: var(--workspace-accent); background: var(--workspace-accent-soft); }
.qq-icon { color: var(--workspace-sand); background: var(--workspace-sand-soft); }
.wechat-icon { color: var(--workspace-green); background: var(--workspace-green-soft); }
.bind-meta { flex: 1; min-width: 0; }
.bind-meta h3 { margin: 0 0 4px; color: var(--workspace-text); font-size: 14px; font-weight: 600; line-height: 1.4; }
.bind-status { display: inline-flex; align-items: center; gap: 5px; color: var(--workspace-caption); font-size: 10px; }
.bind-status::before { width: 5px; height: 5px; background: currentColor; border-radius: 50%; content: ''; }
.bind-status.bound { color: var(--workspace-green); }
.bind-description { min-height: 38px; margin: 12px 0 18px; color: var(--workspace-muted); font-size: 11px; line-height: 1.7; overflow-wrap: anywhere; }
.bind-form { display: flex; flex: 1; flex-direction: column; gap: 13px; min-width: 0; }
.bind-field { min-width: 0; }
.bind-field label { display: flex; align-items: center; gap: 7px; margin-bottom: 6px; color: var(--workspace-muted); font-size: 12px; }
.bind-field label > span { color: var(--workspace-caption); font-size: 10px; }
.field-note { margin: -3px 0 4px; color: var(--workspace-caption); font-size: 11px; line-height: 1.6; }

.settings-page .bind-field :deep(.ant-input) {
  width: 100%;
  height: 36px;
  padding: 6px 10px;
  color: var(--workspace-text) !important;
  font-size: 12px;
  background: var(--workspace-surface) !important;
  border-color: var(--workspace-line) !important;
  border-radius: 8px;
  box-shadow: none;
}

.settings-page .bind-field :deep(.ant-input::placeholder) { color: var(--workspace-caption) !important; }
.settings-page .bind-field :deep(.ant-input:disabled) { color: var(--workspace-caption) !important; background: var(--workspace-soft) !important; }
.settings-page .bind-field :deep(.ant-input:focus) { border-color: var(--workspace-accent-line) !important; box-shadow: 0 0 0 3px var(--workspace-accent-soft); }

.save-button {
  width: 100%;
  height: 36px;
  margin-top: auto;
  color: var(--workspace-bg) !important;
  font-size: 12px;
  font-weight: 500;
  background: var(--workspace-accent) !important;
  border: 0;
  border-radius: 8px;
  box-shadow: none !important;
}

.appearance-current { flex-shrink: 0; padding: 5px 9px; color: var(--workspace-muted); font-size: 11px; background: var(--workspace-soft); border-radius: 7px; }
.appearance-options { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 16px; max-width: 760px; }

.appearance-option {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-width: 0;
  padding: 14px;
  color: var(--workspace-text);
  text-align: left;
  background: var(--workspace-bg);
  border: 1px solid var(--workspace-line);
  border-radius: 11px;
  box-shadow: none;
  transition:
    border-color var(--duration-fast) var(--ease-out),
    background-color var(--duration-fast) var(--ease-out),
    transform var(--duration-press) var(--ease-out);
}

.appearance-option.selected { background: var(--workspace-accent-soft); border-color: var(--workspace-accent-line); }
.appearance-copy { display: flex; flex-direction: column; gap: 4px; padding-right: 22px; }
.appearance-name { display: inline-flex; align-items: center; gap: 6px; font-size: 13px; font-weight: 600; }
.appearance-description { color: var(--workspace-muted); font-size: 11px; }
.appearance-check { position: absolute; right: 16px; bottom: 27px; color: var(--workspace-accent); font-size: 17px; }
.appearance-note { margin: 18px 0 0; color: var(--workspace-caption); font-size: 11px; line-height: 1.7; }

.theme-preview {
  --preview-bg: #f8f6f0;
  --preview-surface: #fffdf9;
  --preview-line: #ece6dc;
  --preview-accent: #cf654a;
  --preview-soft: #f3e7dd;
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 114px;
  overflow: hidden;
  background: var(--preview-bg);
  border: 1px solid var(--preview-line);
  border-radius: 7px;
}

.dark-preview { --preview-bg: #24221f; --preview-surface: #2e2b27; --preview-line: #433c33; --preview-accent: #ed9b7e; --preview-soft: #443027; }
.preview-top { display: flex; align-items: center; gap: 4px; height: 23px; padding: 0 10px; background: var(--preview-surface); border-bottom: 1px solid var(--preview-line); }
.preview-top i { width: 4px; height: 4px; background: var(--preview-line); border-radius: 50%; }
.preview-top i:first-child { background: var(--preview-accent); }
.preview-body { display: flex; flex: 1; flex-direction: column; gap: 9px; padding: 10px; }
.preview-menu { display: block; width: 45%; height: 9px; background: var(--preview-soft); border-radius: 3px; }
.preview-cards { display: grid; flex: 1; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 8px; }
.preview-cards i { background: var(--preview-surface); border: 1px solid var(--preview-line); border-radius: 4px; }

.settings-footer { display: flex; align-items: center; justify-content: space-between; gap: 16px; margin-top: 20px; padding: 18px 24px; border: 1px solid var(--workspace-line); border-radius: var(--workspace-radius); }
.settings-footer strong { color: var(--workspace-muted); font-size: 12px; font-weight: 500; }
.settings-footer p { margin: 3px 0 0; color: var(--workspace-caption); font-size: 11px; }
.logout-button { display: inline-flex !important; flex-shrink: 0; align-items: center; gap: 6px; height: 34px; padding: 0 12px; color: var(--workspace-accent) !important; font-size: 12px; background: var(--workspace-accent-soft) !important; border-color: var(--workspace-accent-line) !important; border-radius: 8px; box-shadow: none !important; }

.settings-tab:focus-visible,
.appearance-option:focus-visible,
.settings-page :deep(.ant-btn:focus-visible) { outline: 2px solid var(--workspace-accent); outline-offset: 2px; }
.settings-tab:focus:not(:focus-visible),
.appearance-option:focus:not(:focus-visible),
.settings-page :deep(.ant-btn:focus:not(:focus-visible)) { outline: none; }
.settings-page :deep(.ant-spin-dot-item) { background: var(--workspace-accent); }

@media (hover: hover) and (pointer: fine) {
  .settings-tab:hover { color: var(--workspace-accent); background: var(--workspace-soft); box-shadow: none; transform: none; }
  .settings-tab.selected:hover { background: var(--workspace-accent-soft); }
  .settings-page :deep(.ant-btn:hover) { box-shadow: none; transform: none; }
  .save-button:hover { background: var(--workspace-accent-hover) !important; }
  .secondary-button:hover,
  .logout-button:hover { color: var(--workspace-accent-hover) !important; border-color: var(--workspace-accent) !important; }
  .settings-page .bind-field :deep(.ant-input:not(:disabled):hover) { border-color: var(--workspace-accent-line) !important; }
  .appearance-option:hover { background: var(--workspace-surface); border-color: var(--workspace-accent-line); box-shadow: none; transform: none; }
  .appearance-option.selected:hover { background: var(--workspace-accent-soft); }
}

.settings-tab:active,
.appearance-option:active,
.settings-page .save-button:active { transform: scale(0.97); }

@media (max-width: 1000px) {
  .binding-grid { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .bind-card { padding: 18px; }
}

@media (max-width: 768px) {
  .settings-page { min-height: calc(100vh - 57px); padding: 18px var(--page-gutter) 30px; }
  .settings-heading h1 { font-size: 20px; }
  .settings-header { padding-bottom: 11px; }
  .settings-toolbar { margin-bottom: 15px; }
  .settings-tab { padding: 0 12px; font-size: 12px; }
  .settings-content { padding: 21px; }
  .section-header { gap: 12px; margin-bottom: 18px; }
  .binding-grid { grid-template-columns: 1fr; gap: 12px; }
  .bind-card { padding: 18px; }
  .bind-description { min-height: 0; margin-bottom: 15px; }
  .settings-footer { margin-top: 16px; padding: 16px 21px; }
}

@media (max-width: 480px) {
  .settings-page { padding: 16px var(--page-gutter) 24px; }
  .settings-header { gap: 10px; }
  .settings-brand { gap: 10px; }
  .settings-mark { width: 36px; height: 40px; font-size: 22px; border-radius: 9px; }
  .settings-heading h1 { font-size: 19px; }
  .settings-heading p { margin-top: 3px; font-size: 11px; }
  .settings-badge { display: none; }
  .settings-content { padding: 18px 15px; }
  .section-header { flex-wrap: wrap; align-items: flex-start; gap: 12px; }
  .section-header h2 { font-size: 18px; }
  .section-header p { font-size: 11px; }
  .secondary-button { height: 32px; font-size: 11px; }
  .profile-summary { gap: 11px; padding: 14px; margin-bottom: 12px; }
  .profile-copy strong { font-size: 15px; }
  .info-grid { grid-template-columns: 1fr; }
  .info-item { padding: 14px 0; }
  .info-item:nth-last-child(-n + 2) { padding-bottom: 14px; border-bottom: 1px solid var(--workspace-line); }
  .info-item:last-child { padding-bottom: 0; border-bottom: 0; }
  .info-value { font-size: 13px; }
  .bind-card { padding: 16px; }
  .appearance-options { gap: 10px; }
  .appearance-option { gap: 10px; padding: 10px; }
  .theme-preview { height: 94px; }
  .appearance-copy { padding-right: 0; }
  .appearance-name { font-size: 12px; }
  .appearance-description { font-size: 10px; }
  .appearance-check { right: 15px; bottom: 74px; font-size: 16px; }
  .appearance-current { font-size: 10px; }
  .settings-footer { padding: 15px; gap: 10px; }
  .logout-button { padding: 0 9px; font-size: 11px; }
}

@media (prefers-reduced-motion: reduce) {
  .settings-tab:active,
  .appearance-option:active,
  .settings-page .save-button:active { transform: none; }
}
</style>
