<template>
    <a-layout class="home">
        <a-layout-content>
            <div class="hero-section">
                <div class="hero-content">
                    <a-typography-title :level="1" class="hero-title">
                        <span class="title-text">ZYTool</span>
                    </a-typography-title>
                    <a-typography-paragraph class="hero-subtitle">
                        一站式在线工具集合，提升您的工作效率
                    </a-typography-paragraph>
                </div>
                <div class="hero-decoration">
                    <div class="circle circle-1"></div>
                    <div class="circle circle-2"></div>
                    <div class="circle circle-3"></div>
                </div>
            </div>

            <!-- 工具列表 -->
            <div class="tools-section">
                <div class="container">
                    <div class="tools-categories">
                        <a-spin :spinning="loading" tip="加载工具中...">
                            <a-card class="category-card" v-for="category in categories" :key="category.id"
                                :title="category.name" :bordered="false">
                                <template #extra>
                                    <a-typography-text type="secondary">{{ category.description }}</a-typography-text>
                                </template>
                                <a-row :gutter="[12, 12]" class="tools-grid">
                                    <a-col class="tool-col" :xs="24" :sm="12" :md="8" :lg="6" :xl="4"
                                        v-for="tool in category.tools" :key="tool.id">
                                        <a-card hoverable class="tool-item" @click="openTool(tool)">
                                            <a-space direction="vertical" align="center" style="width:100%">
                                                <div class="tool-icon">{{ tool.icon }}</div>
                                                <a-typography-title :level="4" style="margin:0">{{ tool.name
                                                }}</a-typography-title>
                                                <a-typography-paragraph style="margin:0">{{ tool.description
                                                    }}</a-typography-paragraph>
                                                <a-tag v-if="tool.type === 'frontend'" color="cyan">前端处理</a-tag>
                                                <a-tag v-else-if="tool.type === 'backend'" color="orange">后端处理</a-tag>
                                            </a-space>
                                        </a-card>
                                    </a-col>
                                </a-row>
                            </a-card>
                        </a-spin>
                    </div>
                </div>
            </div>
        </a-layout-content>
    </a-layout>

</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { message } from 'ant-design-vue'
import { ApiService, type Category } from '../services/api'

const router = useRouter()
const categories = ref<Category[]>([])
const loading = ref(false)

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

    // 前端工具直接跳转，不需要检查后端
    if (tool.type === 'frontend' || tool.type === 'backend') {


        const route = routeMap[tool.name]
        if (route) {
            router.push({ path: route })
            return
        }

        message.info(`${tool.name} 工具正在开发中...`)
        return
    }

    // by zzy 临时注释 目前先用mock数据进行调试

    // 后端工具需要检查服务
    // try {
    //     await ApiService.healthCheck()
    //     message.info(`正在打开 ${tool.name} 工具...`)
    // } catch (error) {
    //     message.warning(`后端服务不可用，${tool.name} 工具暂时无法使用`)
    // }
}

onMounted(() => {
    loadCategories()
})
</script>

