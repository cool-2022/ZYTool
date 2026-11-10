<template>
    <div class="tool-page warm-theme">
        <div class="container">
            <div class="page-header has-tool-badge">
                <div>
                    <a-typography-title :level="2">URL 编码工具</a-typography-title>
                    <a-typography-paragraph>编码或解码 URL，保护特殊字符并确保正确传输</a-typography-paragraph>
                </div>
                <span class="tool-badge"><SafetyOutlined /> 本地处理</span>
            </div>

            <div class="tool-action-bar">
                <div class="tool-action-group">
                    <a-button type="primary" @click="encode"><CodeOutlined /> 编码</a-button>
                    <a-button @click="decode"><UnlockOutlined /> 解码</a-button>
                    <a-button @click="loadExample('url')"><FileTextOutlined /> 填入示例</a-button>
                </div>
                <div class="tool-action-group">
                    <a-button :disabled="!result" @click="copyResult"><CopyOutlined /> 复制结果</a-button>
                    <a-button :disabled="!inputText && !result" @click="clearAll"><ClearOutlined /> 清空</a-button>
                </div>
            </div>

            <a-row :gutter="{ xs: 16, sm: 20 }" class="tool-panel-row">
                <a-col :xs="24" :lg="12" class="tool-panel-col">
                    <a-card :bordered="false" class="tool-panel-card text-editor-card">
                        <template #title><span class="tool-card-title"><span class="tool-status-dot"></span>输入 URL</span></template>
                        <template #extra><span class="tool-card-meta">{{ inputText.length }} 字符</span></template>
                        <a-textarea v-model:value="inputText" class="text-tool-input"
                            placeholder="请输入要编码或解码的 URL..." :rows="16" />
                    </a-card>
                </a-col>
                <a-col :xs="24" :lg="12" class="tool-panel-col">
                    <a-card :bordered="false" class="tool-panel-card text-editor-card">
                        <template #title><span class="tool-card-title"><span class="tool-status-dot output"></span>处理结果</span></template>
                        <template #extra><span class="tool-card-meta">{{ result.length }} 字符</span></template>
                        <div class="text-tool-result" :class="{ empty: !result }">
                            <pre v-if="result" class="result-text">{{ result }}</pre>
                            <a-empty v-else description="编码或解码后，结果会显示在这里" />
                        </div>
                    </a-card>
                </a-col>
            </a-row>

            <a-card title="使用说明" :bordered="false" class="tool-help-card">
                <a-descriptions :column="1">
                    <a-descriptions-item label="什么是 URL 编码">将空格、中文和特殊符号转换为百分号编码，确保 URL 正确传输。</a-descriptions-item>
                    <a-descriptions-item label="使用场景">URL 参数传递、搜索查询、API 接口调用和超链接构建。</a-descriptions-item>
                </a-descriptions>
            </a-card>
            <div class="tool-page-note"><SafetyOutlined /> 输入内容不会上传到服务器</div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { SafetyOutlined, CodeOutlined, UnlockOutlined, FileTextOutlined, CopyOutlined, ClearOutlined } from '@ant-design/icons-vue'
import { urlEncode, urlDecode } from '../../services/frontendTools'

const inputText = ref('')
const result = ref('')

// 编码
const encode = () => {
    if (!inputText.value.trim()) {
        message.warning('请输入要编码的URL')
        return
    }

    const res = urlEncode(inputText.value)
    if (res.success) {
        result.value = res.result
        message.success('编码成功')
    } else {
        message.error('编码失败: ' + res.result)
        result.value = ''
    }
}

// 解码
const decode = () => {
    if (!inputText.value.trim()) {
        message.warning('请输入要解码的URL')
        return
    }

    const res = urlDecode(inputText.value)
    if (res.success) {
        result.value = res.result
        message.success('解码成功')
    } else {
        message.error('解码失败: ' + res.result)
        result.value = ''
    }
}

// 复制结果
const copyResult = () => {
    if (!result.value) {
        message.warning('暂无可复制的结果')
        return
    }

    navigator.clipboard.writeText(result.value)
        .then(() => message.success('复制成功'))
        .catch(() => message.error('复制失败'))
}

// 清空
const clearAll = () => {
    inputText.value = ''
    result.value = ''
}

// 加载示例
const loadExample = (type: string) => {
    switch (type) {
        case 'url':
            inputText.value = 'https://example.com/search?q=前端开发&type=tutorial&lang=zh-CN'
            break
        case 'chinese':
            inputText.value = 'https://www.example.com/文章/技术分享.html'
            break
        case 'encoded':
            inputText.value = 'https%3A%2F%2Fexample.com%2Fsearch%3Fq%3D%E5%89%8D%E7%AB%AF%E5%BC%80%E5%8F%91'
            break
    }
    result.value = ''
}
</script>

<style scoped>
.tool-page {
    min-height: 100vh;
    background: var(--gradient-bg);
    padding: 2rem var(--page-gutter);
}

.container {
    max-width: var(--page-max-width);
    margin: 0 auto;
    padding: 0;
}

.page-header {
    text-align: center;
    margin-bottom: 2rem;
    animation: fadeInDown var(--duration-enter) var(--ease-out);
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
    transition: box-shadow var(--duration-base) var(--ease-out), border-color var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out), color var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out);
    border: 1px solid var(--border-color);
    background: var(--bg-primary);
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

.result-container {
    min-height: 300px;
    background: var(--bg-secondary);
    border-radius: var(--border-radius);
    padding: 1rem;
    border: 2px solid var(--border-color);
}

.result-text {
    margin: 0;
    font-family: 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.6;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-all;
}

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
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
