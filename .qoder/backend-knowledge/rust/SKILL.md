# Rust 后端知识库（backend-knowledge/rust）

位置：`Backup/tools_project_rust/`（axum，默认端口 8000）

双端目标契约与当前差异见 `../common/01-api-contract.md`；完整路由、认证要求和 Python 覆盖情况见 `../common/03-route-coverage.md`。

## 目录地图

```
src/
├── main.rs            # 入口：初始化配置/DB/用户缓存/中间件，挂载路由
├── core/              # config.rs（Settings/envy）、auth.rs（JWT/CurrentUser）、
│                      # db.rs（sqlx 连接池）、crypto.rs（模型密钥加解密）、
│                      # error.rs（AppError）、middleware.rs
├── models/            # 请求/响应模型（serde），按域分文件（agents/auth/text/...）
├── routes/            # 路由层
│   ├── agents/        #   AI 助手：mod.rs（挂载）、sessions.rs（会话/模型）、chat.rs（流式/同步）
│   ├── tools/         #   工具路由（按工具分文件，mod.rs 挂载）
│   └── auth.rs / health.rs / protected.rs
└── services/          # 业务逻辑
    ├── agents/        #   AI 助手：store.rs（DB 持久化）、llm.rs（模型客户端）
    └── tools/         #   工具服务（map/password/regex/text/timestamp）
```

## 关键约定

- 普通 JSON handler 返回 `Json<BaseResponse<T>>`；SSE 聊天除外。错误用 `core::error` 的 `bad_request/unauthorized/not_found/internal_error`，错误响应不透传 `BaseInfo`
- 配置只走 `core::config::SETTINGS`；新增配置 = `Settings` 加字段 + `.env` 加变量。字段存在不代表功能已接线，限流、请求体限制、文件日志、天气和 JWT algorithm 当前均为预留配置
- 对外模型列表来自 `GET /agents/models`；首页等静态文案不能作为实际模型提供方依据

## 认证与账号绑定现状

- 密码登录只按 `users.nickname` 查询，请求字段虽叫 `username`，不支持邮箱或手机号登录；注册密码最少 6 位，成功直接返回 token，注册模型中的 `email` 当前未保存
- 启动时加载用户到内存缓存，缓存未命中才查 DB；注册和 QQ 登录会更新缓存。直接修改 DB 后，缓存及既有 token 不一定立即反映新密码、角色或状态
- JWT 含 `sub/user_id/roles/exp`，算法固定 HS256；`CurrentUser` 只验证 token，不在每次请求时刷新用户状态。`logout` 无黑名单或服务端撤销
- 手机绑定只校验格式并直接标记已验证，没有验证码流程；`bind/qq`、`bind/wechat` 直接接受 `open_id`，不做服务端 OAuth 校验
- 只有 QQ 实现 OAuth 登录；`state` 生成后未保存和校验。自动建用户与写绑定不是显式事务，昵称获取失败不阻塞登录

## Agents 行为

- 模型选择：显式 `model_id` 优先（按 id 加载并解密，**允许选择停用模型**）；否则取 `ai_models` 中 `is_enabled=true` 且 priority 最高项（60s 缓存）；最后回退 `.env` 的 `KIMI_*`
- `model_id` 会写入会话，但后续请求未再次携带时**不会读取会话的 `model_id`**，而会重新选择全局默认模型
- 上下文取最近 20 条历史；系统提示词是 `llm.rs` 常量，未使用 `ai_sessions.system_prompt` 或 `ai_prompt_templates`
- 上游请求体只有 `model/messages/stream`；模型 `max_tokens_default/config` 与会话 temperature/top_p/max_tokens 均未接入。部分模型（如 k3-256k）不允许自定义 temperature
- 两种聊天在未传 `session_id` 时都会自动创建会话，标题取首条消息前 20 个字符并追加 `...`，但响应/SSE 不回传新会话 UUID；同步接口成功返回 `{reply}`
- SSE 错误、中断持久化及 token 统计见 `../common/01-api-contract.md`
- 会话/消息使用 `ai_sessions`、`ai_messages`；删除为单个数据修改 CTE：先保存活跃消息的精简快照到 `ai_session_archives`，再物理删除会话，关联数据随 CASCADE 清除
- 模型延迟参考（2026-08 特定环境实测，非稳定保证）：k3-256k TTFB 6~12s、k3 ~3s、kimi-for-coding-highspeed ~0.8s

## 工具与运行边界

- 工具目录是硬编码展示数据，不等于后端能力清单；地图路线为 mock 直线插值，固定 30 km/h 估算，`waypoints` 未使用；天气配置没有业务调用
- DB 只在启动时初始化一次，失败后可运行 DB 无关工具，但不会因 DB 稍后恢复而自动重连初始化
- `start.sh` 使用 debug 二进制看门狗，只有二进制不存在才自动 `cargo build`；修改源码后需显式重新 build，不能靠看门狗重启加载新代码

## 扩展入口

- 新工具：见公共库 `../../common-knowledge/resources/03-dev-conventions.md`
- 新 LLM 提供方：`services/agents/` 下新建客户端，参照 `llm.rs` 的 `is_configured/chat_stream/chat_complete` 模式

## Resources

- `../common/02-database-schema.md`：PostgreSQL 全部表结构及初始化前置条件
- `../common/03-route-coverage.md`：完整路由矩阵与 Python 降级覆盖
