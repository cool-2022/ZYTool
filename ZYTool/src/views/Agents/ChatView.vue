<template>
    <a-config-provider :theme="chatTheme">
    <div class="chat-view-page warm-theme">
        <header class="chat-page-header">
            <div class="chat-page-brand">
                <span class="chat-page-mark" aria-hidden="true"><RobotOutlined /></span>
                <div class="chat-page-copy">
                    <h1>AI 助手<span class="title-dot">.</span></h1>
                    <p>聊聊问题，也聊聊新想法</p>
                </div>
            </div>
            <div class="chat-page-actions">
                <span class="chat-purpose">写作 · 编程 · 灵感</span>
            </div>
        </header>

        <div class="chat-workspace">
        <div v-if="!sidebarCollapsed" class="sidebar-backdrop" aria-hidden="true" @click="toggleSidebar"></div>

        <div v-if="sidebarCollapsed" class="sidebar-collapsed-rail">
            <a-tooltip title="展开对话列表" placement="right">
                <a-button type="text" class="sidebar-toggle" aria-label="展开对话列表"
                    aria-controls="chat-session-list" :aria-expanded="!sidebarCollapsed" @click="toggleSidebar">
                    <MenuUnfoldOutlined /><span class="rail-label">对话</span>
                </a-button>
            </a-tooltip>
            <a-tooltip title="新建对话" placement="right">
                <a-button type="text" class="rail-new-chat" aria-label="新建对话" @click="createNewSession">
                    <PlusOutlined />
                </a-button>
            </a-tooltip>
        </div>

        <aside id="chat-session-list" class="chat-sidebar" :class="{ collapsed: sidebarCollapsed }"
            :inert="sidebarCollapsed" aria-label="对话列表">
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <span class="sidebar-logo-icon" aria-hidden="true"><MessageOutlined /></span>
                    <div>
                        <div class="sidebar-logo-title">我的对话</div>
                        <div class="sidebar-logo-subtitle">每一次交流，都有新收获</div>
                    </div>
                </div>
                <a-button type="text" class="sidebar-close" aria-label="收起对话列表" @click="toggleSidebar">
                    <MenuFoldOutlined />
                </a-button>
            </div>

            <div class="sidebar-new-chat">
                <a-button type="primary" block class="new-chat-button" @click="createNewSession">
                    <PlusOutlined />
                    新建对话
                </a-button>
            </div>

            <div class="sidebar-content">
                <a-spin :spinning="loadingSessions">
                    <div v-if="chatSessions.length === 0 && !loadingSessions" class="session-empty">
                        <MessageOutlined />
                        <span>还没有历史对话</span>
                    </div>
                    <template v-for="(group, date) in groupedSessions" :key="date">
                        <div class="session-group">
                            <div class="session-group-title">{{ date }}</div>
                            <div v-for="session in group" :key="session.id" class="session-item"
                                :class="{ active: currentSession?.id === session.id }" role="button" tabindex="0"
                                :aria-current="currentSession?.id === session.id ? 'true' : undefined"
                                @click="selectSession(session)" @keydown.enter.self="selectSession(session)"
                                @keydown.space.self.prevent="selectSession(session)">
                                <MessageOutlined class="session-icon" />
                                <a-input v-if="editingSessionId === session.id" v-model:value="editingTitle"
                                    size="small" class="session-rename-input" autofocus @click.stop
                                    @press-enter="confirmRename(session.id)" @blur="confirmRename(session.id)"
                                    @keydown.esc.stop="cancelRename" />
                                <div v-else class="session-title" :title="session.title"
                                    @dblclick.stop="startRename(session)">{{ session.title }}</div>
                                <a-popconfirm title="删除这个对话？" ok-text="删除" cancel-text="取消"
                                    overlay-class-name="chat-session-confirm" @confirm="deleteSession(session.id)">
                                    <a-button type="text" size="small" danger class="session-delete"
                                        aria-label="删除对话" @click.stop>
                                        <DeleteOutlined />
                                    </a-button>
                                </a-popconfirm>
                            </div>
                        </div>
                    </template>
                </a-spin>
            </div>

            <div class="sidebar-footer">
                <span>共 {{ chatSessions.length }} 个对话</span>
                <span>双击标题可重命名</span>
            </div>
        </aside>

        <main class="chat-main">
            <div v-if="messages.length > 0" class="chat-header">
                <div class="chat-header-info">
                    <span class="conversation-label"><MessageOutlined /> {{ messages.length }} 条消息</span>
                    <span v-if="isSending" class="generation-label" role="status">正在生成回复…</span>
                </div>
                <div class="chat-header-actions">
                    <a-tooltip title="仅清空当前页面，不删除服务器中的历史消息">
                        <a-button type="text" class="clear-button" @click="clearCurrentChat">
                            <ClearOutlined />
                            <span>清空</span>
                        </a-button>
                    </a-tooltip>
                </div>
            </div>

            <div class="chat-messages-container">
                <a-spin :spinning="loadingMessages" tip="加载消息中..." class="chat-loading">
                    <div class="chat-conversation">
                        <div v-if="messages.length === 0 && !loadingMessages" class="chat-empty">
                            <span class="chat-empty-tag"><BulbOutlined /> 给想法一点空间</span>
                            <h2 class="chat-empty-text">把想法，<span>聊成答案</span></h2>
                            <p class="chat-empty-hint">写一段文字，解一个难题，或一起理清下一步。<br class="welcome-break" />从你想聊的事情开始就好。</p>
                            <div class="prompt-grid">
                                <button v-for="prompt in quickPrompts" :key="prompt.title" type="button"
                                    class="prompt-card" @click="applyPrompt(prompt.content)">
                                    <span class="prompt-icon"><component :is="prompt.icon" /></span>
                                    <span class="prompt-copy">
                                        <strong>{{ prompt.title }}</strong>
                                        <span>{{ prompt.description }}</span>
                                    </span>
                                    <ArrowUpOutlined class="prompt-arrow" />
                                </button>
                            </div>
                        </div>

                        <div v-for="msg in messages" :key="msg.id" class="message-item"
                            :class="{ self: msg.role === 'user' }">
                            <div class="message-avatar" :class="msg.role">
                                <UserOutlined v-if="msg.role === 'user'" />
                                <RobotOutlined v-else />
                            </div>
                            <div class="message-content">
                                <div class="message-meta">
                                    <span class="message-role">{{ msg.role === 'user' ? '你' : 'ZY AI' }}</span>
                                    <span class="message-time">{{ formatTime(msg.timestamp) }}</span>
                                </div>
                                <div class="message-bubble">
                                    <div v-if="isStreamingMessage(msg) && !msg.content" class="thinking-indicator"
                                        role="status">
                                        <span class="thinking-dots"><i></i><i></i><i></i></span>
                                        <span class="thinking-text">正在思考</span>
                                    </div>
                                    <div v-else class="message-text" :class="{ streaming: isStreamingMessage(msg) }">{{ msg.content }}</div>
                                </div>
                                <div v-if="msg.role === 'assistant' && msg.content && !isStreamingMessage(msg)"
                                    class="message-actions">
                                    <a-tooltip :title="copiedMessageId === msg.id ? '已复制' : '复制内容'">
                                        <a-button type="text" size="small" class="message-action"
                                            :aria-label="copiedMessageId === msg.id ? '已复制' : '复制内容'"
                                            @click="copyMessage(msg)">
                                            <CheckOutlined v-if="copiedMessageId === msg.id" />
                                            <CopyOutlined v-else />
                                        </a-button>
                                    </a-tooltip>
                                </div>
                            </div>
                        </div>
                    </div>
                </a-spin>
            </div>

            <div class="chat-input-area">
                <div class="chat-input-shell">
                    <div class="chat-input-wrapper">
                        <a-textarea ref="inputRef" v-model:value="userInput" aria-label="输入消息"
                            placeholder="有什么想聊的？写下你的问题或想法…" :auto-size="{ minRows: 2, maxRows: 6 }"
                            @keydown="handleInputKeydown" />
                        <div class="chat-input-toolbar">
                            <div class="input-tools">
                                <a-tooltip :title="isListening ? '停止语音输入' : '语音输入'">
                                    <a-button type="text" class="voice-button" :class="{ 'voice-active': isListening }"
                                        :aria-label="isListening ? '停止语音输入' : '语音输入'" @click="toggleVoice">
                                        <AudioOutlined />
                                    </a-button>
                                </a-tooltip>
                                <span v-if="isListening" class="voice-status">正在聆听，请开始说话</span>
                                <span v-else class="input-hint">Enter 发送 · Shift + Enter 换行</span>
                            </div>
                            <div class="input-submit-actions">
                                <a-select v-model:value="selectedModelId" class="model-select" :options="modelOptions"
                                    popup-class-name="chat-model-dropdown" placeholder="选择模型" :bordered="false" aria-label="选择模型" />
                                <a-button v-if="isSending" danger class="stop-button" @click="stopStreaming">
                                    <StopOutlined />
                                    停止生成
                                </a-button>
                                <a-tooltip v-else title="发送消息">
                                    <a-button type="primary" class="send-button" aria-label="发送消息"
                                        :disabled="!userInput.trim()" @click="sendMessage">
                                        <ArrowUpOutlined />
                                    </a-button>
                                </a-tooltip>
                            </div>
                        </div>
                    </div>
                    <div class="chat-input-disclaimer">内容由 AI 生成，请注意甄别重要信息</div>
                </div>
            </div>
        </main>
        </div>
    </div>
    </a-config-provider>
