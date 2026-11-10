<template>
    <div class="tool-page warm-theme json-tool-page">
        <div class="container">
            <div class="page-header json-page-header">
                <div>
                    <a-typography-title :level="2">JSON 格式化工具</a-typography-title>
                    <a-typography-paragraph>格式化、压缩和校验 JSON，数据仅在浏览器中处理</a-typography-paragraph>
                </div>
                <span class="local-badge"><SafetyOutlined /> 本地处理</span>
            </div>

            <div class="json-toolbar">
                <div class="toolbar-primary">
                    <a-button type="primary" :loading="loading" @click="formatJson">
                        <AlignLeftOutlined /> 格式化
                    </a-button>
                    <a-button :loading="loading" @click="compressJson">
                        <CompressOutlined /> 压缩
                    </a-button>
                    <a-button @click="loadExample">
                        <FileTextOutlined /> 填入示例
                    </a-button>
                </div>
                <div class="toolbar-secondary">
                    <a-button :disabled="!formattedResult" @click="copyResult">
                        <CopyOutlined /> 复制结果
                    </a-button>
                    <a-button :disabled="!inputText && !formattedResult" @click="clearAll">
                        <ClearOutlined /> 清空
                    </a-button>
                </div>
            </div>

            <a-row :gutter="{ xs: 16, sm: 20 }" class="json-workspace">
                <a-col :xs="24" :lg="12" class="json-editor-col">
                    <a-card :bordered="false" class="json-editor-card">
                        <template #title>
                            <span class="editor-title"><span class="editor-dot input-dot"></span>输入 JSON</span>
                        </template>
                        <template #extra>
                            <span class="character-count">{{ inputText.length }} 字符</span>
                        </template>
                        <a-textarea v-model:value="inputText" class="json-input"
                            placeholder="请输入 JSON 文本，或点击“填入示例”..." :rows="18" />
                    </a-card>
                </a-col>

                <a-col :xs="24" :lg="12" class="json-editor-col">
                    <a-card :bordered="false" class="json-editor-card">
                        <template #title>
                            <span class="editor-title"><span class="editor-dot output-dot"></span>处理结果</span>
                        </template>
                        <template #extra>
                            <span class="character-count">{{ formattedResult.length }} 字符</span>
                        </template>
                        <a-spin :spinning="loading" tip="处理中..." class="result-spin">
                            <div class="json-result" :class="{ empty: !formattedResult }">
                                <pre v-if="formattedResult" class="formatted-json">{{ formattedResult }}</pre>
                                <a-empty v-else description="格式化或压缩后，结果会显示在这里" />
                            </div>
                        </a-spin>
                    </a-card>
                </a-col>
            </a-row>

            <div class="json-note">
                <SafetyOutlined /> 所有处理均在当前浏览器完成，输入内容不会上传到服务器
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import {
    SafetyOutlined,
    AlignLeftOutlined,
    CompressOutlined,
    CopyOutlined,
    ClearOutlined,
    FileTextOutlined
} from '@ant-design/icons-vue'
import { formatJSON, minifyJSON } from '../../services/frontendTools'

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

const loadExample = () => {
    inputText.value = exampleJson
    formattedResult.value = ''
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
    "theme": "warm",
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
    transition: border-color var(--duration-base) var(--ease-out), box-shadow var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out), transform var(--duration-base) var(--ease-out);
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
    transition: box-shadow var(--duration-base) var(--ease-out), border-color var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out), color var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out);
}

.tool-page :deep(.ant-card-hoverable:hover) .tool-icon {
    transform: scale(1.2) rotate(5deg);
}

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
    border-radius: var(--border-radius);
    font-weight: 600;
    transition: box-shadow var(--duration-base) var(--ease-out), border-color var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out), color var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out);
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
    transition: box-shadow var(--duration-base) var(--ease-out), border-color var(--duration-base) var(--ease-out), background-color var(--duration-base) var(--ease-out), color var(--duration-base) var(--ease-out), transform var(--duration-press) var(--ease-out);
}

.tool-page :deep(.ant-btn:not(.ant-btn-primary):hover) {
    background: var(--primary-color);
    color: white;
    transform: translateY(-2px);
}

