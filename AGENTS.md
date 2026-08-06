# AGENTS.md

## 项目概述

ZYTool 是一个全栈在线工具集合，采用前后端分离 + 双后端兜底架构：

- **后端（主要）**：Rust / axum，位于 `Backup/tools_project_rust/`，默认 `http://localhost:8000`
- **后端（兜底）**：Python 3.9+ / FastAPI，位于 `Backup/tools_project_py/`，默认 `http://localhost:8001`，Rust 后端不可用时自动降级
- **前端**：Vue 3 + TypeScript + Vite + Ant Design Vue 4.x，位于 `ZYTool/`

核心功能：AI Agent 系统（DeepSeek + Function Call + SSE 流式对话）、在线工具集（JSON 格式化、Base64、正则测试、密码生成等）、MES 系统、MCP 协议支持。

### 双后端机制（前端）

前端 `src/services/api.ts` 中创建两个 Axios 实例（配置见 `src/config/appConfig.ts`）：

- `rustApi`：`VITE_RUST_API_BASE_URL` + `/api/v1`，默认实例（`export const api = rustApi`）
- `pythonApi`：`VITE_PYTHON_API_BASE_URL` + `/api/v1`，兜底实例

普通请求默认走 `rustApi`；流式聊天 `AgentService.chatStream` 优先请求 Rust 的 `POST /agents/chat`，失败时降级到 Python 后端。流式请求使用 Axios `adapter: 'fetch'` + `responseType: 'stream'` 获取 `ReadableStream`，按 SSE（`data: ...`）格式解析，遇 `[DONE]` 结束。

## 目录结构

```
100_Code/
├── Backup/                        # 后端项目
│   ├── tools_project_rust/        # Rust 后端（主要，axum）
│   │   ├── src/
│   │   │   ├── main.rs            # 入口：初始化配置/DB/中间件，挂载路由
│   │   │   ├── core/              # 核心：config.rs（配置）、auth.rs（JWT）、
│   │   │   │                      #       db.rs（sqlx 连接池）、error.rs、middleware.rs
│   │   │   ├── models/            # 请求/响应数据模型（serde）
│   │   │   ├── routes/            # 路由层：agents / auth / health / protected / tools/
│   │   │   └── services/          # 业务逻辑：agents、tools/
│   │   ├── Cargo.toml
│   │   └── start.md
│   └── tools_project_py/          # Python FastAPI 兜底后端（默认端口 8001）
└── ZYTool/                        # 前端项目
    └── src/
        ├── views/           # 页面组件（Agents/ ViewFront/ ViewBack/ MES/）
        ├── services/        # API 服务（api.ts：rustApi / pythonApi 双 Axios 实例）
        ├── config/          # 应用配置（appConfig.ts：双后端地址）
        ├── router/          # 路由配置
        └── utils/           # 工具函数
```

## 常用命令

### 后端 Rust（Backup/tools_project_rust/）

```bash
cargo run            # 启动服务，默认监听 http://0.0.0.0:8000
cargo build          # 编译（类型检查）
cargo build --release  # 生产构建（opt-level 3 + LTO）
cargo check          # 快速类型检查
```

### 前端（ZYTool/）

```bash
npm run dev       # 启动开发服务器（端口 5050，支持热重载）
npm run build     # vue-tsc 类型检查 + 生产构建（提交前必跑，作为 typecheck）
npm run preview   # 预览生产构建
```

API 入口：`/health`（根健康检查）+ `/api/v1` 下按模块 nest：`agents` / `auth` / `health` / `protected` / `tools`，认证方式为 JWT Bearer Token。

## 开发约定

### 添加新工具

- **Rust 后端工具**：
  1. 在 `src/services/tools/` 创建服务文件实现业务逻辑，并在 `src/services/tools/mod.rs` 导出
  2. 在 `src/models/` 创建对应的请求/响应模型（serde `Deserialize`/`Serialize`）
  3. 在 `src/routes/tools/` 创建路由文件并在 `src/routes/tools/mod.rs` 中挂载
- **前端工具**：在 `src/views/ViewFront/`（浏览器端）或 `src/views/ViewBack/`（服务器端）创建 Vue 组件 → 在 `src/router/index.ts` 添加路由 → 在 `src/services/api.ts` 添加 API 调用（普通请求用默认 `api` 即 `rustApi`；需要兜底时参照 `chatStream` 的 try/catch 降级模式）
- **Agent 工具**：Rust 侧在 `src/services/agents.rs` 与 `src/routes/agents.rs` 中扩展

### 代码风格

- Rust：axum 路由 + handler 返回 `Json<BaseResponse<T>>`（统一响应结构，含 `BaseInfo` 扩展）；配置统一走 `core::config::SETTINGS`（`once_cell::Lazy` + `envy` 从环境变量读取）；异步基于 tokio；数据库使用 sqlx（PostgreSQL，可选，未连接时服务仍可启动）
- 前端：Vue 3 Composition API + TypeScript，UI 使用 Ant Design Vue 组件，HTTP 请求统一走 Axios（`src/services/api.ts`）；流式响应用 `adapter: 'fetch'` + `responseType: 'stream'`，手动解析 SSE 并按 `data: ` 行 yield
- 遵循现有文件的代码风格与目录组织方式，不要随意引入新依赖

### 环境变量

后端通过 `.env`（`dotenvy` 加载，不要提交到仓库）配置，对应 `core/config.rs` 中的 `Settings` 字段：

```env
SECRET_KEY=your-secret-key-here
QWEATHER_KEY=your-qweather-key
DATABASE_URL=postgres://user:pass@localhost:5432/zytool
QQ_APP_ID=...
QQ_APP_KEY=...
```

常用配置默认值：`host=0.0.0.0`、`port=8000`、CORS 允许 `localhost:5000/5050/5173`。

前端通过 Vite 环境变量配置双后端地址（见 `src/config/appConfig.ts`）：

```env
VITE_RUST_API_BASE_URL=http://localhost:8000
VITE_PYTHON_API_BASE_URL=http://localhost:8001
```

## 注意事项

- 提交前端代码前务必运行 `npm run build` 确认 vue-tsc 类型检查通过；提交 Rust 代码前运行 `cargo check`（或 `cargo build`）确认编译通过
- 数据库未连接时后端会打印警告并继续以无 DB 模式启动（`main.rs` 中 `init_pool`）
- 项目目前缺少单元测试与完整用户系统，改动时注意兼容性
- 不要提交任何密钥、API Key 或 `.env` 文件
- 数据库（可选）：PostgreSQL（sqlx）