</template>

<script setup lang="ts">
import { computed, inject, nextTick, onBeforeUnmount, onMounted, ref } from 'vue'
import type { Ref } from 'vue'
import {
    RobotOutlined,
    PlusOutlined,
    DeleteOutlined,
    MenuUnfoldOutlined,
    MenuFoldOutlined,
    ClearOutlined,
    MessageOutlined,
    UserOutlined,
    StopOutlined,
    AudioOutlined,
    ArrowUpOutlined,
    BulbOutlined,
    CodeOutlined,
    FileTextOutlined,
    CompassOutlined,
    CopyOutlined,
    CheckOutlined
} from '@ant-design/icons-vue'
import { message, theme as antdTheme } from 'ant-design-vue'
import { useChatView } from './ChatView'
import { ToolsFuntions } from '@/utils/startVoiceInput'

const pageTheme = inject<Readonly<Ref<'light' | 'dark'>>>('theme', ref<'light' | 'dark'>('light'))
const chatTheme = computed(() => {
    const dark = pageTheme.value === 'dark'
    return {
        algorithm: dark ? antdTheme.darkAlgorithm : antdTheme.defaultAlgorithm,
        token: {
            colorPrimary: dark ? '#ed9b7e' : '#cf654a',
            colorInfo: dark ? '#ed9b7e' : '#cf654a',
            colorSuccess: dark ? '#a0c0a8' : '#5d8d74',
            colorBgBase: dark ? '#24221f' : '#f8f6f0',
            colorBgContainer: dark ? '#2e2b27' : '#fffdf9',
            colorBgElevated: dark ? '#2e2b27' : '#fffdf9',
            colorText: dark ? '#eee8df' : '#423e38',
            colorTextSecondary: dark ? '#c0b4a4' : '#81786e',
            colorBorder: dark ? '#433c33' : '#ece6dc',
            colorPrimaryBg: dark ? '#443027' : '#f3e7dd',
            borderRadius: 8,
        },
    }
})

