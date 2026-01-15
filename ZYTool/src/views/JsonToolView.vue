<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">JSON格式化工具（前端处理）</a-typography-title>
                <a-typography-paragraph>输入JSON文本，浏览器本地处理，无需后端</a-typography-paragraph>
            </div>

            <!-- 工具内容 -->
            <a-row :gutter="24">
                <a-col :xs="24" :lg="12">
                    <a-card title="输入JSON" :bordered="false">
                        <a-textarea v-model:value="inputText" placeholder="请输入JSON文本..." :rows="15"
                            @change="handleInputChange" />
                    </a-card>
                </a-col>
                <a-col :xs="24" :lg="12">
                    <a-card title="格式化结果" :bordered="false">
                        <div class="result-container">
                            <a-spin :spinning="loading" tip="格式化中...">
                                <pre v-if="formattedResult" class="formatted-json">{{ formattedResult }}</pre>
                                <a-empty v-else description="暂无结果" />
                            </a-spin>
                        </div>
                        <template #extra>
                            <a-space>
                                <a-button @click="formatJson" type="primary" :loading="loading">
                                    格式化
                                </a-button>
                                <a-button @click="compressJson" :loading="loading">
                                    压缩
                                </a-button>
                                <a-button @click="copyResult">
                                    复制
                                </a-button>
                                <a-button @click="clearAll">清空</a-button>
                            </a-space>
                        </template>
                    </a-card>
                </a-col>
            </a-row>

            <!-- 其他文本工具 -->
            <a-divider>其他前端工具</a-divider>
            <a-row :gutter="16">
                <a-col :xs="24" :sm="12" :md="8" :lg="6">
                    <a-card hoverable @click="openTool('base64')">
                        <a-space direction="vertical" align="center" style="width:100%">
                            <div class="tool-icon">64</div>
                            <a-typography-title :level="5" style="margin:0">Base64编码</a-typography-title>
                            <a-typography-paragraph
                                style="margin:0;color:var(--primary-color)">前端处理</a-typography-paragraph>
                        </a-space>
                    </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="8" :lg="6">
                    <a-card hoverable @click="openTool('url')">
                        <a-space direction="vertical" align="center" style="width:100%">
                            <div class="tool-icon">%</div>
                            <a-typography-title :level="5" style="margin:0">URL编码</a-typography-title>
                            <a-typography-paragraph
                                style="margin:0;color:var(--primary-color)">前端处理</a-typography-paragraph>
                        </a-space>
                    </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="8" :lg="6">
                    <a-card hoverable @click="openTool('regex')">
                        <a-space direction="vertical" align="center" style="width:100%">
                            <div class="tool-icon">.*</div>
                            <a-typography-title :level="5" style="margin:0">正则测试</a-typography-title>
                            <a-typography-paragraph style="margin:0;color:orange">后端处理</a-typography-paragraph>
                        </a-space>
                    </a-card>
                </a-col>
                <a-col :xs="24" :sm="12" :md="8" :lg="6">
                    <a-card hoverable @click="openTool('password')">
                        <a-space direction="vertical" align="center" style="width:100%">
                            <div class="tool-icon">🔐</div>
                            <a-typography-title :level="5" style="margin:0">密码生成</a-typography-title>
                            <a-typography-paragraph style="margin:0;color:orange">后端处理</a-typography-paragraph>
                        </a-space>
                    </a-card>
                </a-col>
            </a-row>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { formatJSON, minifyJSON } from '../services/frontendTools'

// 响应式数据
const inputText = ref('')
const formattedResult = ref('')
const loading = ref(false)

// 格式化JSON（前端处理）
const formatJson = () => {
    if (!inputText.value.trim()) {
        message.warning('请输入JSON文本')
        return
    }

    loading.value = true

    // 使用延迟模拟处理过程
    setTimeout(() => {
        const result = formatJSON(inputText.value)

        if (result.success) {
            formattedResult.value = result.result
            message.success('JSON格式化成功（前端处理）')
        } else {
            message.error('JSON格式错误: ' + result.result)
            formattedResult.value = ''
        }

        loading.value = false
    }, 300)
}

// 压缩JSON
const compressJson = () => {
    if (!inputText.value.trim()) {
        message.warning('请输入JSON文本')
        return
    }

    loading.value = true

    setTimeout(() => {
        const result = minifyJSON(inputText.value)

        if (result.success) {
            formattedResult.value = result.result
            message.success('JSON压缩成功（前端处理）')
        } else {
            message.error('JSON格式错误: ' + result.result)
            formattedResult.value = ''
        }

        loading.value = false
    }, 300)
}

// 处理输入变化
const handleInputChange = () => {
    // 可以添加实时验证逻辑
}

// 清空所有内容
const clearAll = () => {
    inputText.value = ''
    formattedResult.value = ''
}

// 复制结果
const copyResult = () => {
    if (!formattedResult.value) {
        message.warning('暂无可复制的结果')
        return
    }

    navigator.clipboard.writeText(formattedResult.value)
        .then(() => {
            message.success('复制成功')
        })
        .catch(() => {
            message.error('复制失败')
        })
}