<style scoped>
.home {
    width: 100%;
    background: linear-gradient(135deg, #63d2d2 0%, #63d2d2 100%);
    color: white;
}

@media (prefers-color-scheme: dark) {
    .home {
        background: linear-gradient(135deg, #4fb8b8 0%, #3a9e9e 100%);
    }
}

.hero-section {
    text-align: center;
    padding: 80px 20px 60px;
    position: relative;
    overflow: hidden;
}

.hero-content {
    position: relative;
    z-index: 2;
}

.hero-title {
    animation: fadeInUp 0.8s ease-out;
}

.title-text {
    font-size: 4rem;
    font-weight: 800;
    background: linear-gradient(120deg, #fff, #f0f0f0);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    text-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
}

.hero-subtitle {
    font-size: 1.5rem;
    margin-bottom: 0;
    opacity: 0.95;
    animation: fadeInUp 0.8s ease-out 0.2s backwards;
    color: white !important;
}

.hero-decoration {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 1;
    pointer-events: none;
}

.circle {
    position: absolute;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.1);
    animation: float 6s ease-in-out infinite;
}

.circle-1 {
    width: 300px;
    height: 300px;
    top: -100px;
    right: -100px;
    animation-delay: 0s;
}

.circle-2 {
    width: 200px;
    height: 200px;
    bottom: -50px;
    left: -50px;
    animation-delay: 2s;
}

.circle-3 {
    width: 150px;
    height: 150px;
    top: 50%;
    left: 10%;
    animation-delay: 4s;
}

@keyframes float {

    0%,
    100% {
        transform: translateY(0) rotate(0deg);
    }

    50% {
        transform: translateY(-20px) rotate(180deg);
    }
}

@keyframes fadeInUp {
    from {
        opacity: 0;
        transform: translateY(30px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* 工具列表区域 */
.tools-section {
    padding: 40px 20px 80px;
    background: var(--bg-primary);
}

.container {
    max-width: 1600px;
    margin: 0 auto;
}

.tools-categories {
    display: flex;
    flex-direction: column;
    gap: 2rem;
}

.category-card {
    border-radius: var(--border-radius-xl);
    box-shadow: var(--shadow-md);
    transition: all var(--transition-speed);
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
}

.category-card:hover {
    box-shadow: var(--shadow-lg);
    transform: translateY(-4px);
}

.category-card :deep(.ant-card-head) {
    background: var(--gradient-primary);
    color: white;
    border-radius: var(--border-radius-xl) var(--border-radius-xl) 0 0;
}

.category-card :deep(.ant-card-head-title) {
    color: white;
    font-weight: 600;
    font-size: 1.2rem;
}

.category-card :deep(.ant-card-extra) {
    color: rgba(255, 255, 255, 0.9);
}

.tools-grid {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    margin: 0 -6px;
}

.tool-col {
    box-sizing: border-box;
    padding: 6px;
    flex: 0 0 100%;
    max-width: 100%;
}

/* 小屏：1 列 */
@media (min-width: 576px) {
    .tool-col {
        flex: 0 0 50% !important;
        max-width: 50% !important;
    }
}

/* 中等屏：3 列 */
@media (min-width: 768px) {
    .tool-col {
        flex: 0 0 33.3333% !important;
        max-width: 33.3333% !important;
    }
}

/* 大屏：5 列 */
@media (min-width: 992px) {
    .tool-col {
        flex: 0 0 20% !important;
        max-width: 20% !important;
    }
}

.tool-item {
    text-align: center;
    cursor: pointer;
    border-radius: var(--border-radius-lg);
    transition: all var(--transition-speed);
    border: 2px solid transparent;
    height: 100%;
    background: var(--bg-primary);
}

.tool-item:hover {
    transform: translateY(-8px) scale(1.02);
    box-shadow: var(--shadow-primary-hover);
    border-color: var(--primary-color);
}

.tool-item:active {
    transform: translateY(-4px) scale(1.01);
}

.tool-icon {
    font-size: 2.5rem;
    margin-bottom: 1rem;
    background: var(--gradient-primary);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    transition: all var(--transition-speed);
}

.tool-item:hover .tool-icon {
    transform: scale(1.2) rotate(5deg);
}

.tool-item :deep(.ant-typography-title) {
    color: var(--text-primary);
    font-weight: 600;
    transition: color 0.3s ease;
}

.tool-item:hover :deep(.ant-typography-title) {
    color: var(--primary-dark);
}

.tool-item :deep(.ant-typography-paragraph) {
    color: var(--text-secondary);
    font-size: 0.9rem;
}

@media (max-width: 768px) {
    .hero-section {
        padding: 60px 20px 40px;
    }

    .title-text {
        font-size: 2.5rem;
    }

    .hero-subtitle {
        font-size: 1.2rem;
    }

    .circle-1,
    .circle-2,
    .circle-3 {
        display: none;
    }

    .tools-section {
        padding: 20px 10px 60px;
    }

    .category-card {
        border-radius: var(--border-radius-lg);
    }
}
</style>
