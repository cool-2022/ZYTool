# 02 数据库表结构

PostgreSQL（`zytool` 库），DDL 源文件：`ToolsAI/zytool.sql`，当前仅 Rust 后端接入。各表按用途记录时间戳；仅 `users`、`user_auths`、`ai_models`、`ai_prompt_templates`、`ai_sessions`、`ai_session_settings` 带 `updated_at` 触发器。

## 表总览

| 表 | 用途 | 域 |
|---|---|---|
| `users` | 用户主表 | 认证 |
| `user_auths` | 第三方登录绑定（QQ/微信） | 认证 |
| `user_verify_codes` | 验证码（手机/邮箱） | 认证 |
| `user_login_logs` | 登录日志 | 认证 |
| `ai_models` | LLM 模型配置 | AI |
| `ai_prompt_templates` | 提示词模板 | AI |
| `ai_sessions` | AI 会话 | AI |
| `ai_messages` | AI 消息 | AI |
| `ai_session_settings` | 会话级推理参数 | AI |
| `ai_message_feedbacks` | 消息反馈（点赞/点踩） | AI |
| `ai_session_archives` | 会话物理删除时的精简快照 | AI |

## 当前数据库核验

2026-10-27 使用 DBeaver 的 PostgreSQL JDBC 42.7.13 驱动，通过 Java 对 `zytool` 数据库执行只读查询：服务端为 PostgreSQL 18.4（Windows x86_64），`public` 模式共有 11 张业务表，与上方表总览一致。

数据库对象中保存的表注释如下（不是根据代码推测的说明）：

| 表 | 数据库表注释 |
|---|---|
| `users` | 用户主表 |
| `user_auths` | 第三方登录授权表（QQ/微信） |
| `user_verify_codes` | 手机/邮箱验证码表 |
| `user_login_logs` | 登录日志表 |
| `ai_models` | AI 模型配置表 |
| `ai_prompt_templates` | 系统提示词模板表 |
| `ai_sessions` | AI 助手会话表 |
| `ai_messages` | AI 助手消息表 |
| `ai_session_settings` | 会话级参数覆盖表 |
| `ai_message_feedbacks` | AI 消息反馈表 |
| `ai_session_archives` | 未设置表注释；用途见下方“删除备份履历” |

## 认证域

### users（用户主表）
- `id bigserial PK`、`nickname`、`avatar_url`、`gender int2`、`phone`（部分唯一）、`phone_verified`、`email`（部分唯一）、`email_verified`、`password_hash`
- `status int2 DEFAULT 1`、`last_login_at`、`last_login_ip inet`、`roles text[] DEFAULT '{user}'`
- 唯一索引：`uk_users_email` / `uk_users_phone`（IS NOT NULL 时）、`uk_users_nickname`（非空时）

### user_auths（第三方绑定）
- `user_id FK→users CASCADE`、`provider varchar(20)`、`open_id`、`union_id`、`access_token`、`refresh_token`、`expires_at`、`nickname`、`avatar_url`、`raw_profile jsonb`
- 唯一约束：`uk_user_auths_provider_openid (provider, open_id)`

### user_verify_codes（验证码）
- `target`（手机号/邮箱）、`channel varchar(10)`、`scene varchar(20)`、`code_hash`、`expires_at`、`used_at`、`attempts`、`ip inet`

### user_login_logs
- `user_id FK→users CASCADE`、`provider`、`ip inet`、`user_agent`、`device`、`success`、`fail_reason`

## AI 域

