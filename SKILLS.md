# Skills - 项目技能配置

### 关键文件

| 文件 | 职责 |
|------|------|
| `api/v1/agents/router.py` | API 路由、流式处理、Function Call 逻辑 |
| `services/public/deepseek_client.py` | DeepSeek SDK 封装 |
| `services/public/tools/__init__.py` | 工具定义和执行 |
| `core/config.py` | CORS 配置 |

---

## 注意
- 前后端服务需要手动启动，agent 不需要自动启动服务
- 后端端口: 8000
- 前端端口: 5050
