# Skills - 项目技能配置

## 后端 (Backend)

### 启动服务
```bash
cd project
python -m uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
```

### 运行测试
```bash
cd project && pytest
pytest app/tests/test_text.py::test_text_process_json_format
```

### 代码检查
```bash
cd project
flake8 .
black .
mypy .
```

---

## 前端 (ZYTool)

### 安装依赖
```bash
cd ZYTool && npm install
```

### 启动开发服务器
```bash
npm run dev
```

### 构建
```bash
npm run build
```

---

## DeepSeek AI 对话

### 流式对话 (支持 Function Call)
- POST `/api/v1/agents/chat`

### 同步对话 (支持 Function Call)
- POST `/api/v1/agents/chat/sync`

### 可用工具
- `get_current_time` - 获取当前时间
- `calculate` - 数学计算
- `search_web` - 网页搜索

---

## POST /api/v1/agents/chat 执行顺序详解

### 请求流程

```
1. 前端发送 POST 请求
      │
      ▼
2. FastAPI 接收请求 (router.py chat 函数)
      │
      ├── 验证 message 是否为空
      │
      ▼
3. 返回 StreamingResponse，调用 generate_stream()
      │
      ▼
4. generate_stream() 执行:
      │
      ├── 4.1 调用 deepseek_client.chat(message, tools=AVAILABLE_TOOLS)
      │         │
      │         ▼
      │    deepseek_client.py chat() 方法
      │         │
      │         ├── 构造 messages (system + user)
      │         ├── 附加 tools 参数
      │         │
      │         ▼
      │    调用 DeepSeek API (非流式)
      │         │
      │         ▼
      │    返回 result = {content, tool_call}
      │         │
      │    返回到 generate_stream
      │
      ├── 4.2 判断 result["tool_call"] 是否存在
      │
      ├──▶ 有 tool_call (需要调用工具):
      │    │
      │    ├── 4.2.1 解析 tool_call
      │    │    - tool_name: 工具名称
      │    │    - arguments: 工具参数
      │    │
      │    ├── 4.2.2 执行工具: execute_tool(tool_name, arguments)
      │    │    │
      │    │    ├── tools/__init__.py 中的函数
      │    │    ├── get_current_time()
      │    │    ├── calculate(expression)
      │    │    └── search_web(query)
      │    │
      │    ├── 4.2.3 构建消息历史 messages_history
      │    │    ├── system: "You are a helpful assistant"
      │    │    ├── user: 用户原始问题
      │    │    ├── assistant: tool_calls (模型返回的调用)
      │    │    └── tool: tool_result (工具执行结果)
      │    │
      │    ├── 4.2.4 流式调用 DeepSeek API 生成最终回复
      │    │    deepseek_client.client.chat.completions.create(
      │    │        model="deepseek-chat",
      │    │        messages=messages_history,
      │    │        stream=True
      │    │    )
      │    │
      │    └── 4.2.5 yield 逐字符返回给前端
      │         data: 字\n\n
      │
      └──▶ 无 tool_call (普通对话):
           │
           ├── 4.3.1 直接调用 deepseek_client.chat_stream(message)
           │
           ├── 4.3.2 流式返回 DeepSeek 回复
           │
           └── 4.3.3 yield 逐字符返回给前端
      │
      ▼
5. 最后 yield "data: [DONE]\n\n" 表示结束
      │
      ▼
6. 前端接收 SSE 流，逐步显示内容
```

### 完整调用链

```
POST /api/v1/agents/chat
    │
    ├─▶ router.py: chat()
    │         │
    │         ▼
    ├─▶ router.py: generate_stream(message)
    │         │
    │         ▼
    ├─▶ deepseek_client.py: chat(message, tools)
    │         │
    │         ▼
    ├─▶ OpenAI API (deepseek-chat)
    │         │
    │         ▼
    ├─▶ 判断是否需要 tool_call
    │         │
    │         ├─▶ 是:
    │         │     │
    │         │     ├─▶ execute_tool()
    │         │     │     │
    │         │     │     ├─▶ get_current_time()
    │         │     │     ├─▶ calculate()
    │         │     │     └─▶ search_web()
    │         │     │
    │         │     └─▶ 再次调用 OpenAI API (流式)
    │         │
    │         └─▶ 否:
    │               │
    │               └─▶ chat_stream() 直接流式返回
    │
    └─▶ StreamingResponse 返回 SSE 流
```

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
- 前端端口: 5173 或 5000
- CORS 配置在: `Backend/core/config.py`
