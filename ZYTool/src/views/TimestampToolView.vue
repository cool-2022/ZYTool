<template>
    <div class="tool-page">
        <div class="container">
            <!-- 页面标题 -->
            <div class="page-header">
                <a-typography-title :level="2">时间戳转换工具（前端处理）</a-typography-title>
                <a-typography-paragraph>时间戳与日期时间互相转换，浏览器本地处理</a-typography-paragraph>
            </div>

            <!-- 当前时间 -->
            <a-card title="当前时间" :bordered="false" style="margin-bottom: 24px;">
                <a-descriptions :column="{ xs: 1, sm: 2, md: 2 }" bordered>
                    <a-descriptions-item label="当前时间戳（毫秒）">
                        <a-space>
                            <span class="timestamp-value">{{ currentTimestamp }}</span>
                            <a-button size="small" @click="copyText(currentTimestamp.toString())">复制</a-button>
                        </a-space>
                    </a-descriptions-item>
                    <a-descriptions-item label="当前时间戳（秒）">
                        <a-space>
                            <span class="timestamp-value">{{ Math.floor(currentTimestamp / 1000) }}</span>
                            <a-button size="small"
                                @click="copyText(Math.floor(currentTimestamp / 1000).toString())">复制</a-button>
                        </a-space>
                    </a-descriptions-item>
                    <a-descriptions-item label="当前日期时间">
                        <a-space>
                            <span>{{ currentDateTime }}</span>
                            <a-button size="small" @click="copyText(currentDateTime)">复制</a-button>
                        </a-space>
                    </a-descriptions-item>
                    <a-descriptions-item label="ISO格式">
                        <a-space>
                            <span>{{ currentISO }}</span>
                            <a-button size="small" @click="copyText(currentISO)">复制</a-button>
                        </a-space>
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>

            <!-- 转换工具 -->
            <a-row :gutter="24">
                <!-- 时间戳转日期 -->
                <a-col :xs="24" :lg="12">
                    <a-card title="时间戳 → 日期时间" :bordered="false">
                        <a-space direction="vertical" style="width: 100%">
                            <div>
                                <label>时间戳：</label>
                                <a-input-group compact>
                                    <a-input v-model:value="inputTimestamp" placeholder="请输入时间戳（毫秒或秒）"
                                        style="width: calc(100% - 100px)" />
                                    <a-select v-model:value="timestampUnit" style="width: 100px">
                                        <a-select-option value="ms">毫秒</a-select-option>
                                        <a-select-option value="s">秒</a-select-option>
                                    </a-select>
                                </a-input-group>
                            </div>

                            <a-button type="primary" block @click="convertTimestamp">
                                转换为日期时间
                            </a-button>

                            <div v-if="timestampResult" class="result-box">
                                <a-descriptions :column="3" bordered size="small">
                                    <a-descriptions-item label="日 期：" :span="3">
                                        {{ timestampResult.datetime }}
                                        <a-button size="small"
                                            @click="copyText(timestampResult.datetime.toString())">复制</a-button>
                                    </a-descriptions-item>
                                    <a-descriptions-item label="年月日:">
                                        {{ timestampResult.date }}
                                    </a-descriptions-item>
                                    <a-descriptions-item label="时分秒:">
                                        {{ timestampResult.time }}
                                    </a-descriptions-item>
                                    <a-descriptions-item label="星 期:" :span="1">
                                        {{ timestampResult.weekday }}
                                    </a-descriptions-item>
                                </a-descriptions>
                            </div>
                        </a-space>
                    </a-card>
                </a-col>

                <!-- 日期转时间戳 -->
                <a-col :xs="24" :lg="12">
                    <a-card title="日期时间 → 时间戳" :bordered="false">
                        <a-space direction="vertical" style="width: 100%">
                            <div>
                                <label>日期时间：</label>
                                <a-date-picker v-model:value="inputDatetime" show-time format="YYYY-MM-DD HH:mm:ss"
                                    placeholder="选择日期时间" style="width: 100%" />
                            </div>

                            <a-button type="primary" block @click="convertDatetime">
                                转换为时间戳
                            </a-button>

                            <div v-if="datetimeResult" class="result-box">
                                <a-descriptions :column="1" bordered size="small">
                                    <a-descriptions-item label="毫秒时间戳:">
                                        <a-space>
                                            <span>{{ datetimeResult.milliseconds }}</span>
                                            <a-button size="small"
                                                @click="copyText(datetimeResult.milliseconds.toString())">复制</a-button>
                                        </a-space>
                                    </a-descriptions-item>
                                    <a-descriptions-item label="秒 时间戳:">
                                        <a-space>
                                            <span>{{ datetimeResult.seconds }}</span>
                                            <a-button size="small"
                                                @click="copyText(datetimeResult.seconds.toString())">复制</a-button>
                                        </a-space>
                                    </a-descriptions-item>
                                </a-descriptions>
                            </div>
                        </a-space>
                    </a-card>
                </a-col>
            </a-row>

            <!-- 使用说明 -->
            <a-divider>使用说明</a-divider>
            <a-card :bordered="false">
                <a-descriptions :column="1">
                    <a-descriptions-item label="什么是时间戳">
                        时间戳是指从1970年1月1日00:00:00 UTC到现在的总秒数或毫秒数
                    </a-descriptions-item>
                    <a-descriptions-item label="毫秒 vs 秒">
                        JavaScript使用毫秒时间戳（13位），Unix使用秒时间戳（10位）
                    </a-descriptions-item>
                    <a-descriptions-item label="使用场景">
                        数据库存储、日志记录、API接口、缓存过期时间、性能分析等
                    </a-descriptions-item>
                    <a-descriptions-item label="快捷操作">
                        <a-space>
                            <a-button size="small" @click="setTimestampNow">当前时间</a-button>
                            <a-button size="small" @click="setTimestampYesterday">昨天</a-button>
                            <a-button size="small" @click="setTimestampTomorrow">明天</a-button>
                            <a-button size="small" @click="setTimestampWeekAgo">一周前</a-button>
                        </a-space>
                    </a-descriptions-item>
                </a-descriptions>
            </a-card>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { message } from 'ant-design-vue'
