# 前端知识库（frontend-knowledge）

位置：`ZYTool/`（Vue 3 + TS + Vite + Ant Design Vue 4.x，开发端口 5050）

## 资源索引

- `resources/05-ui-animation-design.md` — UI 动画与设计规范（摘要速查）：动画取值标准（easing/时长/弹簧）、常用配方、评审标准、Apple 设计原则、库选型、反模式清单。涉及动效编写/评审/组件选型时必读。
- `resources/ui-animation-skills/` — 上述规范的**完整中文版**（00-索引 ~ 10，共 11 个文件），自包含、可直接移植到其他项目；需要完整规则细节时按 `resources/ui-animation-skills/00-索引.md` 定位。
- `../backend-knowledge/common/03-route-coverage.md` — 后端完整路由与 Python 降级覆盖；Python 仅部分兜底。

## 目录地图

```
src/
├── views/
│   ├── Agents/      # AI 助手：ChatView.vue/.ts/.css 三件套
│   ├── ViewFront/   # Json/Base64/Url/ColorPicker 复用 frontendTools.ts；Timestamp 在组件内用 dayjs/Date
│   ├── ViewBack/    # Diff 调后端；RouteMap 用高德浏览器能力并调后端；SqlRationality 为纯前端静态检查
│   ├── MES/         # GCPMRuleView.vue/.ts/.css + Mock/GCPMRuleData.ts（Mock 演示）
│   └── HomeView / ToolView / LoginView / SettingsView
├── services/
│   ├── api.ts           # rustApi/pythonApi + requestWithFallback + ApiService（唯一后端入口）
│   └── frontendTools.ts # 纯前端 JSON/Base64/URL/RGB/HEX 函数（按需命名导出，勿加默认导出）
├── config/appConfig.ts  # 双后端地址、apiPrefix、高德 Key
├── router/index.ts      # 路由 + beforeEach 认证守卫（meta.requiresAuth）
├── styles/index.css     # variables → base → components → utilities
├── Mock/                # Mock 数据与类型（ChatData/GCPMRuleData/SqlRation）
└── utils/
    ├── auth.ts          # Token/UserInfo 存取、BaseInfo 读取/清理/默认构建、clearAuth
    └── startVoiceInput.ts # Web Speech API 语音输入
```

目录名不等于运行时分类：SQL 工具虽位于 ViewBack，但不连接数据库、不调后端；`MapLocationView.vue` 当前未挂载路由，不是可访问工具页面。

## 路由与访问控制

| 路径 | 页面 |
|---|---|
| `/` | 登录/注册/QQ OAuth 回调（不是 `/login`） |
| `/home`、`/tools`、`/settings` | 首页、工具中心、设置 |
| `/tools/json`、`/tools/base64`、`/tools/url`、`/tools/color`、`/tools/timestamp` | 五个 ViewFront 工具 |
| `/tools/diff`、`/tools/sql`、`/tools/map` | Diff、SQL 静态检查、RouteMap |
| `/mes/gcpm-rule` | GCPM Mock 演示 |
| `/agents/chat` | AI 助手 |
| `/:pathMatch(.*)*` | 重定向 `/` |

当前所有业务路由均为 `requiresAuth: false`，页面均可直接访问；守卫只是预留机制。`isAuthenticated()` 只检查 token 是否存在，不校验过期时间，最终由接口 401 清理登录态。

## 服务层行为

- 后端请求一律走 `ApiService` 静态方法，禁止组件内直接 new Axios；`ApiService` 只保留有调用方的方法，新增接口时同步添加类型定义
- 两个 Axios 实例超时均为 60s，自动附带 `Authorization` 和 `X-Base-Info`
- 普通请求仅在网络错误/超时/5xx 时降级 Python，4xx 不降级；统一响应存在 `success` 和 `data` 时返回业务 `data`，否则返回原始结构
- 401 会清除认证并跳转 `/?redirect=...`
- 流式聊天有独立的降级和 SSE 分行解析逻辑，支持 `AbortSignal`：`ApiService.chatStream(message, sessionId?, signal?, modelId?)`

## 关键约定

- **前端组件优先使用 Ant Design Vue（4.x）**：新增 UI 组件先查 antd，能用 antd 实现的不手写、不引入其他组件库
- **动效必须走全局 token**（`src/styles/variables.css`）：`--ease-out/--ease-in-out/--ease-drawer` + `--duration-press(160ms)/fast(150ms)/base(200ms)/slow(250ms)/enter(300ms)`；禁止 `transition: all`、UI 禁 `ease-in`；按压反馈 `:active { transform: scale(0.97) }`；hover 动效用 `@media (hover: hover) and (pointer: fine)` 门控；reduced-motion 降级非归零；全局按压/reduced-motion 基座在 `styles/utilities.css`
- 组件逻辑复用：`XxxView.vue` + `XxxView.ts`（`useXxxView()`）+ `XxxView.css` 三件套，参照 Agents/ 与 MES/
- 新工具除页面、路由和 API 外，还需检查后端 `/tools/categories` 数据及 `ToolView.vue` 中按**工具名称**匹配的 `routeMap`/`backendRouteMap`；未映射只显示“开发中”，后端工具跳转前还会执行 `healthCheck()`
- Vite 仅配置 Vue 插件和 `@ → src` alias，没有 dev proxy；开发端口来自 package script，history 路由部署需 SPA fallback
- 当前没有 test、lint 或独立 typecheck 脚本，`yarn build` 是唯一类型检查入口；`start` 脚本存在 `yarn rnu dev` 拼写错误，使用 `yarn dev`

## 主题与样式

- 样式入口顺序固定为 `variables → base → components → utilities`；`base.css`、`components.css` 同时承担 Ant Design 深色适配
- 浅色/深色通过 `:root[data-theme="dark"]` 切换；主题存于 localStorage `zytool-theme`，无存储值时跟随系统偏好，并通过 `provide/inject` 共享
- Ant Design Vue 在 `main.ts` 全量注册并引入 reset.css

## 登录、绑定与设置

- 账号密码登录/注册后保存 token，再请求当前用户；QQ 登录跳转授权 URL，回调仍在 `/`，从 query `code` 完成登录
- 设置页的手机号、QQ、微信绑定是独立接口调用；QQ/微信绑定手填 openid 等字段，不是再次走 OAuth。微信没有 OAuth 登录实现
- 设置页路由公开与其接口需要认证是当前并存现状，不要仅凭页面可访问判断接口公开

## AI 助手页功能点

- 会话按后端返回的 `session.date` 字符串分组；支持新建/删除/双击重命名、消息和会话列表 loading
- 模型下拉来自 `GET /agents/models`，默认 enabled 项；发送时随 `model_id` 上送，实际提供方由后端配置决定，首页 DeepSeek 静态文案不是实现依据
- 流式输出打字光标、停止生成（AbortController）、语音输入（Web Speech API，`zh-CN`、单次识别，依赖浏览器兼容与麦克风权限）
- “清空对话”只清空当前本地消息数组，不删除后端消息

## MES 当前范围

GCPM 页面使用 `Mock/GCPMRuleData`，当前实现规则表格、三种流程详情、主规则树和缩放交互；新增/编辑/删除/刷新/搜索等多数操作仅提示，不调用后端，持久化 CRUD 与后端接口尚未实现。
