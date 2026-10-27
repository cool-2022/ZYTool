# 04 环境变量

## Rust 后端（Backup/tools_project_rust/.env）

`.env` 由 `dotenvy` 加载（禁止提交仓库），字段对应 `core/config.rs` 的 `Settings`（envy 自动映射大写下划线命名）：

| 变量 | 用途 | 默认/说明 |
|---|---|---|
| `SECRET_KEY` | JWT 签名及模型密钥派生 | 有开发用默认值，生产必改 |
| `ACCESS_TOKEN_EXPIRE_MINUTES` | JWT 有效期 | 默认 30 分钟；算法固定 HS256，`ALGORITHM` 配置未生效 |
| `DATABASE_URL` | PostgreSQL 连接串 | 非空优先；为空则拼接 `DB_*`，连接失败后以无 DB 模式启动 |
| `DB_HOST/DB_PORT/DB_USER/DB_PASSWORD/DB_NAME/DB_POOL_SIZE` | 库连接分项 | 默认 localhost:5432/zytool |
| `QQ_APP_ID` / `QQ_APP_KEY` / `QQ_REDIRECT_URI` | QQ 互联 OAuth | 回调须与 QQ 后台及前端 `/` 回调一致；代码默认回调路径仍为 `/login`，需显式核对 |
| `KIMI_API_KEY` | Kimi（Moonshot）密钥 | 无可用 DB 模型且此项为空时，Rust 返回占位回复 |
| `KIMI_BASE_URL` | Kimi API 地址 | https://api.moonshot.cn/v1 |
| `KIMI_MODEL` | Kimi 模型 | kimi-k2-0905-preview |
| `RUST_LOG` / `LOG_LEVEL` | tracing 日志过滤 | `RUST_LOG` 优先；守护模式文件日志来自 shell 重定向 |

常用默认值：`host=0.0.0.0`、`port=8000`。CORS 使用精确清单 + 本机/私网/`.local` 来源规则，见后端公共契约。

**仅声明、当前未生效的字段**：`DEBUG`、`MAX_REQUEST_SIZE`、`LOG_FILE`、`LOG_DIR`、`RATE_LIMIT_*`、`ALGORITHM`、`QWEATHER_KEY/QWEATHER_HOST`。不要据此认为已有限流、请求体限制、文件日志、可配置 JWT 算法或天气业务。

## Python 后端（Backup/tools_project_py/）

Python 有两套配置加载路径：

- `app/core/config.py` 的 `Settings` 从 `.env` 和环境变量读取应用、JWT、CORS、天气配置；天气变量名为 `QWEATHER_API_KEY` / `QWEATHER_HOST`
- `services/Agents/deepseek_client.py` 另行从 `.env.development` 和环境变量读取 `KIMI_API_KEY/KIMI_BASE_URL/KIMI_MODEL`，未配置密钥时回退 `DEEPSEEK_API_KEY`
- `PORT` 默认 8000；与 Rust 并行时必须显式设为 8001
- `RATE_LIMIT_*`、`MAX_REQUEST_SIZE` 未接入有效中间件；`LOG_DIR` 未传入日志初始化

## 前端（ZYTool/，Vite 环境变量）

见 `src/config/appConfig.ts`：

| 变量 | 用途 | 默认 |
|---|---|---|
| `VITE_RUST_API_BASE_URL` | Rust 后端地址 | 默认跟随访问 hostname（`http://${hostname}:8000`），localhost/IP 打开自动适配；设值则固定 |
| `VITE_PYTHON_API_BASE_URL` | Python 后端地址 | 同上（8001） |
| `VITE_AMAP_KEY` | 高德地图 JS API Key | 有内置开发 Key |

`.env.example` 中的 `VITE_MAP_CENTER` 当前未被源码读取，设置它不会改变地图中心；两个地图页面的默认中心点仍在组件内。

## 敏感文件位置（仅记录位置，禁止复制内容）

- `Backup/tools_project_rust/.env`：Rust 后端密钥（DB、QQ、Kimi 等）
- `Backup/tools_project_py/.env`：Python 应用/JWT/天气配置
- `Backup/tools_project_py/.env.development`：Python LLM 密钥与模型配置
- `ZYTool/.env`：前端本地 Vite 配置（可能包含高德 Key）
