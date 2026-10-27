# 前端知识库（frontend-knowledge）

位置：`ZYTool/`（Vue 3 + TS + Vite + Ant Design Vue 4.x，开发端口 5050）

## 资源索引

- `resources/05-ui-animation-design.md` — UI 动画与设计规范（摘要速查）：动画取值标准（easing/时长/弹簧）、常用配方、评审标准、Apple 设计原则、库选型、反模式清单。涉及动效编写/评审/组件选型时必读。
- `resources/ui-animation-skills/` — 上述规范的**完整中文版**（00-索引 ~ 10，共 11 个文件），自包含、可直接移植到其他项目；需要完整规则细节时按 `resources/ui-animation-skills/00-索引.md` 定位。

## 目录地图

```
src/
├── views/
│   ├── Agents/      # AI 助手：ChatView.vue（模板/脚本）、ChatView.ts（useChatView 逻辑）、
│   │                #   ChatView.css（scoped 样式，<style src> 引入）
│   ├── ViewFront/   # 浏览器端工具：Json/Base64/Url/ColorPicker/Timestamp（走 frontendTools.ts，不调后端）
│   ├── ViewBack/    # 服务器端工具：Diff/SqlRationality/RouteMap（调 ApiService）
│   ├── MES/         # MES 页面：GCPMRuleView.vue/.ts/.css + Mock/GCPMRuleData.ts
│   └── HomeView / ToolView / LoginView / SettingsView
├── services/
│   ├── api.ts           # rustApi/pythonApi + requestWithFallback + ApiService（唯一后端入口）
│   └── frontendTools.ts # 纯前端工具函数（按需命名导出，勿加默认导出）
├── config/appConfig.ts  # AppConfig：双后端地址、apiPrefix、高德 Key
├── router/index.ts      # 路由 + beforeEach 认证守卫（meta.requiresAuth）
├── Mock/                # Mock 数据与类型（ChatData/GCPMRuleData/SqlRation）
└── utils/
    ├── auth.ts          # Token/UserInfo/BaseInfo 存取、clearAuth、buildDefaultBaseInfo
    └── startVoiceInput.ts # 语音输入（ToolsFuntions.startVoiceInput）
```

## 关键约定

- **前端组件优先使用 Ant Design Vue（4.x）**：新增 UI 组件先查 antd 组件库，能用 antd 实现的不手写、不引入其他组件库
- **动效必须走全局 token**（`src/styles/variables.css`）：`--ease-out/--ease-in-out/--ease-drawer` + `--duration-press(160ms)/fast(150ms)/base(200ms)/slow(250ms)/enter(300ms)`；禁止 `transition: all`、UI 禁 `ease-in`；按压反馈 `:active { transform: scale(0.97) }`；hover 动效用 `@media (hover: hover) and (pointer: fine)` 门控；reduced-motion 降级非归零（入场只留淡入）；全局按压/reduced-motion 基座在 `styles/utilities.css`
- 组件逻辑复用：`XxxView.vue` + `XxxView.ts`（`useXxxView()` 组合式函数）+ `XxxView.css` 三件套，参照 Agents/ 与 MES/
- 后端请求一律走 `ApiService` 静态方法，禁止组件内直接 new Axios
- `ApiService` 只保留有调用方的方法；新增接口时同步添加类型定义
- 流式聊天支持中止：`ApiService.chatStream(message, sessionId, signal)`
- 路由默认 `requiresAuth: false`；需要登录的页面显式设置 `meta: { requiresAuth: true }`

## AI 助手页功能点

会话分组（按 updated_at 的 YYYY-MM）、新建/删除/双击重命名会话、模型选择下拉（数据来自 GET /agents/models，默认 is_enabled 项，切换会话跟随 ai_sessions.model_id，发送时随 model_id 上送）、流式输出打字光标、停止生成（AbortController）、语音输入、消息/会话列表 loading。