@media (max-width: 768px) {
    .tool-page {
        padding: 1rem var(--page-gutter);
    }

    .container {
        padding: 0;
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

.json-page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 20px;
}

.local-badge {
    display: inline-flex;
    flex-shrink: 0;
    align-items: center;
    gap: 6px;
    padding: 5px 10px;
    color: var(--workspace-green);
    font-size: 11px;
    font-weight: 600;
    background: var(--workspace-green-soft);
    border: 1px solid color-mix(in srgb, var(--workspace-green) 18%, transparent);
    border-radius: 7px;
}

.json-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 18px;
    padding: 12px 14px;
    background: var(--workspace-surface);
    border: 1px solid var(--workspace-line);
    border-radius: var(--workspace-radius);
    box-shadow: var(--workspace-shadow);
}

.toolbar-primary,
.toolbar-secondary {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
}

.json-workspace {
    align-items: stretch;
}

.json-editor-col {
    display: flex;
    margin-bottom: 20px;
}

.json-editor-card {
    width: 100%;
    height: 100%;
}

.editor-title {
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.editor-dot {
    width: 7px;
    height: 7px;
    border-radius: 50%;
}

.input-dot {
    background: var(--workspace-accent);
}

.output-dot {
    background: var(--workspace-green);
}

.character-count {
    color: var(--workspace-caption);
    font-size: 11px;
    font-weight: 400;
    font-variant-numeric: tabular-nums;
}

.json-editor-card :deep(.ant-card-body) {
    padding: 0;
}

.json-editor-card :deep(.json-input) {
    min-height: 480px;
    padding: 18px;
    color: var(--workspace-text);
    font-family: 'SF Mono', Consolas, 'Courier New', monospace;
    font-size: 13px;
    line-height: 1.75;
    background: var(--workspace-surface);
    border: 0;
    border-radius: 0;
    resize: vertical;
}

.json-editor-card :deep(.json-input:focus) {
    background: var(--workspace-surface);
    box-shadow: inset 0 0 0 1px var(--workspace-accent-line);
}

.result-spin,
.result-spin :deep(.ant-spin-nested-loading),
.result-spin :deep(.ant-spin-container) {
    display: block;
    min-height: 480px;
}

.json-result {
    min-height: 480px;
    max-height: 640px;
    padding: 18px;
    overflow: auto;
    background: var(--workspace-bg);
}

.json-result.empty {
    display: flex;
    align-items: center;
    justify-content: center;
}

.json-result .formatted-json {
    min-height: 100%;
    margin: 0;
    padding: 0;
    color: var(--workspace-text);
    font-family: 'SF Mono', Consolas, 'Courier New', monospace;
    font-size: 13px;
    line-height: 1.75;
    overflow-wrap: anywhere;
    white-space: pre-wrap;
    background: transparent;
}

.json-note {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 7px;
    margin-top: 2px;
    color: var(--workspace-caption);
    font-size: 11px;
    line-height: 1.6;
    text-align: center;
}

@media (max-width: 768px) {
    .json-toolbar {
        align-items: flex-start;
    }

    .json-editor-card :deep(.json-input),
    .result-spin,
    .result-spin :deep(.ant-spin-nested-loading),
    .result-spin :deep(.ant-spin-container),
    .json-result {
        min-height: 360px;
    }
}

@media (max-width: 640px) {
    .json-page-header {
        align-items: flex-start;
    }

    .local-badge {
        margin-top: 3px;
    }

    .json-toolbar {
        flex-direction: column;
        gap: 10px;
    }

    .toolbar-primary,
    .toolbar-secondary {
        width: 100%;
    }

    .toolbar-primary :deep(.ant-btn),
    .toolbar-secondary :deep(.ant-btn) {
        flex: 1;
        padding-right: 10px;
        padding-left: 10px;
    }

    .json-editor-card :deep(.json-input),
    .result-spin,
    .result-spin :deep(.ant-spin-nested-loading),
    .result-spin :deep(.ant-spin-container),
    .json-result {
        min-height: 320px;
    }
}
</style>
