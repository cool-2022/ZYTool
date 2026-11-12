# 01 架构

## 总体架构

前后端分离 + 双后端部分兜底：

- **后端（主要）**：Rust / axum，`Backup/tools_project_rust/`，当前 `http://localhost:10003`（.env `PORT=10003`，源码默认 8000）
- **后端（兜底）**：Python 3.9+ / FastAPI，`Backup/tools_project_py/`，当前 `http://localhost:10004`（.env `PORT=10004`，源码默认 8000）
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

## Redis

- 2026-11-10 起项目新增 Redis 服务（连接配置登记在 `Config/database.env`，无密码，详见 `04-env-vars.md`）
- 双端客户端已接入（2026-11-10）：Rust `core/redis.rs` 启动时初始化多路复用连接（`redis` crate，`get_connection()` 取用，另有 `cache_get/cache_set/cache_del/incr_with_ttl/rate_limit_hit` 通用助手）；Python `app/core/redis_client.py` 懒加载（`redis.asyncio`，`get_redis()` 取用）
- 降级约定与 DB 一致：两端 `REDIS_HOST` 留空或连接失败均以无 Redis 模式继续运行；所有消费点静默降级（缓存未命中回源、限流放行、黑名单不校验）
- Rust 已消费场景（2026-11-10，Python 兜底端不校验/不共享）：
  - 登录防爆破：`login:fail:{用户名}` 计数，默认 5 次失败锁定 600s（`LOGIN_MAX_FAILURES`/`LOGIN_LOCKOUT_SECONDS`），锁定期间 401
  - JWT 注销黑名单：`auth:blacklist:{sha256(token)}`，TTL=token 剩余有效期，`CurrentUser` 提取器命中即 401
  - 工具分类缓存：`tools:categories`，TTL 300s，只缓存读库结果
  - 聊天限流：`rl:chat:{user_id}:{分钟桶}`，默认 20 次/分钟（`CHAT_RATE_LIMIT_PER_MINUTE`），超限 429
  - 公开工具限流：`rl:tools:{IP}:{分钟桶}`，默认 60 次/分钟（`TOOLS_RATE_LIMIT_PER_MINUTE`），中间件挂在 `/tools` 路由，超限 429
  - Agents 会话历史缓存：`agents:history:{session_id}` Redis List，RPUSH 增量追加 + LTRIM 保留 50 条，TTL 1800s，未命中回源 DB 并回填
- Python 已消费场景：天气工具缓存（同步客户端，`weather:now:{城市}` 600s、`weather:city:{城市}` 86400s；与 Rust Function Call 天气工具共用 key，双端互享）

## API 入口

`/health`（根健康检查）+ `/api/v1` 下按模块 nest：`agents` / `auth` / `health` / `protected` / `tools`。完整方法、认证与双端覆盖见 `../../backend-knowledge/common/03-route-coverage.md`。