const {
    chatSessions,
    currentSession,
    messages,
    userInput,
    isSending,
    sidebarCollapsed,
    loadingSessions,
    loadingMessages,
    chatModels,
    selectedModelId,
    selectSession,
    createNewSession,
    sendMessage,
    stopStreaming,
    renameSession,
    deleteSession,
    toggleSidebar,
    clearCurrentChat,
    init
} = useChatView()

onMounted(() => {
    init()
})

const inputRef = ref<{ focus: () => void } | null>(null)
const editingSessionId = ref<string | null>(null)
const editingTitle = ref('')

function startRename(session: { id: string; title: string }) {
    editingSessionId.value = session.id
    editingTitle.value = session.title
}

function cancelRename() {
    editingSessionId.value = null
    editingTitle.value = ''
}

async function confirmRename(sessionId: string) {
    if (editingSessionId.value !== sessionId) return
    const newTitle = editingTitle.value
    cancelRename()
    const session = chatSessions.value.find((s) => s.id === sessionId)
    if (!session || session.title === newTitle.trim()) return
    await renameSession(sessionId, newTitle)
}

function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey && !e.isComposing) {
        e.preventDefault()
        sendMessage()
    }
}

function isStreamingMessage(msg: { id: string; role: string }) {
    if (!isSending.value || msg.role !== 'assistant') return false
    const last = messages.value[messages.value.length - 1]
    return last?.id === msg.id
}

