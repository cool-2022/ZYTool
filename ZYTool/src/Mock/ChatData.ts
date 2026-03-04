import { ref } from 'vue'

export interface ChatMessage {
    id: string
    role: 'user' | 'assistant' | 'system'
    content: string
    timestamp: Date
}

export interface ChatSession {
    id: string
    title: string
    date: string
    messages: ChatMessage[]
}

// 聊天会话列表模拟数据
export const chatSessions = ref<ChatSession[]>([
    {
        id: '1',
        title: '开放式对话',
        date: '2026-01',
        messages: []
    },
    {
        id: '2',
        title: '新对X-code中可以使用图标',
        date: '2025-12',
        messages: []
    },
    {
        id: '3',
        title: '基文翻译：Transfer',
        date: '2025-12',
        messages: []
    },
    {
        id: '4',
        title: '条件运算符提供示例',
        date: '2025-11',
        messages: []
    },
    {
        id: '5',
        title: '3G无线',
        date: '2025-11',
        messages: []
    },
    {
        id: '6',
        title: 'ping命令无法使用端口',
        date: '2025-10',
        messages: []
    },
    {
        id: '7',
        title: 'Project Naming Constraints an...',
        date: '2025-10',
        messages: []
    },
    {
        id: '8',
        title: 'ASP.NET Core自动化性能分析工具',
        date: '2025-10',
        messages: []
    },
    {
        id: '9',
        title: '长效性与业务开发工具链',
        date: '2025-10',
        messages: []
    },
    {
        id: '10',
        title: '如何构建高可靠 K8s Agent',
        date: '2025-10',
        messages: []
    },
    {
        id: '11',
        title: '王者 BATTLE：AX 9750M-E Wi...',
        date: '2025-09',
        messages: []
    }
])
