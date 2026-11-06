<template>
  <div class="tools-page">
    <header class="page-hero">
      <div class="page-hero-content">
        <span class="page-hero-mark" aria-hidden="true"><AppstoreOutlined /></span>
        <div class="page-hero-copy">
          <h1 class="page-hero-title">在线工具集合<span class="title-dot">.</span></h1>
          <p class="page-hero-desc">常用工具，随手取用</p>
        </div>
      </div>
      <span v-if="!loading" class="tools-count"><AppstoreOutlined /> {{ totalToolCount }} 个工具</span>
    </header>

    <div class="container">
      <a-spin :spinning="loading" tip="加载工具分类中...">
        <div class="tools-layout">
          <!-- 顶部横向分类菜单，与财务页菜单保持一致 -->
          <div class="tools-toolbar">
            <nav class="category-list" aria-label="工具分类">
              <button type="button" class="category-item" :class="{ active: activeCategoryId === null }"
                :aria-pressed="activeCategoryId === null" @click="selectCategory(null)">
                <span class="category-item-icon"><AppstoreOutlined /></span>
                <span class="category-item-name">全部工具</span>
                <span class="category-item-count">{{ totalToolCount }}</span>
              </button>
              <button v-for="category in categories" :key="category.id" type="button" class="category-item"
                :class="{ active: activeCategoryId === category.id }" :aria-pressed="activeCategoryId === category.id"
                @click="selectCategory(category.id)">
                <span class="category-item-icon"><component :is="getCategoryIcon(category.name)" /></span>
                <span class="category-item-name">{{ category.name }}</span>
                <span class="category-item-count">{{ category.tools.length }}</span>
              </button>
            </nav>
          </div>

          <!-- 分类菜单下方的工具内容区 -->
          <main class="tools-content">
            <div class="content-header">
              <h2 class="content-title">{{ currentTitle }}</h2>
              <p class="content-desc">{{ currentDesc }}</p>
            </div>

            <div v-if="displayCategories.length === 0" class="empty-tools">
              <InboxOutlined />
              <span>暂无工具</span>
            </div>

            <div v-for="category in displayCategories" :key="category.id" class="category-section">
              <div class="category-section-header">
                <span class="category-icon"><component :is="getCategoryIcon(category.name)" /></span>
                <h3>{{ category.name }}</h3>
                <span class="category-desc">{{ category.description }}</span>
              </div>
              <div class="tools-grid">
                <div class="tool-item" v-for="(tool, index) in category.tools" :key="tool.id"
                  role="button" tabindex="0" :aria-label="`打开${tool.name}`"
                  :style="{ '--i': index }" @click="openTool(tool)"
                  @keydown.enter="openTool(tool)" @keydown.space.prevent="openTool(tool)">
                  <div class="tool-icon-wrapper">
                    <span class="tool-icon">{{ tool.icon }}</span>
                  </div>
                  <div class="tool-info">
                    <h4 class="tool-name">{{ tool.name }}</h4>
                    <p class="tool-desc">{{ tool.description }}</p>
                  </div>
                  <span class="tool-tag" :class="tool.type === 'frontend' ? 'tag-frontend' : 'tag-backend'">
                    {{ tool.type === 'frontend' ? '前端' : '后端' }}
                  </span>
                </div>
              </div>
            </div>
          </main>
        </div>
      </a-spin>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { ApiService, type Category } from '../services/api'
import { message } from 'ant-design-vue'
import router from '../router'
import { AppstoreOutlined, CodeOutlined, InboxOutlined, PictureOutlined, SettingOutlined } from '@ant-design/icons-vue'

// 工具分类数据
const categories = ref<Category[]>([])
const loading = ref(false)
const activeCategoryId = ref<number | null>(null)

const getCategoryIcon = (name: string) => {
  if (name === '前端工具') return CodeOutlined
  if (name === '后端工具') return SettingOutlined
  if (name === '图片工具') return PictureOutlined
  return AppstoreOutlined
}

// 工具总数
const totalToolCount = computed(() => {
  return categories.value.reduce((sum, category) => sum + category.tools.length, 0)
})

// 当前标题和描述
const currentTitle = computed(() => {
  if (activeCategoryId.value === null) return '全部工具'
  const category = categories.value.find(c => c.id === activeCategoryId.value)
  return category?.name || '全部工具'
})

