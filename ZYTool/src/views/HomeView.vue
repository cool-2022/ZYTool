<template>
  <div class="home">
    <!-- Hero 区域 -->
    <section class="hero-section">
      <div class="hero-grid">
        <div class="hero-glow hero-glow-1"></div>
        <div class="hero-glow hero-glow-2"></div>
      </div>
      <div class="hero-content">
        <div class="hero-badge">
          <span class="badge-dot"></span>
          一站式在线工具平台
        </div>
        <h1 class="hero-title">
          <span class="title-main">ZYTool</span>
          <span class="title-sub">效率工具集合</span>
        </h1>
        <p class="hero-subtitle">
          简洁、强大、开箱即用的在线工具箱，让开发和工作更高效
        </p>
        <div class="hero-actions">
          <router-link to="/tools" class="btn btn-primary">
            开始使用
            <span class="btn-icon">→</span>
          </router-link>
          <router-link to="/agents/chat" class="btn btn-secondary">
            AI 助手
          </router-link>
        </div>
      </div>
    </section>

    <!-- 工具列表 -->
    <section class="tools-section">
      <div class="container">
        <div class="section-header">
          <h2 class="section-title">工具分类</h2>
          <p class="section-desc">选择你需要的工具，快速完成日常任务</p>
        </div>

        <a-spin :spinning="loading" tip="加载工具中...">
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
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { ApiService, type Category } from '../services/api'

const router = useRouter()
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
          { id: 12, name: '密码生成器', icon: '🔐', description: '生成安全密码', type: 'backend' },
          { id: 13, name: '地图导航', icon: '🗺', description: '显示当前位置地图', type: 'backend' },
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
  const routeMap: Record<string, string> = {
    'JSON格式化': '/tools/json',
    'Base64编码': '/tools/base64',
    'URL编码': '/tools/url',
    '颜色选择器': '/tools/color',
    '时间戳转换': '/tools/timestamp',
    '文本对比': '/tools/diff',
    'Sql合理性检查': '/tools/sql',
    '地图导航': '/tools/map',
  }

  const route = routeMap[tool.name]
  if (route) {
    router.push({ path: route })
    return
  }

  message.info(`${tool.name} 工具正在开发中...`)
}

onMounted(() => {
  loadCategories()
})
</script>

<style scoped>
.home {
  width: 100%;
  min-height: 100vh;
}

/* Hero 区域 */
.hero-section {
  position: relative;
  padding: 100px 24px 120px;
  background: var(--gradient-hero);
  overflow: hidden;
  text-align: center;
}

.hero-grid {
  position: absolute;
  inset: 0;
  pointer-events: none;
}

.hero-glow {
  position: absolute;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.35;
}

.hero-glow-1 {
  width: 500px;
  height: 500px;
  top: -150px;
  right: 10%;
  background: var(--primary-color);
  animation: float 8s ease-in-out infinite;
}

.hero-glow-2 {
  width: 400px;
  height: 400px;
  bottom: -100px;
  left: 5%;
  background: var(--accent-color);
  animation: float 10s ease-in-out infinite reverse;
}

.hero-content {
  position: relative;
  z-index: 2;
  max-width: 800px;
  margin: 0 auto;
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--gradient-glass);
  border: 1px solid rgba(255, 255, 255, 0.15);
  border-radius: 50px;
  color: rgba(255, 255, 255, 0.9);
  font-size: 0.9rem;
  font-weight: 500;
  margin-bottom: 1.5rem;
  backdrop-filter: blur(10px);
}

.badge-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #22c55e;
  box-shadow: 0 0 8px #22c55e;
}

.hero-title {
  margin: 0 0 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.title-main {
  font-size: 4.5rem;
  font-weight: 900;
  background: linear-gradient(135deg, #ffffff 0%, #bfdbfe 50%, #ddd6fe 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  letter-spacing: -2px;
  line-height: 1.1;
}

.title-sub {
  font-size: 1.75rem;
  font-weight: 500;
  color: rgba(255, 255, 255, 0.75);
  letter-spacing: 4px;
}

.hero-subtitle {
  font-size: 1.25rem;
  color: rgba(255, 255, 255, 0.65);
  margin: 0 auto 2rem;
  max-width: 560px;
  line-height: 1.7;
}

.hero-actions {
  display: flex;
  justify-content: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.85rem 1.75rem;
  border-radius: var(--border-radius);
  font-weight: 600;
  font-size: 1rem;
  text-decoration: none;
  transition: all 0.3s ease;
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--gradient-primary);
  color: white;
  box-shadow: 0 4px 16px rgba(59, 130, 246, 0.4);
}

.btn-primary:hover {
  background: var(--gradient-primary-hover);
  transform: translateY(-2px);
  box-shadow: 0 6px 24px rgba(59, 130, 246, 0.5);
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.2);
  backdrop-filter: blur(10px);
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.18);
  transform: translateY(-2px);
}

.btn-icon {
  transition: transform 0.3s ease;
}

.btn-primary:hover .btn-icon {
  transform: translateX(3px);
}

/* 工具列表区域 */
.tools-section {
  padding: 80px 24px 120px;
}

.container {
  max-width: 1400px;
  margin: 0 auto;
}

.section-header {
  text-align: center;
  margin-bottom: 3.5rem;
}

.section-title {
  font-size: 2.25rem;
  font-weight: 800;
  color: var(--text-primary);
  margin-bottom: 0.75rem;
}

.section-desc {
  font-size: 1.1rem;
  color: var(--text-secondary);
}

.tools-categories {
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
}

.category-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius-xl);
  padding: 1.75rem;
  box-shadow: var(--shadow-md);
  transition: all var(--transition-speed);
}

.category-card:hover {
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
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
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

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-20px) scale(1.05);
  }
}

@media (max-width: 768px) {
  .hero-section {
    padding: 70px 20px 90px;
  }

  .title-main {
    font-size: 3rem;
  }

  .title-sub {
    font-size: 1.25rem;
    letter-spacing: 2px;
  }

  .hero-subtitle {
    font-size: 1.05rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
  }

  .tools-section {
    padding: 50px 20px 80px;
  }

  .section-title {
    font-size: 1.75rem;
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
