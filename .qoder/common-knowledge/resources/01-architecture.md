# 01 架构

## 总体架构

前后端分离 + 双后端兜底：

- **后端（主要）**：Rust / axum，`Backup/tools_project_rust/`，默认 `http://localhost:8000`
- **后端（兜底）**：Python 3.9+ / FastAPI，`Backup/tools_project_py/`，默认 `http://localhost:8001`
- **前端**：Vue 3 + TypeScript + Vite + Ant Design Vue 4.x，`ZYTool/`

## 双后端机制（前端实现）

`ZYTool/src/services/api.ts` 创建两个 Axios 实例（地址配置见 `src/config/appConfig.ts`）：

- `rustApi`：`VITE_RUST_API_BASE_URL` + `/api/v1`，默认实例
- `pythonApi`：`VITE_PYTHON_API_BASE_URL` + `/api/v1`，兜底实例

- 普通请求统一走 `requestWithFallback`：优先 Rust；网络错误/超时/5xx 自动降级 Python；4xx 不降级
- 流式聊天 `ApiService.chatStream`：优先 Rust `POST /agents/chat`，失败降级 Python，支持 `AbortSignal` 中止
- 流式实现：Axios `adapter: 'fetch'` + `responseType: 'stream'` 获取 `ReadableStream`，按 SSE（`data: ...`）解析，遇 `[DONE]` 结束

## 统一响应结构

后端 handler 返回 `Json<BaseResponse<T>>`：

- `success: bool`、`message: Option<String>`、`base: Option<BaseInfo>`、`data: T`
- `BaseInfo` 由前端请求头 `X-Base-Info` 传入（computer_name/ip/username/screen_name），中间件解析后经 Extension 透传
- 前端 `unwrapResponse` 自动解包 `data`

## 认证

- JWT Bearer Token，前端存 localStorage（`ZYTool/src/utils/auth.ts`）
- Rust 侧 `core/auth.rs` 提供 `CurrentUser` 提取器
- 401 响应时前端自动 `clearAuth()`

## 数据库

- PostgreSQL（可选），sqlx 连接池（`core/db.rs`）
- 未连接时后端打印警告并以无 DB 模式启动（`main.rs` 中 `init_pool`）
- AI 会话/消息持久化表：`ai_sessions`、`ai_messages`（软删除 status=3）

## API 入口

`/health`（根健康检查）+ `/api/v1` 下按模块 nest：`agents` / `auth` / `health` / `protected` / `tools`。
