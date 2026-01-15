import { createRouter, createWebHistory } from 'vue-router'
import ToolView from '@/views/ToolView.vue'
import HomeView from '@/views/HomeView.vue'
// import JsonToolView from '@/views/JsonToolView.vue'
// import Base64ToolView from '@/views/Base64ToolView.vue'
// import UrlToolView from '@/views/UrlToolView.vue'
// import ColorPickerView from '@/views/ColorPickerView.vue'
// import TimestampToolView from '@/views/TimestampToolView.vue'
// import DiffToolView from '@/views/DiffToolView.vue'


const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView
        },
        {
            path: '/tools',
            name: 'tools',
            component: ToolView
        },
        {
            path: '/tools/json',
            name: 'json-tool',
            // component: JsonToolView
            component: () => import('@/views/JsonToolView.vue')
        },
        {
            path: '/tools/base64',
            name: 'base64-tool',
            // component: Base64ToolView
            component: () => import('@/views/Base64ToolView.vue')
        },
        {
            path: '/tools/url',
            name: 'url-tool',
            // component: UrlToolView
            component: () => import('@/views/UrlToolView.vue')
        },
        {
            path: '/tools/color',
            name: 'color-picker',
            // component: ColorPickerView
            component: () => import('@/views/ColorPickerView.vue')
        },
        {
            path: '/tools/timestamp',
            name: 'timestamp-tool',
            // component: TimestampToolView
            component: () => import('@/views/TimestampToolView.vue')
        },
        {
            path: '/tools/diff',
            name: 'diff-tool',
            // component: DiffToolView
            component: () => import('@/views/DiffToolView.vue')
        },
        {
            path: '/tools/sql',
            name: 'sql-tool',
            component: () => import('@/views/SqlRationality.vue')

        },
        {
            path:'/Login',
            name: 'login',
            component:()=> import('@/views/Login.vue')
        }
    ]
})

export default router
