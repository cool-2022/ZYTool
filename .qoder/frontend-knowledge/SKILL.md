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
│   ├── Finance/     # LedgerView.vue/.ts/.css：小账本，本地收支与月度预算
│   ├── Admin/       # AdminView 外壳（数据库/字根/服务三分类 Tab）+ DatabaseAdminView（表只读浏览）+ RootsAdminView（字根 CRUD）+ ServicesAdminView（双后端状态）
│   └── HomeView / ToolView / LoginView / SettingsView
├── services/
│   ├── api.ts           # rustApi/pythonApi + requestWithFallback + ApiService（唯一后端入口）
│   └── frontendTools.ts # 纯前端 JSON/Base64/URL/RGB/HEX 函数（按需命名导出，勿加默认导出）
├── config/appConfig.ts  # 双后端地址、apiPrefix、高德 Key
├── router/index.ts      # 动态注册路由：`import.meta.glob('../views/**/*.vue')` 收集组件（懒加载），routeDefs 元数据数组循环 addRoute；新页面只登记一行（view 为 views/ 相对路径）；beforeEach 认证守卫（meta.requiresAuth）
├── styles/index.css     # variables → base → components → utilities
├── Mock/                # Mock 数据与类型（ChatData/GCPMRuleData/SqlRation）
└── utils/
    ├── auth.ts          # Token/UserInfo 存取、BaseInfo 读取/清理/默认构建、clearAuth
    ├── startVoiceInput.ts # Web Speech API 语音输入
    ├── formHelper.ts    # EI 风格服务调用助手：SysInfoHelper 子类采集系统信息（时间/电脑名/IP/画面ID），
    │                    #   FormHelper.callService 支持 EIInfo 与普通 JSON，底层走 requestWithFallback
    └── ei/              # EI 平台 EiInfo 数据交换结构（type/ei-tools/ei/index）；
                         #   以 `import { EI } from '@/utils/ei'` 使用（new EI.EIInfo()），
                         #   与 Rust models/ei.rs 一一对应，CRUD 前后端统一按此结构传递
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
| `/finance/ledger` | 小账本（纯浏览器本地收支管理） |
| `/admin` | 后台管理（数据库/字根/服务三分类 Tab，requiresAuth；接口再校验 admin 角色，403 面板内提示）；`/admin/database` 重定向至此 |
| `/:pathMatch(.*)*` | 重定向 `/` |

除 `/admin/database`（requiresAuth: true）外，其余业务路由均为 `requiresAuth: false`，页面均可直接访问；守卫只是预留机制。`isAuthenticated()` 只检查 token 是否存在，不校验过期时间，最终由接口 401 清理登录态。

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
- 新工具除页面、路由和 API 外，还需在 `tools` 表登记数据（含 `route`）；`ToolView.vue` 跳转优先读工具数据里的 `route`，缺失时回退按**工具 ID** 的 `frontendRouteMap`/`backendRouteMap`；未映射只显示“开发中”，后端工具跳转前还会执行 `healthCheck()`
- 工具页额外追加了本地“财务工具”分类（ID 4），其中“小账本”（ID 16）作为前端工具直接打开 `/finance/ledger`；接口数据和备用数据都会合并该分类。分类加载完成后还会移除地图 ID 13/15 并补回唯一的 ID 13，避免后端进程数据未刷新导致重复或缺失
- Vite 仅配置 Vue 插件和 `@ → src` alias，没有 dev proxy；开发端口来自 package script，history 路由部署需 SPA fallback
- 当前没有 test、lint 或独立 typecheck 脚本，`yarn build` 是唯一类型检查入口；`start` 脚本存在 `yarn rnu dev` 拼写错误，使用 `yarn dev`

## 主题与样式

