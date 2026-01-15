<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">URL编码工具（前端处理）</a-typography-title>
                <a-typography-paragraph>URL编码解码，浏览器本地处理，保护隐私</a-typography-paragraph>
            </div>

            <!-- 工具内容 -->
            <a-row :gutter="24">
                <a-col :xs="24" :lg="12">
                    <a-card title="输入文本" :bordered="false">
                        <a-textarea v-model:value="inputText" placeholder="请输入要编码或解码的URL..." :rows="15"
                            @change="handleInputChange" />
                    </a-card>
                </a-col>
                <a-col :xs="24" :lg="12">
                    <a-card title="处理结果" :bordered="false">
                        <template #extra>
                            <a-space>
                                <a-button @click="encode" type="primary">
                                    编码
                                </a-button>
                                <a-button @click="decode">
                                    解码
                                </a-button>
                                <a-button @click="copyResult">复制</a-button>
                                <a-button @click="clearAll">清空</a-button>
                            </a-space>
                        </template>
                        <div class="result-container">
                            <pre v-if="result" class="result-text">{{ result }}</pre>
                            <a-empty v-else description="暂无结果" />
                        </div>
                    </a-card>
                </a-col>
            </a-row>

            <!-- 使用说明 -->
            <a-divider>使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="什么是URL编码">
                        URL编码（百分号编码）是将URL中的特殊字符转换为%加两位十六进制数的形式，确保URL能够被正确传输
                    </a-descriptions-item>
                    <a-descriptions-item label="需要编码的字符">
                        空格、中文、特殊符号（如 & = ? # 等）、非ASCII字符等
                    </a-descriptions-item>
                    <a-descriptions-item label="使用场景">
                        URL参数传递、搜索引擎查询、API接口调用、超链接构建等
                    </a-descriptions-item>
                    <a-descriptions-item label="示例">
                        <a-space>
                            <a-button size="small" @click="loadExample('url')">示例1: 带参数的URL</a-button>
                            <a-button size="small" @click="loadExample('chinese')">示例2: 中文URL</a-button>
                            <a-button size="small" @click="loadExample('encoded')">示例3: 已编码URL</a-button>
                        </a-space>
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { urlEncode, urlDecode } from '../services/frontendTools'

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

// 处理输入变化
const handleInputChange = () => {
    // 可以添加实时验证
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
}
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
