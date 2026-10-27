# Python 后端知识库（backend-knowledge/python）

位置：`Backup/tools_project_py/`（FastAPI）。双后端部署期望使用 8001，但源码 `Settings.port` 和 `start_server.sh` 当前默认/硬编码为 **8000**；与 Rust 并行运行时需显式设置 `PORT=8001`，不要直接使用现有 `start_server.sh`。

## 定位

Rust 后端的**部分接口兜底**，不是完整等价实现。目标契约与当前差异见 `../common/01-api-contract.md`，逐路由覆盖矩阵见 `../common/03-route-coverage.md`。

当前缺失 Rust 的 Agents 会话/模型、账号绑定/QQ 登录、DB 健康检查接口；这些功能不能依赖 Python 降级。

## 目录地图

```
run.py               # 启动入口，uvicorn 使用 Settings.host/port
app/
├── main.py            # FastAPI 实例、CORS 中间件、路由挂载
├── core/              # config.py（Settings 读 .env）、auth.py、security.py、
│                      # exceptions.py、middleware.py、logging_config.py
├── api/v1/endpoints/  # agents/auth/health/map/misc/password/protected/regex/text/timestamp_router.py
├── schemas/           # base.py（当前 BaseResponse 仅 success/message）+ request/ + response/
├── services/
│   ├── Agents/        # deepseek_client.py、generate.py、tools/（calculate/search/time/weather）
│   ├── MCP/           # mcp_client.py、mcp_server.py（实验代码）
│   └── Tools/         # password/regex/text/timestamp_service.py
└── utils/             # text_utils.py
```

## 配置与启动

- 应用/JWT/CORS/天气配置由 `app/core/config.py` 的 `Settings` 从 `.env` 和环境变量读取
- LLM 客户端另行加载 `.env.development`：`KIMI_API_KEY/KIMI_BASE_URL/KIMI_MODEL` 优先，未配置回退 `DEEPSEEK_API_KEY`；模型名统一用 `deepseek_client.model`，不要硬编码
- Python 天气变量名为 `QWEATHER_API_KEY` / `QWEATHER_HOST`，不要与 Rust 的 `QWEATHER_KEY` 混写
- `rate_limit_*`、`max_request_size` 当前未接入有效中间件；`log_dir` 未传入日志初始化，均属声明或预留配置
- 启动：`python3 -m pip install -r requirements.txt`，然后 `PORT=8001 python3 run.py`

## 当前兼容性边界

- 多数工具及认证、健康、同步聊天接口不使用 `BaseResponse.data` 包装，部分连 `success` 也没有；中间件不解析 `X-Base-Info`，错误结构与 Rust 不同
- CORS 只放行 `127.0.0.1` 而非整个 127.*，且开启 credentials；与 Rust 并非完全一致
- 无 PostgreSQL 接入；认证使用进程内测试用户，注册数据重启丢失，不与 Rust 用户库同步
- Agents 请求没有 `model_id`；`session_id` 虽被接收但未使用。两个聊天接口均无鉴权、无会话历史、无持久化
- 流式 `/agents/chat` 调用 `generate_stream_no_tools`，不使用 Function Call；同步 `/agents/chat/sync` 才通过 `generate_sync` 使用工具
- 地图与搜索工具为模拟实现；分类中展示的 SQL 检查没有 Python 后端路由，实际由前端本地执行
- MCP 未挂载 HTTP API，属于实验代码；stdio client 会尝试启动 server，但 server 当前缺少 `mcp.run()`/模块主入口，且其工具清单与普通 Agents 不完全一致

## 维护约定

目前主要维护 Rust 侧，Python 侧按需同步。新增/修改接口时必须明确是已实现兼容、部分兼容还是暂未兜底，并同步更新公共契约与路由覆盖矩阵。
