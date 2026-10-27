# 公共知识库（common-knowledge）

ZYTool 项目公共层知识：架构、模块地图、开发规范、命令、环境变量。

## 路由规则（判断问题归属）

| 问题类型 | 去向 |
|---|---|
| 架构、双后端兜底机制、统一响应结构、JWT 认证 | `resources/01-architecture.md` |
| 启动/构建/类型检查命令 | `resources/02-commands.md` |
| 添加新功能流程、代码风格、提交前检查 | `resources/03-dev-conventions.md` |
| 环境变量、配置项、密钥文件位置 | `resources/04-env-vars.md` |
| Rust 后端具体功能（路由/服务/DB） | `../../rust-backend-knowledge/SKILL.md` |
| Python 兜底后端具体功能 | `../../python-backend-knowledge/SKILL.md` |
| 前端页面/组件/API 调用 | `../../frontend-knowledge/SKILL.md` |
| 综合需求（跨前后端） | 公共层取架构与规范，模块层取业务细节 |

## 模块地图

- `Backup/tools_project_rust/`：Rust axum 主后端（端口 8000）
- `Backup/tools_project_py/`：Python FastAPI 兜底后端（端口 8001）
- `ZYTool/`：Vue 3 + TS 前端（开发端口 5050）

功能域：AI Agent 系统（Kimi + SSE 流式对话）、在线工具集（ViewFront 浏览器端 / ViewBack 服务器端）、MES 系统。
