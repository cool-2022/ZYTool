<template>
  <div class="tools-page">
    <div class="page-hero">
      <div class="page-hero-content">
        <h1 class="page-hero-title">在线工具集合</h1>
        <p class="page-hero-desc">选择你需要的工具，快速完成日常任务</p>
      </div>
    </div>

    <div class="container">
      <a-spin :spinning="loading" tip="加载工具分类中...">
        <div class="tools-categories">
          <div class="category-card" v-for="category in categories" :key="category.id">
            <div class="category-header">
              <div class="category-title">
                <span class="category-icon">{{ getCategoryIcon(category.name) }}</span>
                <h3>{{ category.name }}</h3>
              </div>
              <span class="category-desc">{{ category.description }}</span>
            </div>
            <div class="tools-grid">
              <div class="tool-item" v-for="tool in category.tools" :key="tool.id" @click="openTool(tool)">
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
        </div>
      </a-spin>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ApiService, type Category } from '../services/api'
import { message } from 'ant-design-vue'
import router from '../router'

// 工具分类数据
const categories = ref<Category[]>([])
const loading = ref(false)

const getCategoryIcon = (name: string) => {
  const map: Record<string, string> = {
    '前端工具': '🎨',
    '后端工具': '⚙️',
    '图片工具': '🖼️'
  }
  return map[name] || '📦'
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

  // 后端工具需要检查服务
  try {
    await ApiService.healthCheck()
    message.info(`正在打开 ${tool.name} 工具...`)
    console.log('打开工具:', tool)
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
  min-height: 100vh;
  padding-bottom: 100px;
}

.page-hero {
  background: var(--gradient-hero);
  padding: 70px 24px 90px;
  text-align: center;
  position: relative;
  overflow: hidden;
}

.page-hero::before {
  content: '';
  position: absolute;
  top: -50%;
  left: -20%;
  width: 60%;
  height: 200%;
  background: radial-gradient(circle, rgba(59, 130, 246, 0.15) 0%, transparent 70%);
  pointer-events: none;
}

.page-hero::after {
  content: '';
  position: absolute;
  bottom: -50%;
  right: -20%;
  width: 60%;
  height: 200%;
  background: radial-gradient(circle, rgba(139, 92, 246, 0.15) 0%, transparent 70%);
  pointer-events: none;
}

.page-hero-content {
  position: relative;
  z-index: 1;
}

.page-hero-title {
  font-size: 2.75rem;
  font-weight: 800;
  color: white;
  margin: 0 0 0.75rem;
  letter-spacing: -1px;
}

.page-hero-desc {
  font-size: 1.15rem;
  color: rgba(255, 255, 255, 0.7);
  margin: 0;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 24px;
  margin-top: -40px;
  position: relative;
  z-index: 2;
}

.tools-categories {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.category-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-xl);
  padding: 1.75rem;
  box-shadow: var(--shadow-lg);
}

.category-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.category-title {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.category-icon {
  font-size: 1.5rem;
}

.category-title h3 {
  font-size: 1.35rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
}

.category-desc {
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.tools-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
  gap: 1rem;
}

.tool-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1.25rem;
  background: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-lg);
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.tool-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  width: 3px;
  height: 100%;
  background: var(--gradient-primary);
  opacity: 0;
  transition: opacity 0.3s ease;
}

.tool-item:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow-md);
  border-color: var(--primary-light);
  background: var(--bg-primary);
}

.tool-item:hover::before {
  opacity: 1;
}

.tool-icon-wrapper {
  width: 52px;
  height: 52px;
  min-width: 52px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--gradient-primary);
  border-radius: var(--border-radius);
  font-size: 1.5rem;
  box-shadow: var(--shadow-primary);
  transition: transform 0.3s ease;
}

.tool-item:hover .tool-icon-wrapper {
  transform: scale(1.1) rotate(4deg);
}

.tool-info {
  flex: 1;
  min-width: 0;
}

.tool-name {
  font-size: 1.05rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem;
}

.tool-desc {
  font-size: 0.85rem;
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.tool-tag {
  font-size: 0.7rem;
  font-weight: 600;
  padding: 0.25rem 0.6rem;
  border-radius: 50px;
  text-transform: uppercase;
}

.tag-frontend {
  background: rgba(59, 130, 246, 0.12);
  color: var(--primary-dark);
}

.tag-backend {
  background: rgba(139, 92, 246, 0.12);
  color: var(--accent-color);
}

@media (max-width: 768px) {
  .page-hero {
    padding: 50px 20px 70px;
  }

  .page-hero-title {
    font-size: 2rem;
  }

  .container {
    padding: 0 20px;
    margin-top: -30px;
  }

  .category-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .tools-grid {
    grid-template-columns: 1fr;
  }
}
</style>
