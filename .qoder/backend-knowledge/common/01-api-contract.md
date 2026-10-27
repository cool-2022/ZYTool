# 01 后端接口契约（双端公共）

本文记录双端的**目标契约**与当前差异。Rust 是主后端；Python 目前只实现部分兼容接口，不能把“存在降级逻辑”理解为所有功能都有完整兜底。改动接口时先查 `03-route-coverage.md`，再检查另一端实现。

## 接口一致性与降级

- 目标：统一前缀 `/api/v1`，两端接口路径、请求/响应结构保持一致
- 前端普通请求走 `requestWithFallback`：网络错误/超时/5xx 自动降级 Python；**4xx 不降级**
- Python 缺失的接口在降级后返回 404；Agents 会话/模型、账号绑定/QQ 登录、DB 健康检查目前没有 Python 兜底
- 新增/修改后端接口时原则上两端同步实现；若暂不同步，必须在各自知识库标注能力差异

## 统一响应结构

- 目标：`BaseResponse<T>` 包含 `success`、`message`、`base`、`data`
- `BaseInfo` 由前端请求头 `X-Base-Info` 传入（computer_name/ip/username/screen_name）；前端 `unwrapResponse` 在存在 `success` 和 `data` 时解包，否则返回原始结构
- Rust 普通 JSON handler 按上述结构返回；`/agents/chat` 是 SSE，不属于 JSON 响应。`AppError` 的错误响应不透传 `BaseInfo`
- **Python 当前偏差**：`BaseResponse` 只有 `success/message`，多数工具及认证、健康、同步聊天接口直接返回业务字段；文本对比连 `success` 也不包含，中间件不解析 `X-Base-Info`

## SSE 流式规范

- `/agents/chat` 两端均输出 `data: ...`，正常结束时发送 `data: [DONE]`
- 前端用 Axios `adapter: 'fetch'` + `responseType: 'stream'` 获取 `ReadableStream` 分行解析，支持 `AbortSignal` 中止
- Rust 在流建立前失败可返回普通 HTTP 500；流建立后的上游错误会转成普通 SSE 文本帧并仍以 `[DONE]` 结束，错误文本可能随助手消息持久化
- Rust 用户消息在调用模型前保存；客户端提前断开时，助手消息可能未保存。流式 token 统计当前固定为 0
- Python 流式接口是无状态单轮实现，不使用 Function Call；同步 `/agents/chat/sync` 才使用工具

## CORS

- 目标：精确来源清单 + 私网/本机正则放行；覆盖 localhost、127.*、192.168.*、10.*、172.16-31.*、`.local` 主机，端口不限
- **当前差异**：Python 只放行 `127.0.0.1` 而非整个 127.*；Python 开启 credentials，Rust 未开启

## 认证

- 使用 JWT Bearer Token；前端收到 401 后清除本地认证并跳转 `/?redirect=...`
- Rust 使用 PostgreSQL 用户数据；Python 当前使用进程内测试用户，重启丢失注册数据，也不与 Rust 用户库同步

## 数据库

- 仅 Rust 接入 PostgreSQL（`zytool` 库），表结构见 `02-database-schema.md`；Python 当前无 DB 接入
- Rust 启动时只初始化一次连接池，失败后以无 DB 模式运行；DB 恢复后通常仍需重启进程。Agent 和多数认证接口此时返回 500，DB 无关工具仍可使用
- Rust 启动不自动建表或执行 migration，初始化前置条件见 `02-database-schema.md`
