/**
 * 前端工具服务
 * 这些工具在浏览器端直接处理，无需后端支持
 */

// JSON格式化
export function formatJSON(text: string): { result: string; success: boolean } {
    try {
        const parsed = JSON.parse(text)
        const formatted = JSON.stringify(parsed, null, 2)
        return { result: formatted, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '格式化失败',
            success: false
        }
    }
}

// JSON压缩
export function minifyJSON(text: string): { result: string; success: boolean } {
    try {
        const parsed = JSON.parse(text)
        const minified = JSON.stringify(parsed)
        return { result: minified, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '压缩失败',
            success: false
        }
    }
}

// Base64编码（支持 Unicode 字符）
export function base64Encode(text: string): { result: string; success: boolean } {
    try {
        // 使用 TextEncoder 处理 Unicode 字符
        const encoder = new TextEncoder()
        const data = encoder.encode(text)

        // 将 Uint8Array 转换为二进制字符串
        let binaryString = ''
        for (let i = 0; i < data.length; i++) {
            binaryString += String.fromCharCode(data[i]!)
        }

        // 使用 btoa 编码
        const encoded = btoa(binaryString)
        return { result: encoded, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '编码失败',
            success: false
        }
    }
}

// Base64解码（支持 Unicode 字符）
export function base64Decode(text: string): { result: string; success: boolean } {
    try {
        // 使用 atob 解码
        const binaryString = atob(text)

        // 将二进制字符串转换为 Uint8Array
        const bytes = new Uint8Array(binaryString.length)
        for (let i = 0; i < binaryString.length; i++) {
            bytes[i] = binaryString.charCodeAt(i)
        }

        // 使用 TextDecoder 处理 Unicode 字符
        const decoder = new TextDecoder()
        const decoded = decoder.decode(bytes)
        return { result: decoded, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '解码失败',
            success: false
        }
    }
}

// URL编码
export function urlEncode(text: string): { result: string; success: boolean } {
    try {
        const encoded = encodeURIComponent(text)
        return { result: encoded, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '编码失败',
            success: false
        }
    }
}

// URL解码
export function urlDecode(text: string): { result: string; success: boolean } {
    try {
        const decoded = decodeURIComponent(text)
        return { result: decoded, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '解码失败',
            success: false
        }
    }
}

// 时间戳转换为日期时间
export function timestampToDatetime(timestamp: number): { result: string; success: boolean } {
    try {
        const date = new Date(timestamp)
        const result = date.toLocaleString('zh-CN', {
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false
        })
        return { result, success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '转换失败',
            success: false
        }
    }
}

// 日期时间转换为时间戳
export function datetimeToTimestamp(datetime: string): { result: string; success: boolean } {
    try {
        const timestamp = new Date(datetime).getTime()
        if (isNaN(timestamp)) {
            throw new Error('无效的日期格式')
        }
        return { result: timestamp.toString(), success: true }
    } catch (error) {
        return {
            result: error instanceof Error ? error.message : '转换失败',
            success: false
        }
    }
}

// 获取当前时间戳
export function getCurrentTimestamp(): number {
    return Date.now()
}

// RGB转HEX
export function rgbToHex(r: number, g: number, b: number): string {
    return '#' + [r, g, b].map(x => {
        const hex = x.toString(16)
        return hex.length === 1 ? '0' + hex : hex
    }).join('')
}

// HEX转RGB
export function hexToRgb(hex: string): { r: number; g: number; b: number } | null {
    const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex)
    return result ? {
        r: parseInt(result[1]!, 16),
        g: parseInt(result[2]!, 16),
        b: parseInt(result[3]!, 16)
    } : null
}

// 前端工具类型定义
export const FrontendTools = {
    formatJSON,
    minifyJSON,
    base64Encode,
    base64Decode,
    urlEncode,
    urlDecode,
    timestampToDatetime,
    datetimeToTimestamp,
    getCurrentTimestamp,
    rgbToHex,
    hexToRgb
}

export default FrontendTools