- 样式入口顺序固定为 `variables → base → components → formStyles → toolStyles → utilities`；`base.css`、`components.css` 同时承担 Ant Design 深色适配
- 后续所有新增画面保持暖色工作区风格，复用 `--workspace-*` 变量；`formStyles.css` 提供仅映射暖色变量的 `warm-theme`，以及 `warm-form-page`、`warm-form-card`、`warm-form`、`warm-form-submit`、`warm-form-secondary`、`warm-form-link` 和 `warm-form-note`。首页、工具、聊天、小账本、后台管理与顶栏通过 `warm-theme` 接入，登录和设置复用完整表单类，避免另起蓝紫或其他独立配色
- 所有工具详情页根节点使用 `.tool-page.warm-theme`，共享 `toolStyles.css` 的页面、卡片、控件、结果区、上传区、地图和响应式样式；差异行、颜色预览、SQL 三列布局等业务专属样式保留在页面内，新增工具页不得重新引入蓝紫卡片头或硬编码橙色文案
- `JsonToolView.vue` 使用顶部集中工具栏＋双栏等高编辑器布局，输入/结果卡片带字符计数，支持格式化、压缩、填入示例、复制与清空；小屏收为单列，保留浏览器本地处理
- 其他工具页沿用 JSON 页视觉结构：`toolStyles.css` 统一提供 `has-tool-badge`、`tool-badge`、`tool-action-bar`、`tool-action-group`、`tool-panel-row/col/card`、`tool-card-meta`、`text-tool-input/result` 等类。Base64、URL、SQL 使用双栏编辑器；时间、颜色、文本对比和地图使用集中操作栏＋业务卡片，保留各自转换、上传、定位与路线功能
- `RouteMapView.vue` 的起点区域使用独立“当前位置”按钮；定位成功后清除旧路线与标记，写入起点地址，并在地图上添加当前位置标记、以缩放 16 居中。按钮带 loading，路线信息卡提供暖色空状态
- 滚动条统一由 `utilities.css` 控制，使用 `variables.css` 的 `--scrollbar-size/--scrollbar-thumb/--scrollbar-thumb-hover`：WebKit/Chromium 横纵均为 4px、暖灰圆角滑块及透明轨道，Firefox 使用原生 thin；支持 WebKit 伪元素时将标准 scrollbar 属性重置为 auto，避免覆盖精确宽度。深色同步暖灰值，高对比度模式回退系统尺寸/配色；聊天区、工具和设置菜单不再单独覆盖滚动条。
- 浅色/深色通过 `:root[data-theme="dark"]` 切换；主题存于 localStorage `zytool-theme`，无存储值时跟随系统偏好，并通过 `provide/inject` 共享
- Ant Design Vue 在 `main.ts` 全量注册并引入 reset.css
- 页面布局统一使用 `variables.css` 的 `--page-max-width: 1600px` 与 `--page-gutter: 16px`；768px 及以下边距为 12px、480px 及以下为 10px。导航、首页、工具中心、设置、AI 与财务采用相同内容上限，避免大屏仍被 1200/1280px 容器限制；左右边距只由页面外层承担，内层 `.container` 不重复填充。
- 八个工具详情页的 `.tool-page` 使用页面边距变量、`.container` 共享最大宽度且 padding 为 0；JSON 页手机覆盖亦遵循同一规则。原 24px 栅格间距改为 `{ xs: 16, sm: 24 }`，防止手机边距收紧后行负边距越界。登录表单保留 440px 上限，外层边距及卡片内边距收紧；MES 保留已有全宽、零外层边距布局。
- 首页、工具中心、AI 助手、设置页、财务小账本与顶栏使用 `variables.css` 中独立的 `--workspace-*` 暖色设计 token（奶油白背景、珊瑚橙强调、鼠尾草绿/沙色辅助、13px 卡片圆角）；这些 token 有独立的深色值，不覆盖其他业务页面现有的蓝紫主题变量。
- `HomeView.vue` 使用局部 1280px 内容宽度：顶部为紧凑标题与工具/AI 按钮，不再使用右侧重复介绍卡；下方为工具、AI 两张主要入口卡及小账本横向入口，页尾仅保留简短说明。两张入口卡共享列宽，小屏收为单列横向摘要卡，小账本不省略入口；所有卡片均可通过 router-link 整卡访问，保留键盘焦点、按压与 reduced-motion，不修改路由。`ToolView.vue` 为紧凑标题、顶部单行分类菜单及下方全宽细边框工具卡，内容最大宽度统一由 `--page-max-width` 控制（1600px）；顶部采用与财务页一致的小图标＋文字组合：左侧浅橙底倾斜图标，右侧标题与简短说明上下排列；移除装饰性小标题，桌面标题 22px、图标 42×46px、顶部留白 20px、标题区底部留白 12px；分类菜单使用与财务页一致的浅橙选中态，手机端保持一行并可横向滑动，工具卡为单列布局。
- 工具分类使用原生 button，工具卡支持 Enter/空格激活与明确焦点；分类获取、名称到路由映射、后端健康检查流程不因样式调整而变动。

## 登录、绑定与设置

- 账号密码登录/注册后保存 token，再请求当前用户；QQ 登录跳转授权 URL，回调仍在 `/`，从 query `code` 完成登录
- `SettingsView.vue` 使用暖色工作区、共享的页面内容宽度及与工具页一致的小图标＋标题/说明；「账号信息／账号绑定／外观」为顶部单行原生按钮菜单，选中项浅橙色，手机端可横向滑动。内容使用 v-show 切换，保留未提交的绑定输入。
- 账号资料使用头像摘要及两列字段布局，绑定页为手机号/QQ/微信卡片（桌面三列、窄屏收为单列），字段有可关联标签，绑定状态来自现有接口；外观通过两张浅色/深色预览卡调用既有 toggleTheme，与顶栏及 localStorage 主题联动。局部 ConfigProvider 跟随全局 theme，色值与 `variables.css` 工作区 token 对应，调整配色时须同步。
- 设置页的手机号、QQ、微信绑定是独立接口调用；QQ/微信绑定手填 openid 等字段，不是再次走 OAuth。微信没有 OAuth 登录实现
- 设置页路由公开与其接口需要认证是当前并存现状，不要仅凭页面可访问判断接口公开

