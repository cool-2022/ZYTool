# Debug Session: ai-chat-network-error

- **Status**: [OPEN]
- **Issue**: AI 助手聊天页面发送消息后显示 "错误: Network Error"，错误信息直接作为 AI 回复内容展示在聊天气泡中。
- **Debug Server**: http://127.0.0.1:7777/event
- **Log File**: .dbg/trae-debug-log-ai-chat-network-error.ndjson

## Reproduction Steps

1. 启动前端（localhost:5050）和 Rust 后端（localhost:8000）。
2. 进入 AI 助手页面。
3. 在输入框中输入任意内容并按 Enter 发送。
4. 观察到 AI 助手回复气泡中显示 "错误: Network Error"。

## Hypotheses & Verification

| ID  | Hypothesis                                                                  | Likelihood | Effort | Evidence |
| --- | --------------------------------------------------------------------------- | ---------- | ------ | -------- |
| A   | Rust 后端 `/agents/chat` 调用 Kimi API 失败/超时，导致 SSE 流异常中断       | High       | Medium | Pending  |
| B   | 前端 axios fetch adapter 与 SSE 流式响应不兼容，无法正确读取 ReadableStream | Medium     | Medium | Pending  |
| C   | CORS 配置导致浏览器拒绝读取 SSE 响应体                                      | Medium     | Low    | Pending  |
| D   | 后端数据库连接/会话保存失败，handler 提前返回错误                           | Low        | Low    | Pending  |
| E   | 前端错误处理直接把 `Network Error` 塞入 assistant 消息气泡，产品体验不佳    | High       | Low    | Pending  |

## Instrumentation Points

- `ZYTool/src/services/api.ts:chatStream-start` — 流式调用入口
- `ZYTool/src/services/api.ts:request-start` — axios 发起 SSE 请求
- `ZYTool/src/services/api.ts:request-success` — axios 拿到响应
- `ZYTool/src/services/api.ts:rust-failed` — Rust 端请求失败详情
- `ZYTool/src/services/api.ts:python-fallback-start` — 降级到 Python 后端
- `ZYTool/src/services/api.ts:sse-chunk` — 前 3 条 SSE 数据解析
- `ZYTool/src/services/api.ts:stream-done` — 流读取正常结束
- `ZYTool/src/services/api.ts:stream-read-error` — 流读取异常
- `ZYTool/src/views/Agents/ChatView.ts:sendMessage-error` — UI 层捕获到的错误
- `Backup/tools_project_rust/src/routes/agents/chat.rs:chat-handler-start` — 后端 handler 入口
- `Backup/tools_project_rust/src/routes/agents/chat.rs:user-message-saved` — 用户消息落库
- `Backup/tools_project_rust/src/routes/agents/chat.rs:llm-config-check` — Kimi 是否配置
- `Backup/tools_project_rust/src/services/agents/llm.rs:kimi-request-start` — Kimi 请求发起
- `Backup/tools_project_rust/src/services/agents/llm.rs:kimi-request-error` — Kimi 请求失败
- `Backup/tools_project_rust/src/services/agents/llm.rs:kimi-response-error` — Kimi 返回非 2xx
- `Backup/tools_project_rust/src/services/agents/llm.rs:kimi-response-success` — Kimi 返回成功

## Log Evidence

[Pending]

## Verification Conclusion

[Pending]
