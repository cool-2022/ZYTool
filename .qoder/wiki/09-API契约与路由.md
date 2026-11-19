# 09 API 契约与路由

> 合并自原后端公共知识库 `01-api-contract.md` 与 `03-route-coverage.md`，2026-11-18 核实。

本文记录双端的**目标契约**与当前差异。Rust 是主后端；Python 目前只实现部分兼容接口，不能把"存在降级逻辑"理解为所有功能都有完整兜底。改动接口时先查本文「路由覆盖矩阵」，再检查另一端实现。

## 接口一致性与降级

- 目标：统一前缀 `/api/v1`，两端接口路径、请求/响应结构保持一致
- 前端普通请求走 `requestWithFallback`：网络错误/超时/5xx 自动降级 Python；**4xx 不降级**
- Python 缺失的接口在降级后返回 404；Agents 会话/模型、账号绑定/QQ 登录、DB 健康检查目前没有 Python 兜底；admin 后台管理接口已于 2026-11-06 全部补齐
- 新增/修改后端接口时原则上两端同步实现；若暂不同步，必须在本文路由覆盖矩阵标注能力差异

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

- 目标：精确来源清单 + 私网/本机正则放行；覆盖 localhost、127.*、192.168.*、10.*、172.16-31.*、`.local` 主机，端口不限（Rust 实现见 `main.rs` 的 `is_private_origin`）
- **当前差异**：Python 只放行 `127.0.0.1` 而非整个 127.*；Python 开启 credentials，Rust 未开启

## 认证

- 使用 JWT Bearer Token；前端收到 401 后清除本地认证并跳转 `/?redirect=...`
- Rust 使用 PostgreSQL 用户数据；Python 当前使用进程内测试用户，重启丢失注册数据，也不与 Rust 用户库同步

## 数据库

- 双端均接入 PostgreSQL（`zytool` 库），表结构见 [10-数据库设计](10-数据库设计.md)；Python 自 2026-11-06 起通过 asyncpg 懒加载连接池接入（`app/core/db.py`，`DATABASE_URL` 配置，json/jsonb 自动解码），当前仅供 admin 后台管理接口使用
- Rust 启动时只初始化一次连接池，失败后以无 DB 模式运行；DB 恢复后通常仍需重启进程。Agent 和多数认证接口此时返回 500，DB 无关工具仍可使用；Python 侧为懒加载，DB 恢复后无需重启
- Rust 启动不自动建表或执行 migration，初始化前置条件见 [10-数据库设计](10-数据库设计.md)

---

## 路由覆盖矩阵

除根健康检查外，以下路径均带 `/api/v1` 前缀。Python 栏"缺失"表示 Rust 故障降级后会返回 404，并不代表该功能有兜底。

### 健康检查

| 方法 | 路径 | Rust | Python |
|---|---|---|---|
| GET | `/health`（根路径） | 公开 | 已实现，原始业务结构 |
| GET | `/health/` | 公开 | Python 对应路径为 `/health`（无尾部 `/`） |
| GET | `/health/info` | 公开 | 已实现，原始业务结构 |
| GET | `/health/db` | 公开；无池或 ping 失败也返回 HTTP 200 + `success=false` | 缺失 |

> 已知怪癖（2026-11-05 实测）：`/api/v1/health/`（带尾部斜杠）在 Rust 侧 500「missing BaseInfo extension」——nest 内 `/` 路由丢失 base_info 中间件扩展；`/api/v1/health`（无斜杠）正常。前端 `healthCheck()` 用无斜杠路径，不受影响。

### 认证与账号绑定

| 方法 | 路径 | Rust 认证 | Python |
|---|---|---|---|
| POST | `/auth/login` | 公开；同用户名连续失败 5 次锁定 600s（Redis，无 Redis 不生效） | 已实现；内存测试用户 |
| POST | `/auth/register` | 公开 | 已实现；进程内保存，重启丢失 |
| GET | `/auth/me` | JWT | 已实现；原始用户结构 |
| POST | `/auth/logout` | JWT；token 摘要写 Redis 黑名单（TTL=剩余有效期），此后 Rust 侧请求 401 | 已实现 |
| GET | `/auth/bindings` | JWT | 缺失 |
| POST | `/auth/bind/phone` | JWT | 缺失 |
| POST | `/auth/bind/qq` | JWT | 缺失 |
| POST | `/auth/bind/wechat` | JWT | 缺失 |
| GET | `/auth/qq/auth-url` | 公开 | 缺失 |
| POST | `/auth/qq/login` | 公开 | 缺失 |

Rust 用户数据依赖 PostgreSQL；Python 不与该用户库同步。

### Agents

以下 Rust 接口均要求 JWT 和 DB；Python 两个聊天接口当前不要求 JWT、不访问 DB。Rust 侧 `/agents/chat` 与 `/agents/chat/sync` 按用户限流（默认 20 次/分钟，Redis 分钟桶，超限 429，无 Redis 放行）。Rust 两个聊天接口自 2026-11-10 起支持 Function Call（天气工具，流式端内部两轮请求对前端透明）。

