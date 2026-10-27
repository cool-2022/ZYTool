# 03 开发约定

## 添加新功能流程

### Rust 后端工具

1. 在 `src/services/tools/` 创建服务文件实现业务逻辑，并在 `src/services/tools/mod.rs` 导出
2. 在 `src/models/` 创建对应的请求/响应模型（serde `Deserialize`/`Serialize`）
3. 在 `src/routes/tools/` 创建路由文件并在 `src/routes/tools/mod.rs` 中挂载

### Rust 后端 Agent 功能

在 `src/services/agents/` 与 `src/routes/agents/` 对应文件中扩展；新业务可新建文件并在各自 `mod.rs` 导出/挂载。例：新 LLM 提供方在 `services/agents/` 下新建客户端文件。

### 前端工具

1. 在 `src/views/ViewFront/`（浏览器端）或 `src/views/ViewBack/`（服务器端）创建 Vue 组件
2. 在 `src/router/index.ts` 添加路由
3. 在 `src/services/api.ts` 的 `ApiService` 中添加 API 调用（统一走 `requestWithFallback`；流式请求参照 `chatStream`）

## 代码风格

- Rust：axum 路由 + handler 返回 `Json<BaseResponse<T>>`；配置统一走 `core::config::SETTINGS`（`once_cell::Lazy` + `envy` 读环境变量）；异步基于 tokio；DB 用 sqlx（PostgreSQL，可选）
- 前端：Vue 3 Composition API + TypeScript，UI 用 Ant Design Vue；HTTP 统一走 `ApiService`；流式用 `adapter: 'fetch'` + `responseType: 'stream'` 手动解析 SSE
- 遵循现有文件的代码风格与目录组织方式，不要随意引入新依赖
- 及时清理无用代码（未引用的文件/导出/变量），保持代码简洁

## 注意事项

- 数据库未连接时后端以无 DB 模式启动，涉及 DB 的功能需做降级处理
- 项目目前缺少单元测试与完整用户系统，改动时注意兼容性
- 不要提交任何密钥、API Key 或 `.env` 文件
