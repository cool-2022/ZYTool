# Python 后端知识库（python-backend-knowledge）

位置：`Backup/tools_project_py/`（FastAPI，端口 8001）

## 定位

Rust 后端的兜底实现：接口路径与响应结构保持与 Rust 侧一致（`/api/v1` 前缀、`BaseResponse` 结构），前端在网络错误/超时/5xx 时自动降级到本服务。

## 约定

- 新增/修改后端接口时，原则上 Rust 与 Python 两侧需保持路径与响应结构一致，否则降级会失败
- 流式接口（`/agents/chat`）同样输出 SSE（`data: ...`，`[DONE]` 结束）
- LLM 配置：`.env.development` 中 `KIMI_API_KEY/KIMI_BASE_URL/KIMI_MODEL` 优先，未配置回退 `DEEPSEEK_API_KEY`（deepseek_client.py）；模型名统一用 `deepseek_client.model`，不要硬编码

## 备注

详细结构待补充（目前主要维护 Rust 侧，Python 侧按需同步）。
