/**
 * 应用入口（main.ts）
 * ZYTool 前端启动文件：创建 Vue 3 根实例，注册路由与 Ant Design Vue 4，
 * 并挂载到 index.html 的 #app 节点；全局样式（含 antd reset）在此统一引入。
 */
import { createApp } from 'vue'
import './styles/index.css'
import App from './App.vue'
import router from './router'
import Antd from 'ant-design-vue'
import 'ant-design-vue/dist/reset.css'

createApp(App)
    .use(router)
    .use(Antd)
    .mount('#app')
