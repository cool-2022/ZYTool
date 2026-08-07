# Rust 后端知识库（rust-backend-knowledge）

位置：`Backup/tools_project_rust/`（axum，端口 8000）

## 目录地图

```
src/
├── main.rs            # 入口：初始化配置/DB/中间件，挂载路由
├── core/              # config.rs（Settings/envy）、auth.rs（JWT/CurrentUser）、
│                      # db.rs（sqlx 连接池）、error.rs（AppError）、middleware.rs
├── models/            # 请求/响应模型（serde），按域分文件（agents/auth/text/...）
├── routes/            # 路由层
│   ├── agents/        #   AI 助手：mod.rs（挂载）、sessions.rs（会话 CRUD）、chat.rs（流式聊天）
│   ├── tools/         #   工具路由（按工具分文件，mod.rs 挂载）
│   └── auth.rs / health.rs / protected.rs
└── services/          # 业务逻辑
    ├── agents/        #   AI 助手：store.rs（DB 持久化）、llm.rs（Kimi 客户端）
    └── tools/         #   工具服务（map/password/regex/text/timestamp）
```

## 关键约定

- handler 返回 `Json<BaseResponse<T>>`；错误用 `core::error` 的 `bad_request/unauthorized/not_found/internal_error`
- 配置只走 `core::config::SETTINGS`；新增配置 = `Settings` 加字段 + `.env` 加变量
- AI 聊天：Kimi 未配置 `KIMI_API_KEY` 时回退占位回复；上下文取最近 20 条历史消息
- 注意：部分模型（如 k3-256k）不允许自定义 temperature，请求体不要携带该参数
- 模型延迟参考（2026-08 实测，TTFB）：k3-256k 6~12s（深度推理）、k3 ~3s、kimi-for-coding-highspeed ~0.8s；日常对话建议用 highspeed 模型
- 会话/消息表：`ai_sessions`、`ai_messages`，软删除（status=3），按 `session_uuid`/`message_uuid` 对外暴露

## 扩展入口

- 新工具：见公共库 `common-knowledge/resources/03-dev-conventions.md`
- 新 LLM 提供方：`services/agents/` 下新建客户端，参照 `llm.rs` 的 `is_configured/chat_stream/chat_complete` 模式

## Resources

- `resources/01-database-schema.md`：PostgreSQL 全部表结构（认证域 + AI 域，DDL 源文件 `ToolsAI/zytool.sql`）
