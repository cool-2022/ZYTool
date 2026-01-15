<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">颜色选择器（前端处理）</a-typography-title>
                <a-typography-paragraph>RGB、HEX颜色转换，浏览器本地处理</a-typography-paragraph>
            </div>

            <!-- 颜色选择器 -->
            <a-row :gutter="24">
                <a-col :xs="24" :lg="12">
                    <a-card title="颜色选择" :bordered="false">
                        <div class="color-preview" :style="{ backgroundColor: hexColor }"></div>
                        <a-divider />

                        <!-- 原生颜色选择器 -->
                        <div class="color-input-group">
                            <label>颜色选择：</label>
                            <input type="color" v-model="hexColor" @input="onColorChange" class="color-picker" />
                            <a-input v-model:value="hexColor" placeholder="#000000" @change="onHexInputChange"
                                style="flex: 1; margin-left: 10px;" />
                        </div>

                        <a-divider />

                        <!-- RGB 输入 -->
                        <a-space direction="vertical" style="width: 100%">
                            <div class="rgb-input-group">
                                <label>R:</label>
                                <a-slider v-model:value="rgbColor.r" :min="0" :max="255" @change="onRgbChange"
                                    style="flex: 1; margin: 0 10px;" />
                                <a-input-number v-model:value="rgbColor.r" :min="0" :max="255" @change="onRgbChange"
                                    style="width: 80px;" />
                            </div>
                            <div class="rgb-input-group">
                                <label>G:</label>
                                <a-slider v-model:value="rgbColor.g" :min="0" :max="255" @change="onRgbChange"
                                    style="flex: 1; margin: 0 10px;" />
                                <a-input-number v-model:value="rgbColor.g" :min="0" :max="255" @change="onRgbChange"
                                    style="width: 80px;" />
                            </div>
                            <div class="rgb-input-group">
                                <label>B:</label>
                                <a-slider v-model:value="rgbColor.b" :min="0" :max="255" @change="onRgbChange"
                                    style="flex: 1; margin: 0 10px;" />
                                <a-input-number v-model:value="rgbColor.b" :min="0" :max="255" @change="onRgbChange"
                                    style="width: 80px;" />
                            </div>
                        </a-space>
                    </a-card>
                </a-col>

                <a-col :xs="24" :lg="12">
                    <a-card title="颜色信息" :bordered="false">
                        <a-descriptions :column="1" bordered>
                            <a-descriptions-item label="HEX">
                                <a-space>
                                    <span class="color-value">{{ hexColor }}</span>
                                    <a-button size="small" @click="copyColor('hex')">复制</a-button>
                                </a-space>
                            </a-descriptions-item>
                            <a-descriptions-item label="RGB">
                                <a-space>
                                    <span class="color-value">{{ rgbString }}</span>
                                    <a-button size="small" @click="copyColor('rgb')">复制</a-button>
                                </a-space>
                            </a-descriptions-item>
                            <a-descriptions-item label="RGBA">
                                <a-space>
                                    <span class="color-value">{{ rgbaString }}</span>
                                    <a-button size="small" @click="copyColor('rgba')">复制</a-button>
                                </a-space>
                            </a-descriptions-item>
                        </a-descriptions>

                        <a-divider>常用颜色</a-divider>
                        <div class="preset-colors">
                            <div v-for="color in presetColors" :key="color" class="preset-color"
                                :style="{ backgroundColor: color }" @click="selectPresetColor(color)" :title="color">
                            </div>
                        </div>
                    </a-card>
                </a-col>
            </a-row>

            <!-- 使用说明 -->
            <a-divider>使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="HEX颜色">
                        十六进制颜色代码，格式为 #RRGGBB，例如 #FF0000 表示红色
                    </a-descriptions-item>
                    <a-descriptions-item label="RGB颜色">
                        红绿蓝三原色，每个值范围0-255，例如 rgb(255, 0, 0) 表示红色
                    </a-descriptions-item>
                    <a-descriptions-item label="使用场景">
                        网页设计、UI设计、CSS样式编写、图像处理等
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { message } from 'ant-design-vue'
import { rgbToHex, hexToRgb } from '../services/frontendTools'

const hexColor = ref('#63d2d2')
const rgbColor = ref({ r: 99, g: 210, b: 210 })

// 预设颜色
const presetColors = [
    '#FF0000', '#00FF00', '#0000FF', '#FFFF00', '#FF00FF', '#00FFFF',
    '#000000', '#FFFFFF', '#808080', '#C0C0C0', '#800000', '#808000',
    '#008000', '#800080', '#008080', '#000080', '#FFA500', '#FFC0CB',
    '#63d2d2', '#4fb8b8', '#3a9e9e', '#7de3e3', '#a8ecec', '#c5f3f3'
]

// RGB字符串
const rgbString = computed(() => {
    return `rgb(${rgbColor.value.r}, ${rgbColor.value.g}, ${rgbColor.value.b})`
})

// RGBA字符串
const rgbaString = computed(() => {
    return `rgba(${rgbColor.value.r}, ${rgbColor.value.g}, ${rgbColor.value.b}, 1)`
})

// 颜色选择器变化
const onColorChange = () => {
    const rgb = hexToRgb(hexColor.value)
    if (rgb) {
        rgbColor.value = rgb
    }
}

// HEX输入变化
const onHexInputChange = () => {
    if (hexColor.value.match(/^#[0-9A-Fa-f]{6}$/)) {
        onColorChange()
    }
}

// RGB变化
const onRgbChange = () => {
    hexColor.value = rgbToHex(rgbColor.value.r, rgbColor.value.g, rgbColor.value.b)
}

// 选择预设颜色
const selectPresetColor = (color: string) => {
    hexColor.value = color
    onColorChange()
}

// 复制颜色值
const copyColor = (type: string) => {
    let text = ''
    switch (type) {
        case 'hex':
            text = hexColor.value
            break
        case 'rgb':
            text = rgbString.value
            break
        case 'rgba':
            text = rgbaString.value
            break
    }

    navigator.clipboard.writeText(text)
        .then(() => message.success(`已复制: ${text}`))
        .catch(() => message.error('复制失败'))
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

.color-preview {
    width: 100%;
    height: 200px;
    border-radius: var(--border-radius);
    border: 2px solid var(--border-color);
    transition: all 0.3s ease;
    box-shadow: var(--shadow-md);
}

.color-input-group {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 1rem;
}

.color-input-group label {
    min-width: 80px;
    font-weight: 600;
    color: var(--text-primary);
}

.color-picker {
    width: 60px;
    height: 40px;
    border: 2px solid var(--border-color);
    border-radius: var(--border-radius);
    cursor: pointer;
}

.rgb-input-group {
    display: flex;
    align-items: center;
    gap: 10px;
}

.rgb-input-group label {
    min-width: 30px;
    font-weight: 600;
    color: var(--text-primary);
}

.color-value {
    font-family: 'Courier New', monospace;
    font-weight: 600;
    color: var(--text-primary);
}

.preset-colors {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(50px, 1fr));
    gap: 10px;
}

.preset-color {
    width: 100%;
    height: 50px;
    border-radius: var(--border-radius);
    border: 2px solid var(--border-color);
    cursor: pointer;
    transition: all 0.3s ease;
}

.preset-color:hover {
    transform: scale(1.1);
    box-shadow: var(--shadow-md);
    border-color: var(--primary-color);
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