import dayjs, { Dayjs } from 'dayjs'

const currentTimestamp = ref(Date.now())
const currentDateTime = computed(() => {
    return dayjs(currentTimestamp.value).format('YYYY-MM-DD HH:mm:ss.SSS')
})
const currentISO = computed(() => {
    return new Date(currentTimestamp.value).toISOString()
})

const inputTimestamp = ref('')
const timestampUnit = ref('ms')
const timestampResult = ref<any>(null)

const inputDatetime = ref<Dayjs | null>(null)
const datetimeResult = ref<any>(null)

let timer: number | null = null

// 更新当前时间
const updateCurrentTime = () => {
    currentTimestamp.value = Date.now()
}

// 转换时间戳为日期
const convertTimestamp = () => {
    if (!inputTimestamp.value) {
        message.warning('请输入时间戳')
        return
    }

    try {
        let ts = parseInt(inputTimestamp.value)

        // 如果是秒，转换为毫秒
        if (timestampUnit.value === 's') {
            ts = ts * 1000
        }

        const date = new Date(ts)

        if (isNaN(date.getTime())) {
            throw new Error('无效的时间戳')
        }

        const weekdays = ['星期日', '星期一', '星期二', '星期三', '星期四', '星期五', '星期六']

        timestampResult.value = {
            datetime: dayjs(ts).format('YYYY-MM-DD HH:mm:ss.SSS'),
            date: dayjs(ts).format('YYYY-MM-DD'),
            time: dayjs(ts).format('HH:mm:ss.SSS'),
            weekday: weekdays[date.getDay()]
        }

        message.success('转换成功')
    } catch (error) {
        message.error('转换失败：' + (error as Error).message)
        timestampResult.value = null
    }
}

// 转换日期为时间戳
const convertDatetime = () => {
    if (!inputDatetime.value) {
        message.warning('请选择日期时间')
        return
    }

    try {
        const ts = inputDatetime.value.valueOf()

        datetimeResult.value = {
            milliseconds: ts,
            seconds: Math.floor(ts / 1000)
        }

        message.success('转换成功')
    } catch (error) {
        message.error('转换失败：' + (error as Error).message)
        datetimeResult.value = null
    }
}

// 复制文本
const copyText = (text: string) => {
    navigator.clipboard.writeText(text)
        .then(() => message.success('复制成功'))
        .catch(() => message.error('复制失败'))
}

// 快捷操作
const setTimestampNow = () => {
    inputTimestamp.value = Date.now().toString()
    timestampUnit.value = 'ms'
    inputDatetime.value = dayjs()
}

const setTimestampYesterday = () => {
    inputTimestamp.value = (Date.now() - 24 * 60 * 60 * 1000).toString()
    timestampUnit.value = 'ms'
    inputDatetime.value = dayjs().subtract(1, 'day')
}

const setTimestampTomorrow = () => {
    inputTimestamp.value = (Date.now() + 24 * 60 * 60 * 1000).toString()
    timestampUnit.value = 'ms'
    inputDatetime.value = dayjs().add(1, 'day')
}

const setTimestampWeekAgo = () => {
    inputTimestamp.value = (Date.now() - 7 * 24 * 60 * 60 * 1000).toString()
    timestampUnit.value = 'ms'
    inputDatetime.value = dayjs().subtract(7, 'day')
}

onMounted(() => {
    timer = setInterval(updateCurrentTime, 100) as unknown as number
})

onUnmounted(() => {
    if (timer) {
        clearInterval(timer)
    }
})
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
