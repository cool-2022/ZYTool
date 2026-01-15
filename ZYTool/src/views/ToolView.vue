<template>
    <div class="tools-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">在线工具集合</a-typography-title>
                <a-typography-paragraph>选择您需要的工具开始使用</a-typography-paragraph>
            </div>

            <!-- 工具分类（每个分类一张卡片，内部用栅格展示工具） -->
            <div class="tools-categories">
                <a-spin :spinning="loading" tip="加载工具分类中...">
                    <a-card class="category-card" v-for="category in categories" :key="category.id"
                        :title="category.name" :bordered="false">
                        <template #extra>
                            <a-typography-text type="secondary">{{ category.description }}</a-typography-text>
                        </template>
                        <a-row :gutter="[16, 16]" class="tools-grid">
                            <a-col :xs="24" :sm="12" :md="12" :lg="8" :xl="6" v-for="tool in category.tools"
                                :key="tool.id">
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

</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { ApiService, type Category } from '../services/api'
import { message } from 'ant-design-vue'
import router from '../router'

// 工具分类数据
const categories = ref<Category[]>([])
const loading = ref(false)

// 从API获取工具分类数据
const loadCategories = async () => {
    try {
        loading.value = true
        const response = await ApiService.getCategories()
        categories.value = response.categories
        message.success('工具分类加载成功')
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
        // 根据工具名称跳转到对应页面
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
    background: var(--gradient-bg);
    padding: 2rem 0;
}

.container {
    max-width: 1600px;
    margin: 0 auto;
    padding: 0 20px;
}

.page-header {
    text-align: center;
    margin-bottom: 3rem;
    animation: fadeInDown 0.6s ease-out;
}

.page-header :deep(.ant-typography) {
    color: var(--text-primary);
}

@keyframes fadeInDown {
    from {
        opacity: 0;
        transform: translateY(-20px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
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
    .tools-page {
        padding: 1rem 0;
    }

    .category-card {
        border-radius: 12px;
    }

    .tool-item {
        margin-bottom: 1rem;
    }
}
</style>