const isListening = ref(false)
let stopVoice: (() => void) | null = null

function toggleVoice() {
    if (isListening.value) {
        stopVoice?.()
        stopVoice = null
        isListening.value = false
        return
    }

    isListening.value = true
    stopVoice = ToolsFuntions.startVoiceInput(
        (text) => {
            userInput.value = text
            isListening.value = false
            stopVoice = null
        },
        (err) => {
            message.error(err)
            isListening.value = false
            stopVoice = null
        }
    )
}

const modelOptions = computed(() =>
    chatModels.value.map((m) => ({
        value: m.id,
        label: m.display_name || m.model_name,
    }))
)

const groupedSessions = computed(() => {
    const groups: Record<string, typeof chatSessions.value> = {}
    chatSessions.value.forEach(session => {
        if (!groups[session.date]) {
            groups[session.date] = []
        }
        groups[session.date]!.push(session)
    })
    return groups
})

const quickPrompts = [
    { title: '激发创意', description: '头脑风暴一个新想法', content: '帮我头脑风暴一个关于效率工具的创意，并给出可落地的方案。', icon: BulbOutlined },
    { title: '辅助编程', description: '解释代码或排查问题', content: '请帮我分析一段代码的问题，并给出清晰的修改建议。', icon: CodeOutlined },
    { title: '润色写作', description: '让表达更清晰自然', content: '请帮我润色一段文字，让表达更清晰、简洁、自然。', icon: FileTextOutlined },
    { title: '制定计划', description: '把目标拆成具体步骤', content: '帮我制定一个学习计划，把目标拆分为可执行的具体步骤。', icon: CompassOutlined }
]

function applyPrompt(content: string) {
    userInput.value = content
    nextTick(() => inputRef.value?.focus())
}

const copiedMessageId = ref<string | null>(null)
let copyTimer: number | undefined

async function copyMessage(msg: { id: string; content: string }) {
    try {
        await navigator.clipboard.writeText(msg.content)
        copiedMessageId.value = msg.id
        window.clearTimeout(copyTimer)
        copyTimer = window.setTimeout(() => {
            copiedMessageId.value = null
        }, 1600)
    } catch {
        message.error('复制失败，请手动选择文本复制')
    }
}

onBeforeUnmount(() => {
    stopVoice?.()
    window.clearTimeout(copyTimer)
})

function formatTime(date: Date) {
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const minutes = Math.floor(diff / 60000)

    if (minutes < 1) return '刚刚'
    if (minutes < 60) return `${minutes} 分钟前`

    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours} 小时前`

    return date.toLocaleString('zh-CN', {
        month: '2-digit',
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit'
    })
}
</script>

<style scoped src="./ChatView.css"></style>