const currentDesc = computed(() => {
  if (activeCategoryId.value === null) return '浏览所有可用的在线工具'
  const category = categories.value.find(c => c.id === activeCategoryId.value)
  return category?.description || '浏览所有可用的在线工具'
})

// 根据选中分类过滤显示
const displayCategories = computed(() => {
  if (activeCategoryId.value === null) return categories.value
  return categories.value.filter(c => c.id === activeCategoryId.value)
})

// 选择分类
const selectCategory = (id: number | null) => {
  activeCategoryId.value = id
}

// 从API获取工具分类数据
const loadCategories = async () => {
  try {
    loading.value = true
    const response = await ApiService.getCategories()
    categories.value = response.categories
  } catch (error) {
    console.error('加载工具分类失败:', error)
    message.error('加载工具分类失败，请检查后端服务是否启动')

    // 如果API调用失败，使用本地数据作为备用
    categories.value = [
      {
        id: 1,
        name: '前端工具',
        description: '浏览器直接处理，无需后端',
        tools: [
          { id: 1, name: 'JSON格式化', icon: '{}', description: 'JSON数据格式化美化', type: 'frontend' },
          { id: 2, name: 'Base64编码', icon: '64', description: 'Base64编码解码', type: 'frontend' },
          { id: 3, name: 'URL编码', icon: '%', description: 'URL编码解码', type: 'frontend' },
          { id: 10, name: '颜色选择器', icon: '🎨', description: '选择颜色代码', type: 'frontend' },
          { id: 11, name: '时间戳转换', icon: '⏰', description: '时间戳转换工具', type: 'frontend' }
        ]
      },
      {
        id: 2,
        name: '后端工具',
        description: '需要服务器处理的复杂功能',
        tools: [
          { id: 4, name: '文本对比', icon: '≈', description: '对比两个文本的差异', type: 'backend' },
          { id: 9, name: '正则测试', icon: '.*', description: '测试正则表达式', type: 'backend' },
          { id: 12, name: '密码生成器', icon: '🔐', description: '生成安全密码', type: 'backend' }
        ]
      },
      {
        id: 3,
        name: '图片工具',
        description: '图片处理和转换工具（待开发）',
        tools: [
          { id: 5, name: '图片压缩', icon: '📷', description: '压缩图片文件大小', type: 'frontend' },
          { id: 6, name: '格式转换', icon: '🔄', description: '转换图片格式', type: 'frontend' },
          { id: 7, name: '二维码生成', icon: '📱', description: '生成二维码', type: 'frontend' },
          { id: 8, name: '图片水印', icon: '💧', description: '添加图片水印', type: 'backend' }
        ]
      }
    ]
  } finally {
    loading.value = false
  }
}

// 打开工具
const openTool = async (tool: any) => {
  // 前端工具直接跳转，不需要检查后端
  if (tool.type === 'frontend') {
    const routeMap: Record<string, string> = {
      'JSON格式化': '/tools/json',
      'Base64编码': '/tools/base64',
      'URL编码': '/tools/url',
      '颜色选择器': '/tools/color',
      '时间戳转换': '/tools/timestamp'
    }

    const route = routeMap[tool.name]
    if (route) {
      router.push({ path: route })
      return
    }

    message.info(`${tool.name} 工具正在开发中...`)
    return
  }

  // 后端工具需要检查服务，然后跳转到对应页面
  const backendRouteMap: Record<string, string> = {
    '文本对比': '/tools/diff',
    '地图导航': '/tools/map',
    'Sql合理性检查': '/tools/sql'
  }

  const backendRoute = backendRouteMap[tool.name]
  if (!backendRoute) {
    message.info(`${tool.name} 工具正在开发中...`)
    return
  }

  try {
    await ApiService.healthCheck()
    router.push({ path: backendRoute })
  } catch (error) {
    console.error('后端服务不可用:', error)
    message.warning(`后端服务不可用，${tool.name} 工具暂时无法使用`)
  }
}

// 组件挂载时加载数据
onMounted(() => {
  loadCategories()
})
</script>

<style scoped>
.tools-page {
  min-height: calc(100vh - 60px);
  padding: 20px var(--page-gutter) 48px;
  background: var(--workspace-bg);
  color: var(--workspace-text);
}

.page-hero {
  max-width: var(--page-max-width);
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  padding: 0 0 12px;
  border-bottom: 1px solid var(--workspace-line);
}

