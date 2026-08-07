# 04 环境变量

## 后端（Backup/tools_project_rust/.env）

`.env` 由 `dotenvy` 加载（禁止提交仓库），字段对应 `core/config.rs` 的 `Settings`（envy 自动映射大写下划线命名）：

| 变量 | 用途 | 默认/说明 |
|---|---|---|
| `SECRET_KEY` | JWT 签名密钥 | 有开发用默认值，生产必改 |
| `DATABASE_URL` | PostgreSQL 连接串 | 空则无 DB 模式启动 |
| `DB_HOST/DB_PORT/DB_USER/DB_PASSWORD/DB_NAME/DB_POOL_SIZE` | 库连接分项 | localhost:5432/zytool |
| `QWEATHER_KEY` / `QWEATHER_HOST` | 和风天气 API | devapi.qweather.com |
| `QQ_APP_ID` / `QQ_APP_KEY` / `QQ_REDIRECT_URI` | QQ 互联 OAuth | 回调须与 QQ 后台一致 |
| `KIMI_API_KEY` | Kimi（Moonshot）密钥 | 空则 AI 聊天回退占位回复 |
| `KIMI_BASE_URL` | Kimi API 地址 | https://api.moonshot.cn/v1 |
| `KIMI_MODEL` | Kimi 模型 | kimi-k2-0905-preview |

常用默认值：`host=0.0.0.0`、`port=8000`、CORS 允许 `localhost:5000/5050/5173`。

## 前端（ZYTool/，Vite 环境变量）

见 `src/config/appConfig.ts`：

| 变量 | 用途 | 默认 |
|---|---|---|
| `VITE_RUST_API_BASE_URL` | Rust 后端地址 | http://localhost:8000（局域网共享时改为本机 LAN IP，如 http://192.168.77.154:8000） |
| `VITE_PYTHON_API_BASE_URL` | Python 后端地址 | http://localhost:8001（同上） |
| `VITE_AMAP_KEY` | 高德地图 JS API Key | 有内置开发 Key |

## 敏感文件位置（仅记录位置，禁止复制内容）

- `Backup/tools_project_rust/.env`：后端全部密钥（DB 密码、QQ Key、Kimi Key）
