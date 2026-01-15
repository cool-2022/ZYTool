<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">文本比对工具</a-typography-title>
                <a-typography-paragraph>支持文本文件比对，快速发现差异</a-typography-paragraph>
            </div>

            <!-- 上传区域 -->
            <a-row :gutter="24">
                <a-col :xs="24" :lg="12">
                    <a-card title="A 侧文件" :bordered="false">
                        <a-space direction="vertical" style="width: 100%" :size="16">
                            <a-upload-dragger v-model:fileList="fileListA" :before-upload="beforeUploadA"
                                :customRequest="() => { }" @change="handleChangeA" :multiple="false">
                                <p class="ant-upload-drag-icon">
                                    <inbox-outlined></inbox-outlined>
                                </p>
                                <p class="ant-upload-text">点击或拖拽文件到此区域</p>
                                <p class="ant-upload-hint">
                                    仅支持单个文本文件上传
                                </p>
                            </a-upload-dragger>

                            <a-button @click="clearA" block>清空</a-button>
                        </a-space>
                    </a-card>
                </a-col>

                <a-col :xs="24" :lg="12">
                    <a-card title="B 侧文件" :bordered="false">
                        <a-space direction="vertical" style="width: 100%" :size="16">
                            <a-upload-dragger v-model:fileList="fileListB" :before-upload="beforeUploadB"
                                :customRequest="() => { }" @change="handleChangeB" :multiple="false">
                                <p class="ant-upload-drag-icon">
                                    <inbox-outlined></inbox-outlined>
                                </p>
                                <p class="ant-upload-text">点击或拖拽文件到此区域</p>
                                <p class="ant-upload-hint">
                                    仅支持单个文本文件上传
                                </p>
                            </a-upload-dragger>

                            <a-button @click="clearB" block>清空</a-button>
                        </a-space>
                    </a-card>
                </a-col>
            </a-row>

            <!-- 比对按钮 -->
            <div style="text-align: center; margin: 24px 0;">
                <a-button type="primary" size="large" @click="compareFiles" :loading="comparing">
                    开始比对
                </a-button>
            </div>

            <!-- 比对结果 -->
            <a-card v-if="diffResult" title="比对结果" :bordered="false" style="margin-top: 24px;">
                <!-- 文件比对结果 -->
                <div v-if="compareMode === 'file' && diffResult && 'onlyInA' in diffResult">
                    <a-row :gutter="24">
                        <!-- A文件差异 -->
                        <a-col :xs="24" :lg="12">
                            <a-card title="A 文件独有的行" :bordered="false" size="small">
                                <div class="diff-content">
                                    <div v-for="(line, index) in diffResult.onlyInA" :key="'a-' + index"
                                        class="diff-line diff-line-removed">
                                        <span class="line-number">{{ line.lineNumber }}</span>
                                        <span class="line-content">{{ line.content }}</span>
                                    </div>
                                    <a-empty v-if="diffResult.onlyInA.length === 0" description="无差异" />
                                </div>
                            </a-card>
                        </a-col>

                        <!-- B文件差异 -->
                        <a-col :xs="24" :lg="12">
                            <a-card title="B 文件独有的行" :bordered="false" size="small">
                                <div class="diff-content">
                                    <div v-for="(line, index) in diffResult.onlyInB" :key="'b-' + index"
                                        class="diff-line diff-line-added">
                                        <span class="line-number">{{ line.lineNumber }}</span>
                                        <span class="line-content">{{ line.content }}</span>
                                    </div>
                                    <a-empty v-if="diffResult.onlyInB.length === 0" description="无差异" />
                                </div>
                            </a-card>
                        </a-col>
                    </a-row>
                </div>
            </a-card>

            <!-- 使用说明 -->
            <a-divider>使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="文件比对">
                        分别上传两个文本文件进行逐行比对，显示各自独有的行
                    </a-descriptions-item>
                    <a-descriptions-item label="支持格式">
                        支持所有文本文件格式（.txt, .js, .vue, .json, .md 等）
                    </a-descriptions-item>
                    <a-descriptions-item label="注意事项">
                        每次只能上传一个文件，不支持批量上传和文件夹上传
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { message } from 'ant-design-vue'
import { InboxOutlined, FileTextOutlined } from '@ant-design/icons-vue'
import type { UploadProps } from 'ant-design-vue'
import type { DiffLine, FileDiffResult, FolderDiffResult, DiffResult } from '../services/api'
import { ApiService } from '../services/api'

const fileListA = ref<any[]>([])
const fileListB = ref<any[]>([])
const comparing = ref(false)
const diffResult = ref<DiffResult | null>(null)
const compareMode = ref<'file'>('file')

const filesA = ref<Map<string, File>>(new Map())
const filesB = ref<Map<string, File>>(new Map())

// A侧上传前处理
const beforeUploadA: UploadProps['beforeUpload'] = (file) => {
    // 只允许单个文件
    filesA.value.clear()
    filesA.value.set(file.name, file)
    return false
}

// B侧上传前处理
const beforeUploadB: UploadProps['beforeUpload'] = (file) => {
    // 只允许单个文件
    filesB.value.clear()
    filesB.value.set(file.name, file)
    return false
}

// A侧文件变化
const handleChangeA = () => {
    // 文件列表已更新
}

// B侧文件变化
const handleChangeB = () => {
    // 文件列表已更新
}

// 清空A侧
const clearA = () => {
    fileListA.value = []
    filesA.value.clear()
    diffResult.value = null
}