## AI 助手页功能点

- 入口包含全局顶栏「AI 助手」及首页 AI 链接，路由为 `/agents/chat`；顶栏顺序为首页、工具、AI 助手、财务，AI 链接与财务复用 nav-link 暖色样式及路由选中态。400px 及以下顶栏品牌仅显示图标（保留可访问名称），为新增菜单项留出空间，菜单文字不换行。
- 页面采用与工具页一致的紧凑小图标＋标题/说明，桌面标题 22px、图标 42×46px，手机端图标 36×40px；内容最大宽度使用 `--page-max-width`（1600px），248px 会话侧栏与聊天区置于细边框、13px 圆角工作区。移除蓝紫渐变、光斑和玻璃顶栏，欢迎区为暖色快捷提问卡，用户消息使用浅橙底，助手消息使用奶油白底；消息与输入共用局部 `--chat-content-width: 1040px` 和页面边距变量，减少两侧空白并保持一致的宽度约束。
- 样式使用 `--workspace-*`；局部 Ant Design ConfigProvider 通过注入全局 `theme` 同步浅色/深色，模型下拉与删除确认亦采用暖色。ConfigProvider 的色值与 `variables.css` 中工作区 token 对应，调整工作区配色时须同步。
- 会话按后端返回的 `session.date` 字符串分组；支持新建/删除/双击重命名、消息和会话列表 loading；会话项支持 Enter/空格选择，收起侧栏使用 inert 避免隐藏控件获得焦点。
- 折叠侧栏不再把展开入口移到页面右上角：工作区左侧保留 58px 窄工具栏，提供展开列表与独立新建对话；完整侧栏在同一位置展开，移动端仍为左侧抽屉，工具栏位于遮罩之下。
- 模型下拉来自 `GET /agents/models`，默认 enabled 项；发送时随 `model_id` 上送，实际提供方由后端配置决定，首页不再硬编码模型提供方；选择器位于输入区发送按钮旁
- 空状态提供快捷提问，助手消息支持复制，删除会话需二次确认；移动端默认收起侧栏，选择会话后自动收起
- 流式输出打字光标、停止生成（AbortController）、语音输入（Web Speech API，`zh-CN`、单次识别，依赖浏览器兼容与麦克风权限）
- “清空对话”只清空当前本地消息数组，不删除后端消息

## MES 当前范围

GCPM 页面使用 `Mock/GCPMRuleData`，当前实现规则表格、三种流程详情、主规则树和缩放交互；新增/编辑/删除/刷新/搜索等多数操作仅提示，不调用后端，持久化 CRUD 与后端接口尚未实现。

## 财务小账本

- 页面位于 `views/Finance/LedgerView.vue/.ts/.css`，入口为顶栏「财务」及 `/finance/ledger`；不要求登录、不调用后端、不连接银行账户。

## 后台管理