| 方法 | 路径 | Python |
|---|---|---|
| GET | `/agents/sessions` | 缺失 |
| POST | `/agents/sessions` | 缺失 |
| PATCH | `/agents/sessions/{session_id}` | 缺失 |
| DELETE | `/agents/sessions/{session_id}` | 缺失 |
| GET | `/agents/sessions/{session_id}/messages` | 缺失 |
| GET | `/agents/models` | 缺失 |
| POST | `/agents/chat` | 已实现 SSE；无状态单轮、无工具 |
| POST | `/agents/chat/sync` | 已实现；返回 `{reply}`，支持 Function Call |

### 后台管理

以下接口均要求 JWT 且 `roles` 含 `admin`（非管理员返回 403）。**Python 已全部实现兜底（2026-11-06，asyncpg 连接池 + jose 校验同一 SECRET_KEY）**：路径、参数、响应结构与 Rust 对齐，含字根同步的 DDL 操作；`/admin/services` 在 Python 侧返回 Python 自身身份（name 带 `(Python)` 后缀）。

#### 数据库表（只读）

表名经白名单字符校验＋information_schema 存在性确认后才拼接 SQL 标识符，只读浏览、无写操作。

| 方法 | 路径 | 说明 |
|---|---|---|
| GET | `/admin/tables` | public 模式表清单（名称/注释/行数估计） |
| GET | `/admin/tables/{table}/columns` | 字段、类型、可空、主键、默认值、注释 |
| GET | `/admin/tables/{table}/rows` | `page`/`page_size`（默认 20、上限 200）分页；`keyword` 对整行 `to_jsonb` 文本 ILIKE 模糊匹配 |

#### 字根管理（field_roots 字段数据字典）

引用关系不落表：按 `field_name` 与 public 下普通表/分区表的 pg_attribute 同名列实时关联（排除 field_roots）；响应中 `refs` 带基础短名、长度/精度/小数位、`type_expr`、注释及 `type_synced/comment_synced/synced/diffs`。差异逐属性报告，数值方向为「引用列 ≠ 字根标准」。

| 方法 | 路径 | 说明 |
|---|---|---|
| GET | `/admin/roots` | `page`/`page_size`（默认 20、上限 200）分页；数据源为**现有表全部字段实时清单**（LEFT JOIN field_roots 标注 `registered`/`id`），非仅字典记录；`id/created_at/updated_at` 通用技术字段排除；`keyword` 匹配字段名/注释/备注；每条带 `refs` 引用明细（含 type_expr/diffs） |
| POST | `/admin/roots/scan` | 扫描 public 普通表/分区表字段，排除 field_roots；过滤不满足字典 CHECK 的类型参数（如 numeric 负 scale 或 scale 大于 precision），同名优先有注释，其次表名排序，`ON CONFLICT (field_name) DO NOTHING` 不覆盖已有；返回 `inserted`/`total_roots` |
| POST | `/admin/roots` | 手动新增，`field_name` 必填；`data_type` 为 PG 短名白名单（可带 []）；`data_length/data_precision/data_scale` 可空并按类型校验；numeric(p) 小数位缺省归一为 0 |
| PUT | `/admin/roots/{id}` | 只更新字典记录（`updated_at` 置 now()），引用表字段不变 |
| DELETE | `/admin/roots/{id}` | 只删字典记录，引用表字段不受影响 |
| POST | `/admin/roots/{id}/sync` | `sync_comment` 同步注释 / `sync_type` 同步类型+长度/精度/小数位，仅执行所选属性有差异的列；空类型不可同步类型、至少选择一项；父表优先，继承列重读后处理，父表类型变更会递归影响后代；每次操作独立事务，失败回滚，逐表返回 ok/message（含变更属性） |

#### 服务状态

| 方法 | 路径 | 说明 |
|---|---|---|
| GET | `/admin/services` | Rust 自身状态：名称/版本/描述/DB ping 结果/进程运行时长/上报时间；Python 状态由前端直连 `GET /health` 探测 |

### 受保护示例

| 方法 | 路径 | Rust 认证 | Python |
|---|---|---|---|
| GET | `/protected/data` | JWT | 已实现 |
| GET | `/protected/admin` | admin；权限不足返回 HTTP 200 + `success=false` | 已实现 |

### 工具

以下 Rust 接口均为公开接口，不依赖用户登录；Python 已实现相同路径，但部分响应结构与模拟行为不同。Rust 侧 `/tools` 整体挂 IP 限流中间件（默认 60 次/分钟，Redis 分钟桶，超限 429，无 Redis 放行）。

| 方法 | 路径 |
|---|---|
| GET | `/tools/categories` |
| GET | `/tools/system-info` |
| POST | `/tools/text/process` |
| POST | `/tools/text/compare` |
| POST | `/tools/regex/test` |
| POST | `/tools/password/generate` |
| POST | `/tools/timestamp/convert` |
| POST | `/tools/map/route` |
| GET | `/tools/map/route/test` |

`GET /tools/categories` 自 2026-11-06 起读 `tool_categories`/`tools` 表（含 `route` 字段驱动前端跳转，is_enabled+sort_order 过滤排序），无 DB 或失败时回退硬编码；2026-11-10 起 Rust 侧优先读 Redis 缓存（`tools:categories`，TTL 300s，只缓存读库结果，改分类后最长 5 分钟生效）；包含纯前端及待开发条目，不能当作后端已实现能力清单。两端地图路线均为 mock 直线插值，`waypoints` 不参与计算；SQL 合理性检查在前端本地执行，没有对应后端路由。