.page-hero-content {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.page-hero-mark {
  display: grid;
  place-items: center;
  flex-shrink: 0;
  width: 42px;
  height: 46px;
  border-radius: 11px;
  background: var(--workspace-accent-soft);
  color: var(--workspace-accent);
  font-size: 25px;
  transform: rotate(-5deg);
}

.page-hero-copy { min-width: 0; }

.page-hero-title {
  font-size: 22px;
  font-weight: 600;
  line-height: 1.4;
  color: var(--workspace-text);
  margin: 0;
  letter-spacing: 0.5px;
}

.title-dot { color: var(--workspace-accent); margin-left: 3px; }

.page-hero-desc {
  font-size: 12px;
  line-height: 1.5;
  color: var(--workspace-muted);
  margin: 4px 0 0;
  letter-spacing: 0.5px;
}

.tools-count {
  display: inline-flex;
  align-items: center;
  flex-shrink: 0;
  gap: 6px;
  padding: 4px 8px;
  border: 1px solid var(--workspace-line);
  border-radius: 6px;
  color: var(--workspace-muted);
  background: var(--workspace-surface);
  font-size: 11px;
  white-space: nowrap;
}

.tools-page :deep(.ant-spin) { color: var(--workspace-accent); }
.tools-page :deep(.ant-spin-dot-item) { background: var(--workspace-accent); }

.container {
  max-width: var(--page-max-width);
  margin: 0 auto;
  padding: 0;
}

.tools-layout {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

/* 顶部分类菜单：保持单行，窄屏可横向滑动 */
.tools-toolbar {
  width: 100%;
  min-width: 0;
  padding: 9px 0;
  border-bottom: 1px solid var(--workspace-line);
}

.category-list {
  display: flex;
  flex-wrap: nowrap;
  gap: 5px;
  width: 100%;
  padding: 4px;
  overflow-x: auto;
}

.category-item {
  display: flex;
  align-items: center;
  flex: 0 0 auto;
  gap: 8px;
  height: 38px;
  padding: 0 17px;
  border-radius: 8px;
  background: transparent;
  color: var(--workspace-muted);
  cursor: pointer;
  white-space: nowrap;
  box-shadow: none;
  transition: background-color var(--duration-fast) var(--ease-out),
    color var(--duration-fast) var(--ease-out),
    transform var(--duration-press) var(--ease-out);
  border: 0;
}

@media (hover: hover) and (pointer: fine) {
  .category-item:hover {
    background: var(--workspace-soft);
    border-color: var(--workspace-line);
    transform: none;
    box-shadow: none;
  }
}

.category-item:active { transform: scale(0.97); }
.category-item:focus-visible, .tool-item:focus-visible { outline: 2px solid var(--workspace-accent); outline-offset: 3px; }

.category-item.active {
  color: var(--workspace-accent);
  background: var(--workspace-accent-soft);
  border-color: var(--workspace-accent-line);
}

.category-item-icon { font-size: 16px; display: flex; flex-shrink: 0; }

.category-item-name {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--workspace-text);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.category-item.active .category-item-name { color: var(--workspace-accent); font-weight: 600; }

.category-item-count {
  font-size: 10px;
  font-weight: 500;
  color: var(--workspace-caption);
  background: var(--workspace-soft);
  padding: 2px 6px;
  border-radius: 5px;
  min-width: 22px;
  text-align: center;
  font-variant-numeric: tabular-nums;
}

.category-item.active .category-item-count { color: var(--workspace-accent); background: var(--workspace-surface); }

/* 主内容区 */
.tools-content {
  width: 100%;
  min-width: 0;
  background: var(--workspace-surface);
  border: 1px solid var(--workspace-line);
  border-radius: var(--workspace-radius);
  padding: 20px;
}

.content-header {
  margin-bottom: 24px;
  padding-bottom: 20px;
  border-bottom: 1px solid var(--workspace-line);
}

.content-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--workspace-text);
  margin: 0 0 6px;
}

.content-desc { font-size: 12px; color: var(--workspace-muted); margin: 0; }

.empty-tools {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 70px 20px;
  color: var(--workspace-caption);
  font-size: 14px;
  gap: 12px;
}

.empty-tools :deep(.anticon) {
  font-size: 48px;
  opacity: 0.5;
}

