import { ref, nextTick } from 'vue'
import { message } from 'ant-design-vue'
import { ApiService } from '@/services/api'
import type { ChatMessage, ChatSession } from '@/Mock/ChatData'

export function useChatView() {
    // 聊天会话列表
    const chatSessions = ref<ChatSession[]>([])
    const loadingSessions = ref(false)

    // 当前选中的会话
    const currentSession = ref<ChatSession | null>(null)

    // 当前消息列表
    const messages = ref<ChatMessage[]>([])
    const loadingMessages = ref(false)

    // 用户输入
    const userInput = ref('')

    // 是否正在发送
    const isSending = ref(false)

    // 流式请求中止控制器
    let abortController: AbortController | null = null

    // 侧边栏是否折叠
    const sidebarCollapsed = ref(false)

    // 加载会话列表
    async function loadSessions() {
        loadingSessions.value = true
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
        } finally {
            loadingSessions.value = false
        }
    }

    // 选择会话
    async function selectSession(session: ChatSession) {
        currentSession.value = session
        await loadMessages(session.id)
    }

    // 加载会话消息
    async function loadMessages(sessionId: string) {
        loadingMessages.value = true
        try {
            const response = await ApiService.getChatMessages(sessionId)
            messages.value = response.messages.map((m) => ({
                id: m.id,
                role: m.role as 'user' | 'assistant' | 'system',
                content: m.content,
                timestamp: new Date(m.created_at),
            }))
            await nextTick()
            scrollToBottom()
        } catch (error: any) {
            console.error('加载消息失败:', error)
            message.error('加载消息失败')
            messages.value = []
        } finally {
            loadingMessages.value = false
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
        // 生成期间不允许重复发送（避免并发竞态）
        if (isSending.value) {
            message.warning('正在生成回复，请稍候')
            return
        }

        // 提前捕获并清空输入，防止 await 期间被并发调用读到空值
        const inputContent = userInput.value.trim()
        if (!inputContent) {
            message.warning('请输入消息内容')
            return
        }
        userInput.value = ''

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
            content: inputContent,
            timestamp: new Date(),
        }

        messages.value.push(userMessage)

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

        // 调用后端流式 API（可中止）
        abortController = new AbortController()
        let stopped = false
        try {
            for await (const chunk of ApiService.chatStream(inputContent, sessionId, abortController.signal)) {
                const lastMsg = messages.value[messages.value.length - 1]
                if (lastMsg && lastMsg.role === 'assistant') {
                    lastMsg.content += chunk
                }

                await nextTick()
                scrollToBottom()
            }

            stopped = abortController?.signal.aborted ?? false
            if (stopped && !assistantMessage.content) {
                assistantMessage.content = '*已停止生成*'
            }

            // 刷新当前会话的消息和列表（同步后端标题、消息数等）
            if (!stopped) {
                await refreshCurrentSession()
            }
        } catch (error: any) {
            // #region debug-point E:sendMessage-error
            fetch('http://127.0.0.1:7777/event', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify({ sessionId: 'ai-chat-network-error', runId: 'pre-fix', hypothesisId: 'E', location: 'ChatView.ts:sendMessage-error', msg: '[DEBUG] sendMessage caught error', data: { message: error?.message, name: error?.name, stack: error?.stack?.slice(0, 500) }, ts: Date.now() }) }).catch(() => {})
            // #endregion
            assistantMessage.content = `错误: ${error?.message || '发送消息失败'}`
            console.error('Chat error:', error)
        } finally {
            isSending.value = false
            abortController = null
            nextTick(() => {
                scrollToBottom()
            })
        }
    }

    // 停止生成
    function stopStreaming() {
        abortController?.abort()
    }

    // 重命名会话
    async function renameSession(sessionId: string, title: string) {
        const trimmed = title.trim()
        if (!trimmed) {
            message.warning('会话标题不能为空')
            return false
        }
        try {
            await ApiService.updateChatSessionTitle(sessionId, trimmed)
            const session = chatSessions.value.find((s) => s.id === sessionId)
            if (session) {
                session.title = trimmed
            }
            if (currentSession.value?.id === sessionId) {
                currentSession.value.title = trimmed
            }
            message.success('标题已更新')
            return true
        } catch (error: any) {
            console.error('重命名会话失败:', error)
            message.error('重命名会话失败')
            return false
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
        loadSessions,
        init,
    }
}
