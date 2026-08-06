/**
 * Token 认证工具
 * 提供 token 的存储、获取、删除功能
 */

const TOKEN_KEY = 'zy_tool_token'
const USER_INFO_KEY = 'zy_tool_user'
const BASE_INFO_KEY = 'zy_tool_base_info'

export interface BaseInfo {
  computer_name?: string
  ip?: string
  username?: string
  screen_name?: string
}

/**
 * 保存 token 到 localStorage
 */
export function setToken(token: string): void {
  localStorage.setItem(TOKEN_KEY, token)
}

/**
 * 从 localStorage 获取 token
 */
export function getToken(): string | null {
  return localStorage.getItem(TOKEN_KEY)
}

/**
 * 从 localStorage 删除 token
 */
export function removeToken(): void {
  localStorage.removeItem(TOKEN_KEY)
}

/**
 * 检查是否已登录（是否有 token）
 */
export function isAuthenticated(): boolean {
  return !!getToken()
}

/**
 * 保存用户信息
 */
export function setUserInfo(userInfo: any): void {
  localStorage.setItem(USER_INFO_KEY, JSON.stringify(userInfo))
}

/**
 * 获取用户信息
 */
export function getUserInfo(): any | null {
  const userInfo = localStorage.getItem(USER_INFO_KEY)
  return userInfo ? JSON.parse(userInfo) : null
}

/**
 * 删除用户信息
 */
export function removeUserInfo(): void {
  localStorage.removeItem(USER_INFO_KEY)
}

/**
 * 从 localStorage 获取基础信息
 */
export function getBaseInfo(): BaseInfo | null {
  const baseInfo = localStorage.getItem(BASE_INFO_KEY)
  return baseInfo ? JSON.parse(baseInfo) : null
}

/**
 * 删除基础信息
 */
export function removeBaseInfo(): void {
  localStorage.removeItem(BASE_INFO_KEY)
}

/**
 * 构建默认基础信息（用于请求头）
 */
export function buildDefaultBaseInfo(): BaseInfo {
  const userInfo = getUserInfo()
  return {
    computer_name: '',
    ip: '',
    username: userInfo?.username || '',
    screen_name: typeof document !== 'undefined' ? document.title : '',
  }
}

/**
 * 清除所有认证信息
 */
export function clearAuth(): void {
  removeToken()
  removeUserInfo()
  removeBaseInfo()
}
