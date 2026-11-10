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
| `field_roots` | 字段字根表（数据字典），按字段名自动关联各表字段 | 后台管理 |
| `tool_categories` | 工具分类（工具中心展示） | 工具 |
| `tools` | 工具（含 route 驱动前端跳转） | 工具 |

## 当前数据库核验

2026-10-27 使用 DBeaver 的 PostgreSQL JDBC 42.7.13 驱动，通过 Java 对 `zytool` 数据库执行只读查询：服务端为 PostgreSQL 18.4（Windows x86_64），`public` 模式当时共有 11 张业务表，对应上方认证/AI 域，尚未创建 field_roots。

2026-11-05 以同样方式（Java + DBeaver PostgreSQL JDBC 驱动，连接串取自 Rust `.env` 的 `DATABASE_URL`）执行 DDL：`DROP TABLE IF EXISTS input_roots` 并创建 `field_roots`，已核验表存在。

2026-11-06 用 Java + DBeaver JDBC 填充 `field_roots`：取 `public` 下除 `field_roots` 外 11 张业务表的全部字段，按字段名去重写入 69 条（`ON CONFLICT DO NOTHING`）。同名字段优先取有列注释的那一条，其次按表名排序；因此字典记录的是“代表定义”，并非每张引用表都一致：
- `provider`：字典取 `ai_models` 的 `varchar(32)`，`user_auths`/`user_login_logs` 为 `varchar(20)`
- `status`、`comment` 等通用字段在各表含义不同，字典注释只代表其中一张表
- 写入时 44 条注释为空（对应列本身未设置 `COMMENT`），需后续人工补充
- 对上述字段执行「同步」（`POST /admin/roots/{id}/sync`）会把字典的类型/注释覆盖到所有同名列，执行前须确认

2026-11-06 类型拆分后再次使用 Java + DBeaver JDBC 只读核验：field_roots 共 69 条，四个类型属性均匹配至少一个真实引用列；全部创建人为 admin、修改人 NULL。本次连接配置由 Java 直接读取 `Config/database.env`（仅记录位置与用途，不记录连接参数/凭据），Rust 的系统目录查询 SQL 已在数据库执行验证。同日又在单事务中按语义分组重建物理列顺序，重排前后 69 行内容摘要一致，id 序列、主键/唯一/CHECK 约束与表列注释均已核验。

注：`Config/database.env` 自 2026-11-10 起同时登记 Redis 连接配置（无密码）；同日双端已接入 Redis 客户端（Rust `core/redis.rs`、Python `app/core/redis_client.py`，读取各自 `.env`），但业务尚无消费，详见 `../../common-knowledge/resources/04-env-vars.md`。

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
| `field_roots` | 字段字根表（数据字典）：字段名与现有表字段按名自动关联，注释/类型可手动同步到引用表字段 |

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

## 后台管理域

### field_roots（字段字根表/数据字典，2026-11-05 新增；替代已删除的 input_roots 五笔字根表）
- 物理列顺序按语义分组（2026-11-06 重建核验）：`id bigserial PK`、`field_name varchar(63) UNIQUE`、`data_type`、`data_length`、`data_precision`、`data_scale`、`comment`、`remark`、`created_by`、`updated_by`、`created_at`、`updated_at`
- `field_name` 为字段名（即字根标识）；`comment text` 为标准注释；`remark text` 为备注，不同步
- 2026-11-06 类型结构拆为四列，存量 69 条已迁移，保留 id/注释/备注/创建修改信息；未新增可空或默认值属性：
  - `data_type varchar(32) NOT NULL DEFAULT ''`：PostgreSQL 基础短名，如 `varchar`、`numeric`、`int8`、`timestamptz`；数组加 `[]`，不含长度/精度
  - `data_length int4 NULL`：`varchar/bpchar` 字符长度，API 限制 1–10485760
  - `data_precision int4 NULL`：`numeric` 总位数（1–1000）；`time/timetz/timestamp/timestamptz/interval` 秒的小数位（0–6）
  - `data_scale int4 NULL`：`numeric` 小数位（0–precision），必须同时指定精度；`numeric(p)` 未填小数位时归一为 0
  - NULL 表示未显式指定参数，非 0；DB CHECK 保证长度为正、精度 0–1000、小数位 0–precision，按类型的更严格限制由 API 校验
