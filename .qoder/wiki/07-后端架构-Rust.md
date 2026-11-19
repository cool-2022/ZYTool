# 07 后端架构 - Rust

> 迁移自原后端知识库 `backend-knowledge/rust/SKILL.md`，2026-11-18 核实。

位置：`Backup/tools_project_rust/`（axum，源码默认端口 8000；当前 .env `PORT=10003`）

双端目标契约与当前差异见 [09-API契约与路由](09-API契约与路由.md)；数据库表结构见 [10-数据库设计](10-数据库设计.md)。

## 目录地图

```
src/
├── main.rs            # 入口：初始化配置/DB/Redis/用户缓存/中间件，挂载路由（/health + /api/v1 nest），统一 404 兜底
├── core/              # config.rs（Settings/envy）、auth.rs（JWT/CurrentUser）、
│                      # db.rs（sqlx 连接池）、crypto.rs（模型密钥加解密）、redis.rs（Redis 客户端与通用助手）、
│                      # error.rs（AppError）、middleware.rs
│                      # dbclient/：C++ BM2::Data::DbClient 的移植（DbConnection/DbCommand/
│                      #   ParameterCollection/DataTable/DbValue），基于 sqlx 池，@name 命名参数
│                      #   执行时改写为 $n；预留未接线，模块级 #![allow(dead_code)]，见下节
├── models/            # 请求/响应模型（serde），按域分文件（agents/auth/text/...）；
│                      #   ei.rs 为 EI 平台 EiInfo 数据交换结构（blocks: meta.columns+rows、
│                      #   EDInfo 定长数组、三段式 get/set），与前端 ZYTool/src/utils/ei/ 对应；
│                      #   预留未接线，文件级 #![allow(dead_code)]，接入后移除
├── routes/            # 路由层
│   ├── agents/        #   AI 助手：mod.rs（挂载）、sessions.rs（会话/模型）、chat.rs（流式/同步）
│   ├── tools/         #   工具路由（按工具分文件，mod.rs 挂载）
│   └── admin.rs / auth.rs / health.rs / protected.rs   # admin.rs 为后台管理（表只读浏览 + 字根 CRUD + 服务状态）
└── services/          # 业务逻辑
    ├── agents/        #   AI 助手：store.rs（DB 持久化）、llm.rs（模型客户端，支持 Function Call）、tools.rs（工具定义/执行：天气）
    ├── admin.rs       #   后台管理：表清单/表结构/分页行查询（sqlx json feature）、field_roots 字段字根（基础短名+长度/精度/小数位、type_expr、逐属性差异、扫描过滤超范围精度/按名关联引用/父表优先及继承列重读/按表事务手动同步）、服务状态（进程运行时长 START_TIME + DB ping）
    └── tools/         #   工具服务（map/password/regex/text/timestamp）
```

## 关键约定

- 普通 JSON handler 返回 `Json<BaseResponse<T>>`；SSE 聊天除外。错误用 `core::error` 的 `bad_request/unauthorized/not_found/internal_error`，错误响应不透传 `BaseInfo`
- 配置只走 `core::config::SETTINGS`；新增配置 = `Settings` 加字段 + `.env` 加变量。字段存在不代表功能已接线，请求体限制、文件日志和 JWT algorithm 当前均为预留配置（限流以 `LOGIN_*` / `CHAT_*` / `TOOLS_*` 实际生效）
- 对外模型列表来自 `GET /agents/models`；首页等静态文案不能作为实际模型提供方依据

## 认证与账号绑定现状

- 密码登录只按 `users.nickname` 查询，请求字段虽叫 `username`，不支持邮箱或手机号登录；注册密码最少 6 位，成功直接返回 token，注册模型中的 `email` 当前未保存
- 启动时加载用户到内存缓存，缓存未命中才查 DB；注册和 QQ 登录会更新缓存。直接修改 DB 后，缓存及既有 token 不一定立即反映新密码、角色或状态
- JWT 含 `sub/user_id/roles/exp`，算法固定 HS256；`CurrentUser` 只验证 token，不在每次请求时刷新用户状态
- `logout` 有黑名单：`auth:blacklist:{sha256(token)}` 写 Redis（TTL=剩余有效期），`CurrentUser` 提取器命中即 401（无 Redis 时不生效）
- 登录防爆破：`login:fail:{用户名}` Redis 计数，默认 5 次失败锁定 600s，锁定期间 401（无 Redis 时不生效）
- 手机绑定只校验格式并直接标记已验证，没有验证码流程；`bind/qq`、`bind/wechat` 直接接受 `open_id`，不做服务端 OAuth 校验
- 只有 QQ 实现 OAuth 登录；`state` 生成后未保存和校验。自动建用户与写绑定不是显式事务，昵称获取失败不阻塞登录

## Agents 行为

