/**
 * 路由注册中心（router/index.ts）
 * 采用 routeDefs 元数据表 + import.meta.glob 动态收集 views 组件的注册方式：
 * 新增页面只需在 routeDefs 中登记一行（view 指向 views/ 下相对路径，懒加载）；
 * 首页/工具中心/设置等入口页用静态 import，避免首屏闪烁；
 * 全局前置守卫依据 meta.requiresAuth 拦截未登录访问。
 */
import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { isAuthenticated } from '@/utils/auth'
import ToolView from '@/views/ToolView.vue'
import HomeView from '@/views/HomeView.vue'
import SettingsView from '@/views/SettingsView.vue'

// Vite 一次性收集 views 下全部组件（懒加载），key 形如 ../views/ViewFront/JsonToolView.vue
const viewModules = import.meta.glob('../views/**/*.vue')

/** 按 view 相对路径从 glob 结果中取懒加载组件；未登记的文件回退首页并在控制台报错 */
function viewComponent(path: string): RouteRecordRaw['component'] {
    const key = `../views/${path}`
    const loader = viewModules[key]
    if (!loader) {
        console.error(`[router] 未找到组件: ${key}`)
        return () => import('@/views/HomeView.vue')
    }
    return loader as RouteRecordRaw['component']
}

interface RouteDef {
    path: string
    name?: string
    /** views/ 下相对路径（如 ViewFront/JsonToolView.vue）；与 component/redirect 三选一 */
    view?: string
    /** 静态导入的组件（首页/工具中心/设置等入口页，避免首屏闪烁） */
    component?: RouteRecordRaw['component']
    redirect?: string
    requiresAuth?: boolean
}

// 路由元数据表：新增页面只需在此登记一行（顺序即注册顺序，通配 404 必须在最后）
const routeDefs: RouteDef[] = [
    { path: '/home', name: 'home', component: HomeView },
    { path: '/', name: 'login', view: 'LoginView.vue' },
    { path: '/tools', name: 'tools', component: ToolView },
    { path: '/tools/json', name: 'json-tool', view: 'ViewFront/JsonToolView.vue' },
    { path: '/tools/base64', name: 'base64-tool', view: 'ViewFront/Base64ToolView.vue' },
    { path: '/tools/url', name: 'url-tool', view: 'ViewFront/UrlToolView.vue' },
    { path: '/tools/color', name: 'color-picker', view: 'ViewFront/ColorPickerView.vue' },
    { path: '/tools/timestamp', name: 'timestamp-tool', view: 'ViewFront/TimestampToolView.vue' },
    { path: '/tools/diff', name: 'diff-tool', view: 'ViewBack/DiffToolView.vue' },
    { path: '/tools/sql', name: 'sql-tool', view: 'ViewBack/SqlRationality.vue' },
    { path: '/tools/map', name: 'map-location', view: 'ViewBack/RouteMapView.vue' },
    { path: '/mes/gcpm-rule', name: 'gcpm-rule', view: 'MES/GCPMRuleView.vue' },
    { path: '/agents/chat', name: 'agents-chat', view: 'Agents/ChatView.vue' },
    { path: '/finance/ledger', name: 'finance-ledger', view: 'Finance/LedgerView.vue' },
    // 后台管理需要登录，接口侧再校验 admin 角色
    { path: '/admin', name: 'admin', view: 'Admin/AdminView.vue', requiresAuth: true },
    { path: '/admin/database', redirect: '/admin' },
    { path: '/settings', name: 'settings', component: SettingsView },
    { path: '/:pathMatch(.*)*', name: 'not-found', redirect: '/' },
]

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [],
})

// 遍历元数据表动态注册（redirect 路由无 component；未声明 requiresAuth 默认 false）
for (const def of routeDefs) {
    router.addRoute({
        path: def.path,
        name: def.name,
        component: def.redirect ? undefined : def.component ?? viewComponent(def.view!),
        redirect: def.redirect,
        meta: { requiresAuth: def.requiresAuth ?? false },
    } as RouteRecordRaw)
}

// 路由守卫 - 控制页面访问权限
router.beforeEach((to, _from, next) => {
    // 检查路由是否需要认证
    if (to.meta.requiresAuth && !isAuthenticated()) {
        // 需要登录但未登录，跳转到登录页
        next({
            name: 'login',
            query: { redirect: to.fullPath } // 保存目标路由，登录后跳转
        })
    } else {
        // 正常访问
        next()
    }
})

export default router
