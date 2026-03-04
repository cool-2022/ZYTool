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
                <template v-for="(group, date) in groupedSessions" :key="date">
                    <div class="session-group">
                        <div class="session-group-title">{{ date }}</div>
                        <div v-for="session in group" :key="session.id" class="session-item"
                            :class="{ active: currentSession?.id === session.id }" @click="selectSession(session)">
                            <div class="session-title">{{ session.title }}</div>
                            <a-button type="text" size="small" danger class="session-delete"
                                @click.stop="deleteSession(session.id)">
                                <DeleteOutlined />
                            </a-button>
                        </div>
                    </div>
                </template>
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
                <div v-if="messages.length === 0" class="chat-empty">
                    <div class="chat-empty-icon">
                        <CommentOutlined />
                    </div>
                    <div class="chat-empty-text">今天有什么可以帮到你?</div>
                    <div class="chat-empty-hint">输入问题开始对话</div>
                </div>

                <div v-for="msg in messages" :key="msg.id" class="message-item">
                    <div class="message-avatar" :class="msg.role">
                        <UserOutlined v-if="msg.role === 'user'" />
                        <RobotOutlined v-else />
                    </div>
                    <div class="message-content">
                        <div class="message-role">{{ msg.role === 'user' ? '你' : 'AI 助手' }}</div>
                        <div class="message-text">{{ msg.content }}</div>
                        <div class="message-time">{{ formatTime(msg.timestamp) }}</div>
                    </div>
                </div>

                <div v-if="isSending" class="message-item">
                    <div class="message-avatar assistant">
                        <RobotOutlined />
                    </div>
                    <div class="message-content">
                        <div class="message-role">AI 助手</div>
                        <div class="message-text">
                            <a-spin size="small" /> 正在思考...
                        </div>
                    </div>
                </div>
            </div>

            <div class="chat-input-area">
                <div class="chat-input-wrapper">
                    <a-textarea v-model:value="userInput" placeholder="输入消息..." :auto-size="{ minRows: 1, maxRows: 4 }"
                        @pressEnter="handleEnter" :disabled="isSending" />
                    <div class="chat-input-actions">
                        <a-button type="text" size="small">
                            <PaperClipOutlined />
                        </a-button>
                        <a-button type="text" size="small">
                            <AudioOutlined />
                        </a-button>
                        <a-button type="primary" @click="sendMessage" :loading="isSending" :disabled="!userInput.trim()">
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
import { computed } from 'vue'
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
    PaperClipOutlined,
    AudioOutlined
} from '@ant-design/icons-vue'
import { useChatView } from './ChatView'

const {
    chatSessions,
    currentSession,
    messages,
    userInput,
    isSending,
    sidebarCollapsed,
    selectSession,
    createNewSession,
    sendMessage,
    deleteSession,
    toggleSidebar,
    clearCurrentChat
} = useChatView()

// 按日期分组会话
const groupedSessions = computed(() => {
    const groups: Record<string, typeof chatSessions.value> = {}
    chatSessions.value.forEach(session => {
        if (!groups[session.date]) {
            groups[session.date] = []
        }
        groups[session.date].push(session)
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

// 处理 Enter 键
function handleEnter(e: KeyboardEvent) {
    if (e.shiftKey) {
        // Shift + Enter 换行，不做处理
        return
    }
    e.preventDefault()
    sendMessage()
}
</script>

<style scoped src="./ChatView.css"></style>