- 模型选择：显式 `model_id` 优先（按 id 加载并解密，**允许选择停用模型**）；否则取 `ai_models` 中 `is_enabled=true` 且 priority 最高项（60s 缓存）；最后回退 `.env` 的 `KIMI_*`
- `model_id` 会写入会话，但后续请求未再次携带时**不会读取会话的 `model_id`**，而会重新选择全局默认模型
- 上下文取最近 20 条历史；系统提示词是 `llm.rs` 常量，未使用 `ai_sessions.system_prompt` 或 `ai_prompt_templates`
- 上游请求体只有 `model/messages/stream`（+ Function Call 时带 `tools`）；模型 `max_tokens_default/config` 与会话 temperature/top_p/max_tokens 均未接入。部分模型（如 k3-256k）不允许自定义 temperature
- Function Call（2026-11-10 起，`services/agents/tools.rs`）：天气工具（和风 geo/now 两次调用，Redis 缓存与 Python 共用 `weather:now`/`weather:city` key）；流式端内部两轮请求对前端透明，非流式端两轮合并 tokens
- 两种聊天在未传 `session_id` 时都会自动创建会话，标题取首条消息前 20 个字符并追加 `...`，但响应/SSE 不回传新会话 UUID；同步接口成功返回 `{reply}`
- SSE 错误、中断持久化及 token 统计见 [09-API契约与路由](09-API契约与路由.md)
- 会话/消息使用 `ai_sessions`、`ai_messages`；删除为单个数据修改 CTE：先保存活跃消息的精简快照到 `ai_session_archives`，再物理删除会话，关联数据随 CASCADE 清除
- 会话历史 Redis List 缓存：`agents:history:{session_id}`，RPUSH 增量追加 + LTRIM 保留 50 条，TTL 1800s，未命中回源 DB 并回填
- 模型延迟参考（2026-08 特定环境实测，非稳定保证）：k3-256k TTFB 6~12s、k3 ~3s、kimi-for-coding-highspeed ~0.8s

## 工具与运行边界

- 工具目录是 `tool_categories`/`tools` 表驱动（读库优先，无 DB/失败回退硬编码；Rust 侧有 `tools:categories` Redis 缓存 TTL 300s），包含纯前端及待开发条目，不等于后端能力清单
- 地图路线为 mock 直线插值，固定 30 km/h 估算，`waypoints` 未使用
- 和风天气：GeoAPI 与天气共用 `QWEATHER_HOST`（`{host}/geo/v2/city/lookup`，旧 geoapi.qweather.com/v2 路径已下线），新控制台账号必须配专属 API Host
- DB 只在启动时初始化一次，失败后可运行 DB 无关工具，但不会因 DB 稍后恢复而自动重连初始化
- `start.sh` 使用 debug 二进制看门狗，只有二进制不存在才自动 `cargo build`；修改源码后需显式重新 build，不能靠看门狗重启加载新代码

## DbClient（core/dbclient，C++ CDbConnection/CDbCommand 移植）

- 来源：SBMES C++ `Include/DbClient/BM2_DbConnection.h`、`BM2_DbCommand.h`、`BM2_DataParameter*.h`；目标是让 MES 业务函数（`CDbCommand cmd(conn); SetCommandText; Parameters.Set; ExecuteReader/Read/GetString; Close`）可按同样节奏改写为 Rust
- 连接：`DbConnection::get_current()` 包装 `core::db` 全局池（未连接返回 `DbError`）；`DbConnection::create(&ConnectOptions)` 可另建池并登记为当前连接；`get_xa_connection()` 等同 `get_current`，PG 下只有本地事务
- 事务：`begin_transaction/commit/rollback` 把 `sqlx::Transaction` 存在连接内的 `tokio::Mutex` 中，期间该连接上的所有命令都走事务；不支持嵌套
- 命令：`DbCommand::new(&conn)` / `with_text`；`CommandType::{Text, StoredProcedure(CALL name($1..)), TableDirect(SELECT *)}`；`execute_non_query -> u64`、`execute_query(start, max) -> DataTable`、`execute_reader -> 行数` + `read()` 游标（整表拉入内存）、`execute_scalar -> Decimal`（无行/NULL 为 0）
- 参数：SQL 里写 `@name`，`cmd.parameters.set("name", value)`；名字大小写不敏感，跳过单引号字符串和 `@@`，同名复用同一 `$n`，引用未设置参数报错；`ParamDirection` 仅记录，PG 无 OUT 参数
- 取值：列索引 0 基；`get_i32/get_string/get_decimal/...` 对 NULL 返回默认值，类型不可转换报错，判空用 `is_db_null`；`get_datetime` 返回 `Option`；`get_as::<T>`/`get_into` 对应 C++ 泛型 `Get`；`fetch_into(&mut DataTable)` / `fetch_into_from(table, col_pos)` / `fetch_into_row(table, idx)` 对应 `Fetch/Get(CDataTable&)`
- 解码：按 PG 类型名映射（INT2/4/8、FLOAT4/8、NUMERIC→Decimal、TEXT 族、TIMESTAMP/TIMESTAMPTZ(转本地)/DATE、BYTEA、UUID/JSON/TIME 文本化），数组等未覆盖类型直接报错；空结果集没有列信息
- `get_schema(collection, &[Option<&str>])` 支持 Tables/Columns/Schemas/Databases（information_schema），对应 ADO.NET 限制项位置
- 依赖：为此新增 `rust_decimal` 及 sqlx `rust_decimal` feature；`DbError` 可 `?` 转 `AppError`（500）

## 扩展入口

- 新工具：见 [05-开发规范](05-开发规范.md)
- 新 LLM 提供方：`services/agents/` 下新建客户端，参照 `llm.rs` 的 `is_configured/chat_stream/chat_complete` 模式

## 相关文档

- [10-数据库设计](10-数据库设计.md)：PostgreSQL 全部表结构及初始化前置条件
- [09-API契约与路由](09-API契约与路由.md)：完整路由矩阵与 Python 降级覆盖
