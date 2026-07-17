 // 工具集合

 export class ToolsFuntions {
    
    // ========== 语音相关 ==========

    // AI 聊天 - 语音输入（浏览器原生 Web Speech API）
    static startVoiceInput(
        onResult: (text: string) => void,
        onError?: (error: string) => void
    ): () => void {
        const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition
        if (!SpeechRecognition) {
            onError?.('当前浏览器不支持语音输入')
            return () => {}
        }

        const recognition = new SpeechRecognition()
        recognition.lang = 'zh-CN'
        recognition.continuous = false
        recognition.interimResults = false

        recognition.onresult = (event: any) => {
            const text = event.results[0][0].transcript
            onResult(text)
        }

        recognition.onerror = (event: any) => {
            onError?.(event.error === 'not-allowed' ? '麦克风权限被拒绝' : `语音识别错误: ${event.error}`)
        }

        recognition.start()

        // 返回停止函数
        return () => recognition.stop()
    }
}