// 清空B侧
const clearB = () => {
    fileListB.value = []
    filesB.value.clear()
    diffResult.value = null
}

// 读取文件内容
const readFileContent = (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
        const reader = new FileReader()
        reader.onload = (e) => {
            resolve(e.target?.result as string)
        }
        reader.onerror = reject
        reader.readAsText(file)
    })
}

// 比对两个文件内容（调用后端接口）
const compareFileContent = async (fileA: File, fileB: File): Promise<FileDiffResult> => {
    // 读取文件内容
    const contentA = await readFileContent(fileA)
    const contentB = await readFileContent(fileB)

    // 调用后端接口进行比对
    const result = await ApiService.compareFiles({
        text1: contentA,
        text2: contentB
    })

    return result
}

// 开始比对
const compareFiles = async () => {
    if (filesA.value.size === 0 || filesB.value.size === 0) {
        message.warning('请先上传两侧的文件')
        return
    }

    comparing.value = true
    diffResult.value = null

    try {
        // 文件比对
        compareMode.value = 'file'
        const fileA = Array.from(filesA.value.values())[0]
        const fileB = Array.from(filesB.value.values())[0]

        if (!fileA || !fileB) {
            message.error('请确保两侧都上传了文件')
            return
        }

        // 调用后端接口进行比对
        diffResult.value = await compareFileContent(fileA, fileB)
        message.success('文件比对完成')
    } catch (error) {
        message.error('比对失败: ' + (error as Error).message)
        console.error(error)
    } finally {
        comparing.value = false
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
    max-width: 1400px;
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

.tool-page :deep(.ant-btn-primary) {
    background: var(--gradient-primary);
    border: none;
}

.tool-page :deep(.ant-upload-drag) {
    background: var(--bg-secondary);
    border-color: var(--border-color);
    transition: all var(--transition-speed);
}

.tool-page :deep(.ant-upload-drag:hover) {
    border-color: var(--primary-color);
    background: var(--bg-tertiary);
}

.tool-page :deep(.ant-upload-text) {
    color: var(--text-primary) !important;
}

.tool-page :deep(.ant-upload-hint) {
    color: var(--text-secondary) !important;
}

.tool-page :deep(.ant-upload-drag-icon) {
    color: var(--primary-color);
}

/* 深色模式下上传组件特殊适配 */
@media (prefers-color-scheme: dark) {
    .tool-page :deep(.ant-upload-drag) {
        background: var(--bg-secondary);
        border-color: var(--border-color);
    }

    .tool-page :deep(.ant-upload-drag:hover) {
        background: var(--bg-tertiary);
        border-color: var(--primary-color);
    }

    .tool-page :deep(.ant-upload-text) {
        color: #ffffff !important;
    }

    .tool-page :deep(.ant-upload-hint) {
        color: rgba(255, 255, 255, 0.65) !important;
    }

    .tool-page :deep(.ant-upload-drag-icon .anticon) {
        color: var(--primary-color);
    }

    /* 上传文件列表适配 */
    .tool-page :deep(.ant-upload-list-item) {
        color: #ffffff !important;
    }

    .tool-page :deep(.ant-upload-list-item-name) {
        color: #ffffff !important;
    }

    .tool-page :deep(.ant-upload-list-item-info) {
        background: var(--bg-secondary);
    }

    .tool-page :deep(.ant-upload-list-item:hover) {
        background: var(--bg-tertiary);
    }

    .tool-page :deep(.ant-upload-list-item .anticon) {
        color: rgba(255, 255, 255, 0.65);
    }

    .tool-page :deep(.ant-upload-list-item .tool-page) {
        background: rgba(255, 255, 255, 0.65);
    }



    .tool-page :deep(.ant-list-bordered) {
        border-color: var(--border-color);
    }

    .tool-page :deep(.ant-list-item) {
        border-color: var(--border-color);
        color: var(--text-primary);
    }

    .tool-page :deep(.ant-empty-description) {
        color: var(--text-secondary);
    }

}

.diff-content {
    max-height: 500px;
    overflow-y: auto;
    background: var(--bg-secondary);
    border-radius: var(--border-radius);
    padding: 1rem;
}

.diff-line {
    display: flex;
    padding: 4px 8px;
    margin: 2px 0;
    border-radius: 4px;
    font-family: 'Courier New', monospace;
    font-size: 14px;
    line-height: 1.6;
}

.diff-line-removed {
    background: rgba(255, 77, 79, 0.15);
    border-left: 3px solid #ff4d4f;
}

@media (prefers-color-scheme: dark) {
    .diff-line-removed {
        background: rgba(255, 77, 79, 0.2);
        border-left: 3px solid #ff7875;
    }
}

.diff-line-added {
    background: rgba(82, 196, 26, 0.15);
    border-left: 3px solid #52c41a;
}

@media (prefers-color-scheme: dark) {
    .diff-line-added {
        background: rgba(82, 196, 26, 0.2);
        border-left: 3px solid #73d13d;
    }
}

.line-number {
    display: inline-block;
    width: 50px;
    color: var(--text-secondary);
    text-align: right;
    margin-right: 12px;
    user-select: none;
}

.line-content {
    flex: 1;
    color: var(--text-primary);
    white-space: pre-wrap;
    word-break: break-all;
}

.tool-page :deep(.ant-descriptions-item-label) {
    color: var(--text-primary);
    background: var(--bg-secondary);
}

.tool-page :deep(.ant-descriptions-item-content) {
    color: var(--text-primary);
    background: var(--bg-primary);
}
</style>
