# Python 后端知识库（backend-knowledge/python）

位置：`Backup/tools_project_py/`（FastAPI，端口 8001）

## 定位

Rust 后端的兜底实现。双端必须遵守的公共契约（接口一致性/响应结构/SSE/CORS）见 `../common/01-api-contract.md`。

## 目录地图

```
run.py               # 启动入口
app/
├── main.py            # FastAPI 实例、CORS 中间件、路由挂载
├── core/              # config.py（pydantic-settings 读 .env.development）、auth.py、
│                      # security.py、exceptions.py、middleware.py、logging_config.py
├── api/v1/endpoints/  # 路由层（agents/auth/health/map/misc/password/protected/regex/text/timestamp_router.py）
├── schemas/           # base.py（BaseResponse）+ request/ + response/（按域分文件）
├── services/
│   ├── Agents/        # deepseek_client.py（LLM 客户端）、generate.py、tools/（calculate/search/time/weather）
│   ├── MCP/           # mcp_client.py、mcp_server.py
│   └── Tools/         # password/regex/text/timestamp_service.py
└── utils/             # text_utils.py
```

## Python 侧专有约定

- LLM 配置：`.env.development` 中 `KIMI_API_KEY/KIMI_BASE_URL/KIMI_MODEL` 优先，未配置回退 `DEEPSEEK_API_KEY`（deepseek_client.py）；模型名统一用 `deepseek_client.model`，不要硬编码
- CORS 实现：`config.py` 的 `cors_origins` 清单 + `cors_origin_regex`，行为与 Rust 侧一致

## 备注

目前主要维护 Rust 侧，Python 侧按需同步；新增/修改接口时注意双端一致性（见公共契约）。
