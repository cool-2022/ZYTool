<template>
    <div class="chat-view-page">
        <!-- 左侧边栏 -->
        <div class="chat-sidebar" :class="{ collapsed: sidebarCollapsed }">
            <div class="sidebar-header">
                <div class="sidebar-logo">
                    <RobotOutlined />
                    <span>AI 助手</span>
                </div>
                <div class="sidebar-actions">
                    <a-button type="text" size="small" @click="createNewSession">
                        <PlusOutlined />
                    </a-button>
                </div>
            </div>

            <div class="sidebar-content">
                <a-spin :spinning="loadingSessions">
                    <template v-for="(group, date) in groupedSessions" :key="date">
                        <div class="session-group">
                            <div class="session-group-title">{{ date }}</div>
                            <div v-for="session in group" :key="session.id" class="session-item"
                                :class="{ active: currentSession?.id === session.id }" @click="selectSession(session)">
                                <a-input v-if="editingSessionId === session.id" v-model:value="editingTitle"
                                    size="small" class="session-rename-input" @click.stop
                                    @press-enter="confirmRename(session.id)" @blur="confirmRename(session.id)" />
                                <div v-else class="session-title" title="双击重命名"
                                    @dblclick.stop="startRename(session)">{{ session.title }}</div>
                                <a-button type="text" size="small" danger class="session-delete"
                                    @click.stop="deleteSession(session.id)">
                                    <DeleteOutlined />
                                </a-button>
                            </div>
                        </div>
                    </template>
                </a-spin>
            </div>
        </div>

        <!-- 主聊天区域 -->
        <div class="chat-main">
            <div class="chat-header">
                <div style="display: flex; align-items: center; gap: 12px;">
                    <a-button type="text" @click="toggleSidebar">
                        <MenuOutlined v-if="sidebarCollapsed" />
                        <MenuFoldOutlined v-else />
                    </a-button>
                    <span class="chat-title">{{ currentSession?.title || '今天有什么可以帮到你?' }}</span>
                </div>
                <a-space>
                    <a-button type="text" @click="clearCurrentChat" v-if="messages.length > 0">
                        <ClearOutlined />
                        清空对话
                    </a-button>
                </a-space>
            </div>

            <div class="chat-messages-container">
                <a-spin :spinning="loadingMessages" tip="加载消息中...">
                    <div v-if="messages.length === 0 && !loadingMessages" class="chat-empty">
                        <div class="chat-empty-icon">
                            <CommentOutlined />
                        </div>
                        <div class="chat-empty-text">今天有什么可以帮到你?</div>
                        <div class="chat-empty-hint">输入问题开始对话</div>
                    </div>

                    <div v-for="msg in messages" :key="msg.id" class="message-item" :class="{ self: msg.role === 'user' }">
                        <div class="message-avatar" :class="msg.role">
                            <UserOutlined v-if="msg.role === 'user'" />
                            <RobotOutlined v-else />
                        </div>
                        <div class="message-content">
                            <div class="message-role">{{ msg.role === 'user' ? '你' : 'AI 助手' }}</div>
                            <div class="message-bubble">
                                <div v-if="isStreamingMessage(msg) && !msg.content" class="thinking-indicator">
                                    <img :src="tomRunningGif" alt="正在思考" class="thinking-gif" />
                                    <span class="thinking-text">正在思考</span>
                                </div>
                                <div v-else class="message-text" :class="{ streaming: isStreamingMessage(msg) }">{{ msg.content }}</div>
                            </div>
                            <div class="message-time">{{ formatTime(msg.timestamp) }}</div>
                        </div>
                    </div>
                </a-spin>
            </div>

            <div class="chat-input-area">
                <div class="chat-input-wrapper">
                    <a-textarea v-model:value="userInput" placeholder="输入消息..." :auto-size="{ minRows: 1, maxRows: 4 }"
                        @keydown="handleInputKeydown" />
                    <div class="chat-input-actions">
                        <a-button type="text" size="small">
                            <PaperClipOutlined />
                        </a-button>
                        <a-button type="text" size="small" @click="toggleVoice" :class="{ 'voice-active': isListening }">
                            <AudioOutlined />
                        </a-button>
                        <a-button v-if="isSending" danger @click="stopStreaming">
                            <StopOutlined />
                            停止
                        </a-button>
                        <a-button v-else type="primary" @click="sendMessage" :disabled="!userInput.trim()">
                            <SendOutlined />
                        </a-button>
                    </div>
                </div>
                <div class="chat-input-hint">
                    按 Enter 发送消息，Shift + Enter 换行
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import {
    RobotOutlined,
    PlusOutlined,
    DeleteOutlined,
    MenuOutlined,
    MenuFoldOutlined,
    ClearOutlined,
    CommentOutlined,
    UserOutlined,
    SendOutlined,
    StopOutlined,
    PaperClipOutlined,
    AudioOutlined
} from '@ant-design/icons-vue'
import { message } from 'ant-design-vue'
import { useChatView } from './ChatView'
import { ToolsFuntions } from '@/utils/startVoiceInput'
import tomRunningGif from '@/assets/tom-running.gif'

const {
    chatSessions,
    currentSession,
    messages,
    userInput,
    isSending,
    sidebarCollapsed,
    loadingSessions,
    loadingMessages,
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

// 会话重命名状态
const editingSessionId = ref<string | null>(null)
const editingTitle = ref('')

function startRename(session: { id: string; title: string }) {
    editingSessionId.value = session.id
    editingTitle.value = session.title
}

async function confirmRename(sessionId: string) {
    if (editingSessionId.value !== sessionId) return
    const newTitle = editingTitle.value
    editingSessionId.value = null
    const session = chatSessions.value.find((s) => s.id === sessionId)
    if (!session || session.title === newTitle.trim()) return
    await renameSession(sessionId, newTitle)
}

// 输入框按键处理：Enter 发送，Shift + Enter 换行
function handleInputKeydown(e: KeyboardEvent) {
    if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault()
        sendMessage()
    }
}

// 判断是否为正在流式输出的助手消息（显示打字光标）
function isStreamingMessage(msg: { id: string; role: string }) {
    if (!isSending.value || msg.role !== 'assistant') return false
    const last = messages.value[messages.value.length - 1]
    return last?.id === msg.id
}

// 语音输入状态
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

// 按日期分组会话
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

// 格式化时间
function formatTime(date: Date) {
    const now = new Date()
    const diff = now.getTime() - date.getTime()
    const minutes = Math.floor(diff / 60000)
    
    if (minutes < 1) return '刚刚'
    if (minutes < 60) return `${minutes}分钟前`
    
    const hours = Math.floor(minutes / 60)
    if (hours < 24) return `${hours}小时前`
    
    return date.toLocaleString('zh-CN', { 
        month: '2-digit', 
        day: '2-digit', 
        hour: '2-digit', 
        minute: '2-digit' 
    })
}

</script>

<style scoped src="./ChatView.css"></style>
