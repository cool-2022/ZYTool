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
    message_count: number
    total_tokens: number
    model_id?: number
    updated_at: string
}
