import { createRouter, createWebHistory } from 'vue-router'
import { isAuthenticated } from '@/utils/auth'
import ToolView from '@/views/ToolView.vue'
import HomeView from '@/views/HomeView.vue'

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/home',
            name: 'home',
            component: HomeView,
            meta: { requiresAuth: false } // 首页不需要登录
        },
        {
            path: '/',
            name: 'login',
            component: () => import('@/views/LoginView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools',
            name: 'tools',
            component: ToolView,
            meta: { requiresAuth: false } // 根据需求设置是否需要登录
        },
        {
            path: '/tools/json',
            name: 'json-tool',
            component: () => import('@/views/ViewFront/JsonToolView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/base64',
            name: 'base64-tool',
            component: () => import('@/views/ViewFront/Base64ToolView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/url',
            name: 'url-tool',
            component: () => import('@/views/ViewFront/UrlToolView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/color',
            name: 'color-picker',
            component: () => import('@/views/ViewFront/ColorPickerView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/timestamp',
            name: 'timestamp-tool',
            component: () => import('@/views/ViewFront/TimestampToolView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/diff',
            name: 'diff-tool',
            component: () => import('@/views/ViewBack/DiffToolView.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/sql',
            name: 'sql-tool',
            component: () => import('@/views/ViewBack/SqlRationality.vue'),
            meta: { requiresAuth: false }
        },
        {
            path: '/tools/map',
            name: 'map-location',
            component: () => import('@/views/ViewBack/MapLocationView.vue'),
            meta: { requiresAuth: false }
        }
    ]
})

// 路由守卫 - 控制页面访问权限
router.beforeEach((to, from, next) => {
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
