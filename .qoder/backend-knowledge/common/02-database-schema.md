# 01 数据库表结构

PostgreSQL（`zytool` 库），DDL 源文件：`ToolsAI/zytool.sql`。所有表带 `created_at/updated_at`，多数表有 `trg_*_updated_at` 触发器（`set_updated_at()` 函数）自动维护 `updated_at`。

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
| `ai_session_archives` | 会话删除备份履历（软删除时写入快照） | AI |

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

### ai_prompt_templates
- `name`、`content text`、`description`、`is_system bool`、`sort_order int2`

### ai_sessions（会话）
- `id bigserial PK`、`user_id FK→users CASCADE`、`session_uuid uuid UNIQUE`（对外暴露 ID）
- `title DEFAULT '新对话'`、`model_id FK→ai_models SET NULL`、`system_prompt`
- `status int2 DEFAULT 1`（软删除 = 3）、`message_count`、`total_tokens`
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
- `messages jsonb`：删除时的消息快照（[{role, content, created_at}]）
- `deleted_at timestamptz DEFAULT now()`；索引 `idx_ai_session_archives_user (user_id, deleted_at DESC)`

## 代码对应关系

- Rust 侧访问：`src/services/agents/store.rs`（ai_sessions/ai_messages）、`src/routes/auth.rs`（users/user_auths）
- 对外 ID 一律用 `session_uuid`/`message_uuid`（字符串），内部自增 `id` 不出 API
- 当前已用字段子集；`ai_models`、`ai_prompt_templates`、`ai_session_settings`、`ai_message_feedbacks`、`parent_message_id` 等表/字段已建库但代码尚未接入，属预留扩展点
