# 100_Code 项目 Wiki（Repo Wiki）

本 Wiki 是 100_Code / ZYTool 项目的权威项目文档，由原 `.qoder` 旧知识库（common-knowledge / backend-knowledge / frontend-knowledge）于 2026-11-18 迁移重构而成，全部内容已对照当前代码核实。

## 项目简介

ZYTool 是一个全栈在线工具集合平台，主要功能域：

- **AI Agent 系统**：Kimi（Moonshot）+ SSE 流式对话 + Function Call（天气工具），会话/消息持久化
- **在线工具集**：ViewFront 浏览器端工具（JSON / Base64 / URL / 颜色 / 时间戳）+ ViewBack 服务器端工具（文本对比 / 正则测试 / 密码生成 / 地图路线）
- **MES 系统**：GCPM 规则管理（Mock 数据演示阶段）
- **后台管理**：数据库表只读浏览、field_roots 字段字根数据字典、双后端服务状态
- **财务小账本**：纯浏览器 localStorage 实现，不调后端

## 模块地图

| 模块 | 路径 | 技术栈 | 端口 |
|---|---|---|---|
| Rust 主后端 | `Backup/tools_project_rust/` | Rust（edition 2024）/ axum 0.8 / sqlx | 10003（`.env` PORT，源码默认 8000） |
| Python 兜底后端 | `Backup/tools_project_py/` | Python 3.9+ / FastAPI 0.108 | 10004（`.env` PORT，源码默认 8000） |
| 前端 | `ZYTool/` | Vue 3.5 + TS 5.9 + Vite 7 + Ant Design Vue 4.2 | 5050（package.json dev script） |
| 项目级连接配置 | `Config/database.env` | DB / Redis 连接（git 已忽略；供外部工具读取，三端代码不加载） | — |
| SQL 脚本 | `ToolsAI/` | `zytool.sql` 等 DDL 源文件 | — |

基础设施：PostgreSQL 18（`zytool` 库；Rust 主用，Python 懒加载连接池仅供 admin 接口）、Redis（无密码，双端客户端已接入）。

## 文档导航

| 文档 | 主题 |
|---|---|
| [01-系统概述](01-系统概述.md) | 总体架构、双后端兜底机制、统一响应结构、认证、Redis 消费场景 |
| [02-技术栈](02-技术栈.md) | 三端技术栈与依赖版本清单（已对照 manifest 核实） |
| [03-快速入门](03-快速入门.md) | 环境要求、三端启动命令、验证方式、常见问题 |
| [04-环境配置](04-环境配置.md) | 三端环境变量、Config/database.env、敏感文件位置 |
| [05-开发规范](05-开发规范.md) | 添加新功能流程、代码风格、提交前检查 |
| [06-前端架构](06-前端架构.md) | 前端目录地图、路由机制、服务层、主题样式规范、页面功能点 |
| [07-后端架构-Rust](07-后端架构-Rust.md) | Rust 目录地图、关键约定、认证现状、Agents 行为、DbClient 移植 |
| [08-后端架构-Python](08-后端架构-Python.md) | Python 定位、目录地图、配置启动、兼容性边界 |
| [09-API契约与路由](09-API契约与路由.md) | 统一响应 / SSE / CORS / 认证契约 + 双端路由覆盖矩阵 |
| [10-数据库设计](10-数据库设计.md) | PostgreSQL 全部表结构、field_roots 数据字典、初始化前置条件 |
| [11-UI动画设计规范](11-UI动画设计规范/README.md) | 动效取值标准、常用配方、评审标准（摘要 + 完整版 11 篇） |

## 路由规则（按问题类型查文档）

| 问题类型 | 去向 |
|---|---|
| 架构、双后端兜底机制、统一响应结构、JWT 认证 | 01-系统概述 |
| 启动 / 构建 / 类型检查命令 | 03-快速入门 |
| 环境变量、配置项、密钥文件位置 | 04-环境配置 |
| 添加新功能流程、代码风格、提交前检查 | 05-开发规范 |
| 前端页面 / 组件 / 路由 / 样式 / API 调用 | 06-前端架构 |
| Rust 路由 / 服务 / DB 具体实现 | 07-后端架构-Rust |
| Python 兜底接口实现与兼容边界 | 08-后端架构-Python |
| 接口契约（一致性 / SSE / CORS）、双端路由覆盖 | 09-API契约与路由 |
| 数据库表结构 | 10-数据库设计 |
| 动效编写 / 评审 / 组件库选型 | 11-UI动画设计规范 |
| 综合需求（跨前后端） | 01 取架构与规范，再按端取业务细节 |

## 维护约定

1. 完成代码修改后，同步更新本 Wiki 中受影响的文档，并在项目根 `修改履历.md` 追加记录（字段：修改时间、业务需求、涉及模块、修改内容、备注）。
2. **安全红线**：各端 `.env.*`、`Config/database.env` 及任何含密钥 / 连接串的文件，禁止把具体内容复制进 Wiki 或回答；只记录「位置与用途」。
3. 本 Wiki 取代旧 `.qoder/common-knowledge`、`.qoder/backend-knowledge`、`.qoder/frontend-knowledge` 三个目录（已于 2026-11-18 完成迁移后删除）。
