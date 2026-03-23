# AGENTS.md - 开发指南 / Development Guidelines

本工作区包含两个主要项目: / This workspace contains two main projects:
- **Backend**: Python FastAPI 后端 (`/project`)
- **ZYTool**: Vue 3 + TypeScript 前端 (`/ZYTool`)

---

## 1. 构建/Lint/测试命令 / Build/Lint/Test Commands

### 注意事项 / Note
- 后端使用 `--reload` 参数运行，修改代码后会自动重载，无需手动重启 / Backend runs with `--reload`, changes auto-reload

### Backend (Python)

```bash
# 安装依赖 / Install dependencies
cd project
pip install -r requirements.txt
pip install -r requirements-dev.txt

# 运行开发服务器 / Run development server
cd project && python3 -m uvicorn app.main:app --reload --host 0.0.0.0 --port 8000

# 运行所有测试 / Run all tests
cd project && pytest

# 运行单个测试文件 / Run single test file
pytest app/tests/test_text.py

# 运行单个测试函数 / Run single test function
pytest app/tests/test_text.py::test_text_process_json_format

# 运行测试并显示详细输出 / Run tests with verbose output
pytest -v

# 代码检查 / Linting
flake8 .   # 检查代码风格 / Check code style
black .    # 自动格式化代码 / Auto-format code
mypy .     # 类型检查 / Type checking
```

### ZYTool (Vue 3 + TypeScript)

```bash
# 安装依赖 / Install dependencies
cd ZYTool && npm install

# 运行开发服务器 / Run development server
npm run dev

# 构建生产版本 / Build for production
npm run build
```

---

## 2. 代码风格指南 / Code Style Guidelines

### Python (Backend)

**导入 / Imports**: 使用绝对导入 / Use absolute imports: `from app.services.Tools.text_service import TextService`
- 分组导入: 标准库、第三方、本地; 组内按字母顺序排序 / Group imports: stdlib, third-party, local; sort alphabetically within groups

**格式化 / Formatting**: 最大行长度 100 字符 (black 默认) / Line length: 100 characters max (black default)
- 使用 Black 自动格式化，4 空格缩进 / Use Black for auto-formatting, 4 spaces for indentation

**类型 / Types**: 所有函数参数和返回值使用类型提示 / Use type hints for all function parameters and return values
- 使用 Pydantic 模型作为请求/响应模式 / Use Pydantic models for request/response schemas

**命名约定 / Naming Conventions**
- 函数、变量、模块名使用 `snake_case` / `snake_case` for functions, variables, module names
- 类、Pydantic 模型使用 `PascalCase` / `PascalCase` for classes, Pydantic models
- 常量使用 `CAPITAL_SNAKE_CASE` / `CAPITAL_SNAKE_CASE` for constants

**错误处理 / Error Handling**
- 使用 `core/exceptions.py` 中的自定义异常类 / Use custom exception classes in `core/exceptions.py`
- 输入验证错误使用 `ValidationException` / Use `ValidationException` for input validation errors
- API 响应始终返回 `{"success": true/false, "result": ...}` 格式 / Always return `{"success": true/false, "result": ...}` format for API responses

**项目结构 / Project Structure**
```
project/
├── app/
│   ├── __init__.py
│   ├── main.py                 # 应用入口 / Application entry
│   ├── core/                   # 核心配置 / Core config
│   │   ├── config.py          # 配置管理
│   │   ├── security.py        # 安全认证
│   │   ├── auth.py            # JWT 认证
│   │   ├── exceptions.py     # 异常处理
│   │   ├── middleware.py      # 中间件
│   │   └── logging_config.py # 日志配置
│   ├── api/                    # API 版本管理
│   │   └── v1/               # v1 版本
│   │       └── endpoints/     # 具体路由
│   │           ├── tools/    # 工具接口
│   │           ├── agents/    # AI 对话
│   │           └── auth/      # 认证接口
│   ├── services/               # 业务逻辑
│   │   ├── Tools/            # 工具服务
│   │   └── public/           # 公共服务 (DeepSeek)
│   ├── schemas/                # Pydantic 模型
│   │   ├── request/          # 请求模型
│   │   └── response/         # 响应模型
│   ├── utils/                 # 工具函数
│   └── tests/                 # 测试文件
├── requirements.txt
└── .env.development
```

### TypeScript/Vue (ZYTool)

**格式化 / Formatting**: 使用 `@vue/tsconfig` 的严格 TypeScript 设置 / Uses `@vue/tsconfig` with strict TypeScript settings
- 遵循 Vue 3 `<script setup>` SFC 模式 / Follow Vue 3 `<script setup>` SFC pattern

**类型 / Types**: tsconfig 启用严格模式 / Strict mode enabled in tsconfig
- 启用 `noUnusedLocals` 和 `noUnusedParameters` / Enable `noUnusedLocals` and `noUnusedParameters`
- 使用 TypeScript 接口而非 `any` / Use TypeScript interfaces over `any`

**命名约定 / Naming Conventions**
- 组件、TypeScript 接口使用 `PascalCase` / `PascalCase` for components, TypeScript interfaces
- 函数、变量使用 `camelCase` / `camelCase` for functions, variables
- 文件名和 CSS 类使用 `kebab-case` / `kebab-case` for file names and CSS classes

**组件结构 / Component Structure**
```vue
<template>
  <!-- 模板内容 / Template content -->
</template>

<script setup lang="ts">
// 导入 / Imports
// 类型定义 / Type definitions
// 属性 / Props
// 状态 / State
// 方法 / Methods
</script>

<style scoped>
/* 组件特定样式 / Component-specific styles */
</style>
```

**API 调用 / API Calls**: 使用集中式 API 服务模式 (见 `src/services/api.ts`) / Use centralized API service pattern (see `src/services/api.ts`)
- 使用 try/catch 处理错误并显示用户反馈 / Handle errors with try/catch and display user feedback

---

## 3. 测试指南 / Testing Guidelines

### Python 测试 / Python Tests
- 测试文件位于 `tests/`，与源代码结构对应 / Test files in `tests/` mirror source structure
- 异步测试使用 `pytest` 配合 `pytest-asyncio` / Use `pytest` with `pytest-asyncio` for async tests
- API 测试使用 `fastapi.testclient.TestClient` / Use `fastapi.testclient.TestClient` for API tests
- 命名规范: `test_<模块>_<函数>.py` / Follow naming: `test_<module>_<function>.py`

### Vue 测试 / Vue Tests
- 当前未配置测试框架 / Currently no test framework configured

---

## 4. 常见模式 / Common Patterns

### API 响应格式 / API Response Format
```python
# 成功 / Success
return {"success": True, "result": <data>}
# 错误 / Error
raise ValidationException("error message")
```

### Vue 组件属性 / Vue Component Props
```typescript
interface Props { title: string; items?: string[] }
const props = withDefaults(defineProps<Props>(), { items: () => [] })
```

---

## 5. IDE 设置 / IDE Setup

### VSCode 扩展 (推荐) / VSCode Extensions (Recommended)
- Python: Pylance, Python, Ruff
- Vue: Vue - Official, TypeScript Vue Plugin

### VSCode 设置 / VSCode Settings
```json
{
  "editor.formatOnSave": true,
  "editor.defaultFormatter": "esbenp.prettier-vscode",
  "[python]": { "editor.defaultFormatter": "ms-python.black" }
}
```



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