### ai_models（模型配置）
- `provider varchar(32)`、`model_name varchar(64)`、`display_name`、`api_base_url`、`api_key_encrypted`、`is_enabled`、`priority int2`、`max_tokens_default`、`config jsonb`
- 唯一约束：`uk_ai_models_provider_model (provider, model_name)`
- 索引：`idx_ai_models_enabled_priority (is_enabled, priority DESC)`
- **已接入**：`services/agents/llm.rs::resolve_model` 取 `is_enabled=true` 且 priority 最高的模型（60s 内存缓存），无启用模型时回退 `.env` 的 `KIMI_*`
- 当前数据快照（2026-08 开发环境，非 DDL 保证）：kimi 4 个模型（kimi-for-coding-highspeed 启用 priority=100，其余停用备用）；切换模型需保证只有目标默认项启用，60s 缓存内生效
- `api_key_encrypted` 加密存储（2026-08-07 起）：AES-256-GCM，密钥 = SHA-256(SECRET_KEY)，格式 `v1:base64(nonce+密文)`；加解密见 `core/crypto.rs`；加密工具 `cargo run -- encrypt-key <明文>`；历史明文数据兼容读取，启动时检测到明文会告警

### ai_prompt_templates
- `name`、`content text`、`description`、`is_system bool`、`sort_order int2`

### ai_sessions（会话）
- `id bigserial PK`、`user_id FK→users CASCADE`、`session_uuid uuid UNIQUE`（对外暴露 ID）
- `title DEFAULT '新对话'`、`model_id FK→ai_models SET NULL`、`system_prompt`
- `status int2 DEFAULT 1`（3 是历史软删除预留值；当前 Rust 直接物理删除，不再写入 3）、`message_count`、`total_tokens`
- 索引：`idx_ai_sessions_user_updated (user_id, updated_at DESC)`、`idx_ai_sessions_uuid`

### ai_messages（消息）
- `id bigserial PK`、`session_id FK→ai_sessions CASCADE`、`message_uuid uuid UNIQUE`（对外暴露 ID）
- `role varchar(20)`（user/assistant/system）、`content text`、`content_type DEFAULT 'text'`、`tokens_used`
- `model_id FK→ai_models SET NULL`、`parent_message_id FK→ai_messages SET NULL`（支持消息树/重新生成）
- `status int2 DEFAULT 1`、`metadata jsonb`
- 索引：`idx_ai_messages_session_created (session_id, created_at)`

### ai_session_settings（会话推理参数）
- `session_id UNIQUE FK→ai_sessions CASCADE`、`temperature float4 DEFAULT 0.7`、`max_tokens DEFAULT 4096`、`top_p float4 DEFAULT 1.0`、`extra jsonb`

### ai_message_feedbacks
- `message_id FK→ai_messages CASCADE`、`user_id FK→users CASCADE`、`rating int2`、`comment`
- 唯一约束：`uk_ai_message_feedbacks_user_msg (message_id, user_id)`

### ai_session_archives（删除备份履历）
- 删除会话时由 `store.rs::delete_session` 在同一事务中写入，随后**物理删除** `ai_sessions` 行（消息/设置/反馈随外键 CASCADE 一并清除，数据仅以履历快照形式保留）
- `session_id`、`session_uuid`、`user_id`、`title`、`message_count`、`total_tokens`
- `messages jsonb`：删除时活跃消息（`status=1`）的精简快照（[{role, content, created_at}]），不是完整消息行备份
- `deleted_at timestamptz DEFAULT now()`；索引 `idx_ai_session_archives_user (user_id, deleted_at DESC)`

## 代码对应关系

- Rust 侧访问：`src/services/agents/store.rs`（ai_sessions/ai_messages）、`src/routes/auth.rs`（users/user_auths）
- 会话和消息的内部自增 ID 不对外，分别使用 `session_uuid`/`message_uuid`；模型选择则直接暴露并使用 `ai_models.id`
- 当前已用字段子集；`ai_prompt_templates`、`ai_session_settings`、`ai_message_feedbacks`、`parent_message_id` 等表/字段已建库但代码尚未接入，属预留扩展点

## 初始化前置条件

- Rust 启动不执行建表或 migration，需先准备数据库与表结构。
- `ToolsAI/zytool.sql` 不是独立的一键初始化脚本：使用了 `set_updated_at()` 但脚本未定义该函数；`gen_random_uuid()` 还依赖 PostgreSQL 版本或相应扩展环境。