- `created_by varchar(64) NOT NULL DEFAULT 'admin'`（创建人）、`updated_by varchar(64) NULL`（修改人，未修改时为空），2026-11-06 新增并已在库中执行
- 后端代码暂未读写这两个字段：`FIELD_ROOT_COLUMNS` 未包含，新建依赖默认值 `admin`，`update_root` 不会写入 `updated_by`；接入登录用户后需同步改 `FieldRootRow`、SQL 与前端
- **列表语义（2026-11-06）**：`GET /admin/roots` 的数据源是现有表全部字段的实时清单（`SELECT DISTINCT field_name FROM 实时列元数据` LEFT JOIN `field_roots`），不再只显示已入库记录；每条带 `registered`（是否入库）与 `id`（未入库为 null）。通用技术字段 `id/created_at/updated_at` 在列表与扫描中均排除（`live_columns_sql` 包装 `COLUMN_META_SQL`），但同步仍按引用实况处理。未入库字段的类型/注释取首个引用列作为展示准绳，`refs.diffs` 反映各表之间的分歧
- 无 `updated_at` 触发器，更新由 `services/admin.rs::update_root` 在 SQL 中置 `now()`
- **引用关系不落表**：按 `field_name` 从 public 下普通表/分区表实时关联（排除 field_roots），pg_attribute/pg_type 解码 atttypmod 得短名、长度、精度、小数位；数组沿用元素类型规格
- **API**：字根和引用列返回四个类型属性及组合展示 `type_expr`（如 varchar(64)、numeric(12,2)、timestamptz(3)、varchar(20)[]），引用列另带 `type_synced/comment_synced/synced/diffs`；按类型、长度、精度、小数位、注释分别比较，差异方向为「引用列 ≠ 字根标准」（未入库字段以首个引用列为基准）；**diffs 含具体值**（如 `长度 20 ≠ 32`、`注释 （空） ≠ 「模型提供商标识」`，注释超 20 字截断）
- **入库**：`scan_roots` 用 `INSERT ... SELECT DISTINCT ON (field_name) ... ON CONFLICT DO NOTHING` 扫描，排除字典自身；先过滤不满足字典 CHECK 的长度/精度/小数位（如 numeric(2,-1)、numeric(2,3)），避免一个特殊规格阻断整批扫描；同名字段优先有注释，其次按表名排序，已有定义不覆盖
- **同步**：`sync_root` 手动触发，注释与完整类型规格分开选择，只对所选属性有差异的列执行 `COMMENT ON COLUMN` / `ALTER COLUMN TYPE`；按继承深度先父表后子表，父表变更会由 PostgreSQL 递归传播，处理继承列前重新读取规格以避免重复直接 ALTER；每次表操作独立事务，失败回滚且其他独立表继续，结果含变更属性。目标显式为 public，表名/列名双引号转义，类型短名采用白名单（支持单层 []）并校验参数范围后拼接，注释用单引号转义；空类型不能同步类型，不能两项均不选
- **边界**：不管理 PostgreSQL numeric 负 scale/scale 大于 precision，也不保存 interval 的范围限定（如 YEAR TO MONTH）或数组维数；遇到特殊类型规范先人工确认，不宜盲目全局同步。继承/分区父表 ALTER TYPE 会影响全部后代（包括其他 schema），父子之间不具备逐表隔离，执行前需确认整棵继承树
- DDL 在 `ToolsAI/zytool.sql` 末尾（含 `DROP TABLE IF EXISTS input_roots`），需手工执行（Rust 不做 migration）

## 工具域

### tool_categories / tools（工具中心展示数据，2026-11-06 新增）
- `tool_categories`：`id bigserial PK`、`name`、`description`、`sort_order`、`is_enabled`
- `tools`：`id bigserial PK`、`category_id FK→tool_categories CASCADE`、`name`、`icon`、`description`、`type`（frontend/backend）、`route`（前端路由，空串=未接入页面）、`sort_order`、`is_enabled`；唯一约束 `(category_id, name)`
- 初始数据 id 沿用原硬编码散值（1~14），序列已 setval；`/tools/categories` 双端均为读库优先（`is_enabled` 过滤、sort_order 排序），无 DB/查询失败回退硬编码

## 代码对应关系

- Rust 侧访问：`src/services/agents/store.rs`（ai_sessions/ai_messages）、`src/routes/auth.rs`（users/user_auths）
- 会话和消息的内部自增 ID 不对外，分别使用 `session_uuid`/`message_uuid`；模型选择则直接暴露并使用 `ai_models.id`
- 当前已用字段子集；`ai_prompt_templates`、`ai_session_settings`、`ai_message_feedbacks`、`parent_message_id` 等表/字段已建库但代码尚未接入，属预留扩展点

## 初始化前置条件

- Rust 启动不执行建表或 migration，需先准备数据库与表结构。
- `ToolsAI/zytool.sql` 不是独立的一键初始化脚本：使用了 `set_updated_at()` 但脚本未定义该函数；`gen_random_uuid()` 还依赖 PostgreSQL 版本或相应扩展环境。
