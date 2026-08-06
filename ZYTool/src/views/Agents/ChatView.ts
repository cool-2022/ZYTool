import { ref, nextTick } from 'vue'
import { message } from 'ant-design-vue'
import { ApiService } from '@/services/api'
import type { ChatMessage, ChatSession } from '@/Mock/ChatData'

export function useChatView() {
    // 聊天会话列表
    const chatSessions = ref<ChatSession[]>([])

    // 当前选中的会话
    const currentSession = ref<ChatSession | null>(null)

    // 当前消息列表
    const messages = ref<ChatMessage[]>([])

    // 用户输入
    const userInput = ref('')

    // 是否正在发送
    const isSending = ref(false)

    // 侧边栏是否折叠
    const sidebarCollapsed = ref(false)

    // 加载会话列表
    async function loadSessions() {
        try {
            const response = await ApiService.getChatSessions()
            chatSessions.value = response.sessions.map((s) => ({
                id: s.id,
                title: s.title,
                date: s.date,
                message_count: s.message_count,
                total_tokens: s.total_tokens,
                model_id: s.model_id,
                updated_at: s.updated_at,
                messages: [],
            }))
        } catch (error: any) {
            console.error('加载会话列表失败:', error)
            message.error('加载会话列表失败')
        }
    }

    // 选择会话
    async function selectSession(session: ChatSession) {
        currentSession.value = session
        await loadMessages(session.id)
    }

    // 加载会话消息
    async function loadMessages(sessionId: string) {
        try {
            const response = await ApiService.getChatMessages(sessionId)
            messages.value = response.messages.map((m) => ({
                id: m.id,
                role: m.role as 'user' | 'assistant' | 'system',
                content: m.content,
                timestamp: new Date(m.created_at),
            }))
        } catch (error: any) {
            console.error('加载消息失败:', error)
            message.error('加载消息失败')
            messages.value = []
        }
    }

    // 创建新会话
    async function createNewSession() {
        try {
            const session = await ApiService.createChatSession('新对话')
            const newSession: ChatSession = {
                id: session.id,
                title: session.title,
                date: session.date,
                message_count: session.message_count,
                total_tokens: session.total_tokens,
                model_id: session.model_id,
                updated_at: session.updated_at,
                messages: [],
            }
            chatSessions.value.unshift(newSession)
            await selectSession(newSession)
            message.success('已创建新对话')
        } catch (error: any) {
            console.error('创建会话失败:', error)
            message.error('创建会话失败')
        }
    }

    // 发送消息
    async function sendMessage() {
        if (!userInput.value.trim()) {
            message.warning('请输入消息内容')
            return
        }

        if (!currentSession.value) {
            await createNewSession()
        }

        const sessionId = currentSession.value?.id
        if (!sessionId) {
            message.error('当前会话无效')
            return
        }

        const userMessage: ChatMessage = {
            id: Date.now().toString(),
            role: 'user',
            content: userInput.value,
            timestamp: new Date(),
        }

        messages.value.push(userMessage)

        const inputContent = userInput.value
        userInput.value = ''
        await nextTick()
        isSending.value = true

        // 创建空的助手消息用于流式显示
        const assistantMessage: ChatMessage = {
            id: (Date.now() + 1).toString(),
            role: 'assistant',
            content: '',
            timestamp: new Date(),
        }
        messages.value.push(assistantMessage)

        // 滚动到底部
        await nextTick()
        scrollToBottom()

        // 调用后端流式 API
        try {
            for await (const chunk of ApiService.chatStream(inputContent, sessionId)) {
                const lastMsg = messages.value[messages.value.length - 1]
                if (lastMsg && lastMsg.role === 'assistant') {
                    lastMsg.content += chunk
                }

                await nextTick()
                scrollToBottom()
            }

            // 刷新当前会话的消息和列表（同步后端标题、消息数等）
            await refreshCurrentSession()
        } catch (error: any) {
            assistantMessage.content = `错误: ${error?.message || '发送消息失败'}`
            console.error('Chat error:', error)
        } finally {
            isSending.value = false
            nextTick(() => {
                scrollToBottom()
            })
        }
    }

    // 刷新当前会话数据
    async function refreshCurrentSession() {
        if (!currentSession.value) return
        try {
            const response = await ApiService.getChatMessages(currentSession.value.id)
            messages.value = response.messages.map((m) => ({
                id: m.id,
                role: m.role as 'user' | 'assistant' | 'system',
                content: m.content,
                timestamp: new Date(m.created_at),
            }))

            // 刷新列表中的当前会话元数据
            const sessionsResponse = await ApiService.getChatSessions()
            chatSessions.value = sessionsResponse.sessions.map((s) => ({
                id: s.id,
                title: s.title,
                date: s.date,
                message_count: s.message_count,
                total_tokens: s.total_tokens,
                model_id: s.model_id,
                updated_at: s.updated_at,
                messages: [],
            }))

            const updated = chatSessions.value.find((s) => s.id === currentSession.value?.id)
            if (updated) {
                currentSession.value = updated
            }
        } catch (error) {
            console.error('刷新会话失败:', error)
        }
    }

    // 滚动到底部
    function scrollToBottom() {
        const chatContainer = document.querySelector('.chat-messages-container')
        if (chatContainer) {
            chatContainer.scrollTop = chatContainer.scrollHeight
        }
    }

    // 删除会话
    async function deleteSession(sessionId: string) {
        try {
            await ApiService.deleteChatSession(sessionId)
            const index = chatSessions.value.findIndex((s) => s.id === sessionId)
            if (index > -1) {
                chatSessions.value.splice(index, 1)
                if (currentSession.value?.id === sessionId) {
                    currentSession.value = null
                    messages.value = []
                }
                message.success('已删除会话')
            }
        } catch (error: any) {
            console.error('删除会话失败:', error)
            message.error('删除会话失败')
        }
    }

    // 切换侧边栏
    function toggleSidebar() {
        sidebarCollapsed.value = !sidebarCollapsed.value
    }

    // 清空当前对话
    async function clearCurrentChat() {
        if (currentSession.value) {
            messages.value = []
            message.success('已清空当前对话')
        }
    }

    // 组件初始化时加载会话列表
    async function init() {
        await loadSessions()
    }

    return {
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
        clearCurrentChat,
        loadSessions,
        init,
    }
}
