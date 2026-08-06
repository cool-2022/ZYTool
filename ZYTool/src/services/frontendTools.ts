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
