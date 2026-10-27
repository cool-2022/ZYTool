# 03 双后端路由与降级覆盖

除根健康检查外，以下路径均带 `/api/v1` 前缀。Python 栏“缺失”表示 Rust 故障降级后会返回 404，并不代表该功能有兜底。

## 健康检查

| 方法 | 路径 | Rust | Python |
|---|---|---|---|
| GET | `/health`（根路径） | 公开 | 已实现，原始业务结构 |
| GET | `/health/` | 公开 | Python 对应路径为 `/health`（无尾部 `/`） |
| GET | `/health/info` | 公开 | 已实现，原始业务结构 |
| GET | `/health/db` | 公开；无池或 ping 失败也返回 HTTP 200 + `success=false` | 缺失 |

## 认证与账号绑定

| 方法 | 路径 | Rust 认证 | Python |
|---|---|---|---|
| POST | `/auth/login` | 公开 | 已实现；内存测试用户 |
| POST | `/auth/register` | 公开 | 已实现；进程内保存，重启丢失 |
| GET | `/auth/me` | JWT | 已实现；原始用户结构 |
| POST | `/auth/logout` | JWT；仅返回成功，无服务端撤销 | 已实现 |
| GET | `/auth/bindings` | JWT | 缺失 |
| POST | `/auth/bind/phone` | JWT | 缺失 |
| POST | `/auth/bind/qq` | JWT | 缺失 |
| POST | `/auth/bind/wechat` | JWT | 缺失 |
| GET | `/auth/qq/auth-url` | 公开 | 缺失 |
| POST | `/auth/qq/login` | 公开 | 缺失 |

Rust 用户数据依赖 PostgreSQL；Python 不与该用户库同步。

## Agents

以下 Rust 接口均要求 JWT 和 DB；Python 两个聊天接口当前不要求 JWT、不访问 DB。

| 方法 | 路径 | Python |
|---|---|
| GET | `/agents/sessions` | 缺失 |
| POST | `/agents/sessions` | 缺失 |
| PATCH | `/agents/sessions/{session_id}` | 缺失 |
| DELETE | `/agents/sessions/{session_id}` | 缺失 |
| GET | `/agents/sessions/{session_id}/messages` | 缺失 |
| GET | `/agents/models` | 缺失 |
| POST | `/agents/chat` | 已实现 SSE；无状态单轮、无工具 |
| POST | `/agents/chat/sync` | 已实现；返回 `{reply}`，支持 Function Call |

## 受保护示例

| 方法 | 路径 | Rust 认证 | Python |
|---|---|---|---|
| GET | `/protected/data` | JWT | 已实现 |
| GET | `/protected/admin` | admin；权限不足返回 HTTP 200 + `success=false` | 已实现 |

## 工具

以下 Rust 接口均为公开接口，不依赖用户登录；Python 已实现相同路径，但部分响应结构与模拟行为不同。

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

`GET /tools/categories` 是硬编码展示目录，包含纯前端及待开发条目，不能当作后端已实现能力清单。两端地图路线均为 mock 直线插值，`waypoints` 不参与计算；SQL 合理性检查在前端本地执行，没有对应后端路由。
