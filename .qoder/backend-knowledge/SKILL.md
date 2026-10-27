# 后端知识库（backend-knowledge）

Rust 主后端 + Python 兜底后端的统一知识入口。

## 路由规则

| 问题类型 | 去向 |
|---|---|
| 双后端接口一致性、统一响应结构、SSE 流式规范、CORS | `common/01-api-contract.md` |
| 数据库表结构（PostgreSQL，当前仅 Rust 接入） | `common/02-database-schema.md` |
| 完整路由、认证要求与 Python 降级覆盖 | `common/03-route-coverage.md` |
| Rust 路由/服务/DB 具体实现 | `rust/SKILL.md` |
| Python 兜底后端具体实现 | `python/SKILL.md` |
| 项目级架构/命令/开发规范/环境变量 | `../common-knowledge/SKILL.md` |

## 模块地图

- `Backup/tools_project_rust/`：Rust axum 主后端（端口 8000）
- `Backup/tools_project_py/`：Python FastAPI 部分兜底后端（部署预期 8001，源码默认 8000）

核心原则：**目标上两端接口路径与响应结构应保持一致**，但当前 Python 仅部分兼容；改动时同时维护目标契约、实际差异与路由覆盖矩阵。
