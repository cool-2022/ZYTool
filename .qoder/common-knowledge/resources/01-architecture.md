# 01 架构

## 总体架构

前后端分离 + 双后端部分兜底：

- **后端（主要）**：Rust / axum，`Backup/tools_project_rust/`，默认 `http://localhost:8000`
- **后端（兜底）**：Python 3.9+ / FastAPI，`Backup/tools_project_py/`，部署预期 `http://localhost:8001`；源码默认端口仍为 8000，并行启动需显式 `PORT=8001`
- **前端**：Vue 3 + TypeScript + Vite + Ant Design Vue 4.x，`ZYTool/`

## 双后端机制（前端实现）

`ZYTool/src/services/api.ts` 创建两个 Axios 实例（地址配置见 `src/config/appConfig.ts`）：

- `rustApi`：`VITE_RUST_API_BASE_URL` + `/api/v1`，默认实例
- `pythonApi`：`VITE_PYTHON_API_BASE_URL` + `/api/v1`，兜底实例

- 普通请求统一走 `requestWithFallback`：优先 Rust；网络错误/超时/5xx 自动降级 Python；4xx 不降级
- 流式聊天 `ApiService.chatStream`：优先 Rust `POST /agents/chat`，失败降级 Python，支持 `AbortSignal` 中止
- 流式实现：Axios `adapter: 'fetch'` + `responseType: 'stream'` 获取 `ReadableStream`，按 SSE（`data: ...`）解析，遇 `[DONE]` 结束
- **Python 只覆盖部分接口且有响应结构差异**，Agents 会话/模型、账号绑定/QQ 登录、DB 健康检查不能依赖降级；详见 `../../backend-knowledge/common/03-route-coverage.md`

## 统一响应结构

目标契约与当前差异（`BaseResponse<T>`、SSE、CORS、认证）见 `../../backend-knowledge/common/01-api-contract.md`；前端 `unwrapResponse` 只在存在 `success` 和 `data` 时解包，否则返回原始结构。

## 认证

- JWT Bearer Token，前端存 localStorage（`ZYTool/src/utils/auth.ts`）
- Rust 侧 `core/auth.rs` 提供 `CurrentUser` 提取器，用户使用 PostgreSQL；Python 当前使用独立的内存测试用户
- 401 响应时前端自动 `clearAuth()` 并跳转 `/?redirect=...`

## 数据库

- 仅 Rust 接入 PostgreSQL（可选），表结构与初始化前置条件见 `../../backend-knowledge/common/02-database-schema.md`；Python 当前无 DB 接入
- Rust 连接池只在启动时初始化一次，失败后打印警告并以无 DB 模式运行；DB 恢复后通常仍需重启进程

## API 入口

`/health`（根健康检查）+ `/api/v1` 下按模块 nest：`agents` / `auth` / `health` / `protected` / `tools`。完整方法、认证与双端覆盖见 `../../backend-knowledge/common/03-route-coverage.md`。