.category-section { margin-bottom: 28px; }
.category-section:last-child { margin-bottom: 0; }

.category-section-header {
  display: flex;
  align-items: center;
  gap: 9px;
  margin-bottom: 14px;
  flex-wrap: wrap;
}

.category-section-header .category-icon { display: flex; font-size: 17px; color: var(--workspace-caption); }
.category-section-header h3 { font-size: 15px; font-weight: 600; color: var(--workspace-text); margin: 0; }
.category-section-header .category-desc { font-size: 11px; color: var(--workspace-caption); margin-left: auto; }

.tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(100%, 250px), 1fr));
  gap: 12px;
}

.tool-item {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
  min-height: 106px;
  padding: 17px 15px;
  background: var(--workspace-surface);
  border: 1px solid var(--workspace-line);
  border-radius: 10px;
  cursor: pointer;
  transition: box-shadow var(--duration-base) var(--ease-out),
    transform var(--duration-press) var(--ease-out),
    border-color var(--duration-base) var(--ease-out),
    background-color var(--duration-base) var(--ease-out);
  position: relative;
  animation: toolItemIn var(--duration-base) var(--ease-out) backwards;
  animation-delay: min(calc(var(--i, 0) * 30ms), 150ms);
}

/* 工具卡片入场：30ms stagger（纯装饰，不阻塞交互） */
@keyframes toolItemIn {
  from {
    opacity: 0;
    transform: translateY(8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

@media (hover: hover) and (pointer: fine) {
  .tool-item:hover {
    box-shadow: var(--workspace-shadow-hover);
    border-color: var(--workspace-accent-line);
    background: var(--workspace-soft);
  }
}

.tool-item:active { transform: scale(0.97); }

.tool-icon-wrapper {
  width: 42px;
  height: 42px;
  min-width: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--workspace-accent-soft);
  color: var(--workspace-accent);
  border-radius: 10px;
  font-size: 18px;
  font-weight: 600;
}

.tool-info {
  flex: 1;
  min-width: 0;
}

.tool-name { font-size: 14px; font-weight: 600; color: var(--workspace-text); margin: 0 0 5px; }

.tool-desc {
  font-size: 11px;
  color: var(--workspace-muted);
  margin: 0;
  line-height: 1.7;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.tool-tag {
  align-self: flex-start;
  flex-shrink: 0;
  font-size: 9px;
  font-weight: 500;
  padding: 2px 6px;
  border-radius: 5px;
}

.tag-frontend { background: var(--workspace-green-soft); color: var(--workspace-green); }
.tag-backend { background: var(--workspace-sand-soft); color: var(--workspace-sand); }

/* 减弱动态：入场只保留淡入，取消按压位移 */
@keyframes toolItemFade { from { opacity: 0; } to { opacity: 1; } }
@media (prefers-reduced-motion: reduce) {
  .tool-item { animation-name: toolItemFade; animation-delay: 0ms; }
  .tool-item:active, .category-item:active { transform: none; }
}

@media (max-width: 1000px) {
  .tools-content { padding: 20px; }
  .category-section-header .category-desc { width: 100%; margin-left: 26px; }
}

@media (max-width: 768px) {
  .tools-page { padding: 18px var(--page-gutter) 36px; }
  .page-hero { padding-bottom: 11px; }
  .page-hero-title { font-size: 20px; }
  .tools-count { font-size: 10px; }
  .tools-layout { gap: 15px; }
  .category-item { padding: 0 12px; }
  .category-item-name { font-size: 12px; }
  .tools-content { padding: 21px; }
  .category-section-header .category-desc { margin-left: 26px; }
  .tools-grid { grid-template-columns: repeat(auto-fill, minmax(min(100%, 230px), 1fr)); }
}

@media (max-width: 480px) {
  .tools-page { padding: 16px var(--page-gutter) 30px; }
  .page-hero { gap: 10px; }
  .page-hero-content { gap: 10px; }
  .page-hero-mark { width: 36px; height: 40px; border-radius: 9px; font-size: 22px; }
  .page-hero-title { font-size: 19px; }
  .page-hero-desc { font-size: 11px; margin-top: 3px; }
  .tools-content { padding: 16px 12px; }
  .content-title { font-size: 19px; }
  .tools-grid { grid-template-columns: 1fr; }
  .tool-item { min-height: 95px; }
}
</style>
