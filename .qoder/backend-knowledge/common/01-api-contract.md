# 01 后端接口契约（双端公共）

Rust（8000，主）与 Python（8001，兜底）两端**必须共同遵守**的契约。改动任一端接口时，需同步检查另一端。

## 接口一致性

- 统一前缀 `/api/v1`，两端接口路径、请求/响应结构保持一致
- 前端降级规则：普通请求走 `requestWithFallback`，网络错误/超时/5xx 自动降级 Python；**4xx 不降级**
- 新增/修改后端接口时原则上两端同步实现，否则降级会失败

## 统一响应结构

- handler 返回 `BaseResponse<T>`：`success: bool`、`message: Option<String>`、`base: Option<BaseInfo>`、`data: T`
- `BaseInfo` 由前端请求头 `X-Base-Info` 传入（computer_name/ip/username/screen_name），中间件解析后透传
- 前端 `unwrapResponse` 自动解包 `data`

## SSE 流式规范

- 流式接口（`/agents/chat`）两端均输出 SSE：每条消息 `data: ...`，以 `data: [DONE]` 结束
- 前端用 Axios `adapter: 'fetch'` + `responseType: 'stream'` 获取 `ReadableStream` 手动解析，支持 `AbortSignal` 中止

## CORS

- 放行 localhost/127.*/192.168.*/10.*/172.16-31.* 及 `localhost:5000/5050/5173`，两端行为保持一致

## 认证

- JWT Bearer Token；401 时前端自动 `clearAuth()` 跳转登录

## 数据库

- 两端共用同一个 PostgreSQL（`zytool` 库），表结构见 `02-database-schema.md`
- DB 未连接时后端以无 DB 模式启动，涉及 DB 的功能需降级处理
