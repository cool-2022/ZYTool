import { ref, nextTick } from 'vue'
import { message } from 'ant-design-vue'
import { chatSessions as mockChatSessions, type ChatMessage, type ChatSession } from '@/Mock/ChatData'

export type { ChatMessage, ChatSession }

export function useChatView() {
    // 聊天会话列表
    const chatSessions = mockChatSessions

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

    // 选择会话
    function selectSession(session: ChatSession) {
        currentSession.value = session
        messages.value = [...session.messages]
    }

    // 创建新会话
    function createNewSession() {
        const newSession: ChatSession = {
            id: Date.now().toString(),
            title: '新对话',
            date: new Date().toISOString().slice(0, 7),
            messages: []
        }
        chatSessions.value.unshift(newSession)
        selectSession(newSession)
        message.success('已创建新对话')
    }

    // 发送消息
    async function sendMessage() {
        if (!userInput.value.trim()) {
            message.warning('请输入消息内容')
            return
        }

        if (!currentSession.value) {
            createNewSession()
        }

        const userMessage: ChatMessage = {
            id: Date.now().toString(),
            role: 'user',
            content: userInput.value,
            timestamp: new Date()
        }

        messages.value.push(userMessage)
        
        // 如果是新会话，更新标题
        if (currentSession.value && currentSession.value.messages.length === 0) {
            currentSession.value.title = userInput.value.slice(0, 20) + (userInput.value.length > 20 ? '...' : '')
        }

        const inputContent = userInput.value
        userInput.value = ''
        isSending.value = true

        // 滚动到底部
        await nextTick()
        scrollToBottom()

        // 模拟 AI 回复
        setTimeout(() => {
            const assistantMessage: ChatMessage = {
                id: (Date.now() + 1).toString(),
                role: 'assistant',
                content: `这是对"${inputContent}"的回复。我是一个 AI 助手，可以帮助你解答问题。`,
                timestamp: new Date()
            }
            messages.value.push(assistantMessage)
            
            if (currentSession.value) {
                currentSession.value.messages = [...messages.value]
            }
            
            isSending.value = false
            
            // 滚动到底部
            nextTick(() => {
                scrollToBottom()
            })
        }, 1000)
    }

    // 滚动到底部
    function scrollToBottom() {
        const chatContainer = document.querySelector('.chat-messages-container')
        if (chatContainer) {
            chatContainer.scrollTop = chatContainer.scrollHeight
        }
    }

    // 删除会话
    function deleteSession(sessionId: string) {
        const index = chatSessions.value.findIndex(s => s.id === sessionId)
        if (index > -1) {
            chatSessions.value.splice(index, 1)
            if (currentSession.value?.id === sessionId) {
                currentSession.value = null
                messages.value = []
            }
            message.success('已删除会话')
        }
    }

    // 切换侧边栏
    function toggleSidebar() {
        sidebarCollapsed.value = !sidebarCollapsed.value
    }

    // 清空当前对话
    function clearCurrentChat() {
        if (currentSession.value) {
            currentSession.value.messages = []
            messages.value = []
            message.success('已清空当前对话')
        }
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
        clearCurrentChat
    }
}