// 打开其他工具
const openTool = (toolType: string) => {
    message.info(`正在打开${toolType}工具...`)
    // 这里可以添加路由跳转或打开模态框
}

// 示例JSON数据
const exampleJson = `{
  "name": "ZYTool",
  "version": "1.0.0",
  "features": [
    "JSON格式化",
    "Base64编码",
    "URL编码",
    "正则测试"
  ],
  "config": {
    "theme": "dark",
    "language": "zh-CN"
  }
}`

// 设置示例数据
inputText.value = exampleJson
</script>

<style scoped>
.tool-page {
    min-height: 100vh;
    background: var(--gradient-bg);
    padding: 2rem 0;
}

.container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0 20px;
}

.page-header {
    text-align: center;
    margin-bottom: 2rem;
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

.tool-page :deep(.ant-card) {
    border-radius: var(--border-radius-lg);
    box-shadow: var(--shadow-md);
    transition: all var(--transition-speed);
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
}

.tool-page :deep(.ant-card:hover) {
    box-shadow: var(--shadow-lg);
}

.tool-page :deep(.ant-card-head) {
    background: var(--gradient-primary);
    color: white;
    border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.tool-page :deep(.ant-card-head-title) {
    color: white;
    font-weight: 600;
}

.tool-page :deep(.ant-textarea) {
    border-radius: 8px;
    border: 2px solid #e8e8e8;
    transition: all 0.3s ease;
    font-family: 'Courier New', monospace;
}

.tool-page :deep(.ant-textarea:focus) {
    border-color: var(--primary-color);
    box-shadow: 0 0 0 2px rgba(99, 210, 210, 0.2);
}

.result-container {
    min-height: 300px;
    background: var(--bg-secondary);
    border-radius: var(--border-radius);
    padding: 1rem;
    border: 2px solid var(--border-color);
}

.formatted-json {
    margin: 0;
    font-family: 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-all;
    background: var(--bg-primary);
    padding: 1rem;
    border-radius: 6px;
}

.tool-page :deep(.ant-divider) {
    margin: 3rem 0;
    border-color: #d0d0d0;
}

.tool-page :deep(.ant-divider-inner-text) {
    font-size: 1.1rem;
    font-weight: 600;
    color: var(--primary-dark);
}

.tool-page :deep(.ant-card-hoverable:hover) {
    transform: translateY(-8px) scale(1.02);
    box-shadow: var(--shadow-primary);
    border-color: var(--primary-color);
}

.tool-icon {
    font-size: 2.5rem;
    margin-bottom: 0.5rem;
    background: var(--gradient-primary);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
    font-weight: bold;
    transition: all var(--transition-speed);
}

.tool-page :deep(.ant-card-hoverable:hover) .tool-icon {
    transform: scale(1.2) rotate(5deg);
}

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
    border-radius: var(--border-radius);
    font-weight: 600;
    transition: all var(--transition-speed);
}

.tool-page :deep(.ant-btn-primary:hover) {
    transform: translateY(-2px);
    box-shadow: var(--shadow-primary-hover);
}

.tool-page :deep(.ant-btn:not(.ant-btn-primary)) {
    border-radius: var(--border-radius);
    border: 2px solid var(--primary-color);
    color: var(--primary-dark);
    font-weight: 600;
    transition: all var(--transition-speed);
}

.tool-page :deep(.ant-btn:not(.ant-btn-primary):hover) {
    background: var(--primary-color);
    color: white;
    transform: translateY(-2px);
}

@media (max-width: 768px) {
    .tool-page {
        padding: 1rem 0;
    }

    .container {
        padding: 0 10px;
    }

    .tool-icon {
        font-size: 2rem;
    }
}

.tool-page :deep(.ant-descriptions-item-label) {
    color: var(--text-primary);
    background: var(--bg-secondary);
}

.tool-page :deep(.ant-descriptions-item-content) {
    color: var(--text-primary);
    background: var(--bg-primary);
}

.tool-page :deep(.ant-input) {
    background: var(--bg-primary);
    border-color: var(--border-color);
    color: var(--text-primary);
}

.tool-page :deep(.ant-input::placeholder) {
    color: var(--text-secondary);
}

.tool-page :deep(.ant-select-selector) {
    background: var(--bg-primary) !important;
    border-color: var(--border-color) !important;
    color: var(--text-primary) !important;
}

.tool-page :deep(.ant-picker) {
    background: var(--bg-primary);
    border-color: var(--border-color);
}

.tool-page :deep(.ant-picker-input > input) {
    color: var(--text-primary);
}

.tool-page :deep(.ant-divider-inner-text) {
    color: var(--text-primary);
}

.timestamp-value {
    font-family: 'Courier New', monospace;
    font-weight: 600;
    font-size: 16px;
    color: var(--primary-color);
}

.result-box {
    background: var(--bg-secondary);
    padding: 1rem;
    border-radius: var(--border-radius);
    border: 2px solid var(--border-color);
}

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
}

label {
    display: block;
    margin-bottom: 8px;
    font-weight: 600;
    color: var(--text-primary);
}
</style>