- 外壳 `views/Admin/AdminView.vue/.ts/.css`，路由 `/admin`（旧 `/admin/database` 重定向）；顶栏「管理」入口仅 `userInfo.roles` 含 `admin` 时显示（App.vue `isAdmin`）。
- **统一表格复用件 `src/styles/gridView/` 模块**（GridView.vue + gridView.css + index.ts，插槽式插件，`import GridView from '@/styles/gridView'`）：分页条固定在表头上方（`total` 传 null 则隐藏）、表头吸附滚动（`stickyTop` 视口偏移）、列宽拖拽（表头右侧手柄，Pointer Events）、列级筛选（列配置加 `filterable: true`，表头漏斗下拉输入，客户端仅筛当前页）；`bodyCell/expandedRowRender/emptyText` 插槽透传。后续管理类表格一律用它，不再直接写 a-table。
- **固定区布局**：分类菜单（顶 56px 吸附）→ 面板工具条（104px）→ 分页条（156px）→ 表头（202px）逐级吸附，滚动时红框区域不动；≤768px 工具条/分页条降级为随内容滚动。
- 顶部单行原生按钮分类菜单：数据库管理 / 字根管理 / 服务管理；面板首次切换到时才挂载（visited 集合），之后 v-show 切换保留状态。页面级 `--admin-*` 变量、antd 覆盖、头部、forbidden/panel-toolbar 共用样式在 `AdminView.css`（`.admin-page :deep(...)`），面板 CSS 只写自身样式。
- 数据库管理 `DatabaseAdminView.vue/.ts/.css`：左侧 264px 表列表（名称＋注释、可关键词过滤、吸顶，≤900px 收为顶部区块）；右侧「数据浏览 / 表结构」两个标签页：数据浏览支持整行模糊搜索与分页（每页条数可选 10/20/50/100，默认 20，切换条数回到第一页；字根管理列表分页同款），NULL/布尔/JSON 单元格分别格式化；表结构含主键徽标、类型、可空、默认值、注释。接口 `ApiService.getAdminTables / getAdminTableColumns / getAdminTableRows`。
- 字根管理 `RootsAdminView.vue/.ts/.css`：`field_roots` 字段数据字典（字段名/基础短类型/长度/精度/小数位/注释/备注）。**列表实时展示现有表全部字段**（后端 LEFT JOIN field_roots 标注 `registered`/`id`，未入库行为 null），通用技术字段 id/created_at/updated_at 已排除；`row-key` 用 `field_name`（未入库行无 id）。状态列：未入库（灰）/未引用/一致/有差异；未入库行操作只有「入库」（表单以实时引用信息预填），入库后才有 同步/编辑/删除。列表与引用明细展示后端 `type_expr`；表单用 PostgreSQL 短名选择器（支持 [] 数组），varchar/bpchar 显示长度，numeric 显示总位数与小数位，时间类型显示秒精度，其余类型隐藏参数；完整类型实时预览，NULL 表示未指定，numeric(p) 小数位默认 0，切换类型清空参数。「扫描入库」批量注册未入库字段（排除通用技术字段与字典自身，优先取有注释的同名定义）；引用关系按字段名实时关联，「有差异」徽标悬浮即见各表具体差异（表名：长度/注释等 ≠ 值），展开与同步弹窗逐属性展示 `diffs`（引用列 ≠ 标准，含具体值）；展开明细标题注明比较基准（已入库=字根标准，未入库=首个引用表）。编辑只改字典记录，「同步」按 `type_synced/comment_synced` 统计需要修改的引用表，勾选注释或完整类型规格后手动执行并逐表显示结果，空类型禁用类型同步、无差异禁用开始；类型同步提示父表变更会递归影响继承/分区子表，执行前确认整棵继承树；删除不动引用表。接口 `ApiService.getAdminRoots / scanAdminRoots / createAdminRoot / updateAdminRoot / deleteAdminRoot / syncAdminRoot`。
- 服务管理 `ServicesAdminView.vue/.ts/.css`：Rust 卡片走 `ApiService.getAdminServices`（/admin/services，含 DB 连接与运行时长），Python 卡片走 `ApiService.checkPythonHealth()`（直连 pythonApi /health，不经降级），两张卡片展示状态徽标/版本/地址/上报时间。
- admin 接口仅 Rust 实现、Python 无兜底；401 触发登录跳转，403 时面板内显示「权限不足」。
- 风格与财务/设置页一致：暖色 `--workspace-*` token、42×46 倾斜图标＋22px 标题、13px 圆角工作区、局部 ConfigProvider 跟随全局主题（在外壳 AdminView）。
- `localStorage` 键为 `zytool-little-ledger-v1`，同一对象保存账单、各月份分类预算和演示标记；金额以整数分存储，汇总、SVG 环形图与日趋势、预算进度由当月账单的 computed 统一派生。
- 支持概览、全部账单、预算管理；月份选择、账单增改删、类型/分类/备注关键词筛选和分页。预算按月份独立设置，0 表示未设置；未设预算分类支出仍计入总预算剩余。
- 首次打开生成本月和上月演示数据；重置会覆盖所有月份账单与预算，须二次确认。单笔删除亦需确认；金额、日期、分类、备注在提交与存储读取边界校验。
- 读取失败不覆盖原存储，写入失败不提交本次修改。账本不会跨设备或跨浏览器同步，清除浏览器数据会丢失账本。
- 局部 Ant Design ConfigProvider 注入全局 `theme`，使用 computed 在 defaultAlgorithm/darkAlgorithm 之间切换，表单、下拉日历、分页及删除/重置弹窗与顶栏、设置页主题同步；无需另存财务主题，主题切换不更改账单和预算。
- 基础配色复用 `--workspace-*`，账本专有预算、危险状态及图表颜色由局部 `--ledger-*` 深浅变量控制；SVG 环形底轨、趋势网格/折线/填充/节点使用 CSS 变量，分类色仍与账单图例一致。ConfigProvider 色值与工作区 token 对应，配色调整须同步；手机端保留底部固定「记一笔」入口。
- 顶部与工具/AI 页保持一致的紧凑分割：页面顶部留白 20px，42×46 浅橙底倾斜图标，标题 22px、说明 12px，标题区底部 12px 留白加 1px 分隔线（原 toolbar 顶部线移除，避免双线）；手机端图标 36×40、标题 19px。
