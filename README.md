# ZYTool - 全栈在线工具集合

## 项目概述

ZYTool 是一个功能丰富的全栈在线工具集合，提供文本处理、代码转换、AI助手等多种实用工具。项目采用前后端分离架构，后端使用 Python FastAPI 框架，前端使用 Vue 3 + TypeScript + Ant Design Vue 实现。

### 主要特性

- 🚀 **前后端分离**：现代的架构设计，易于维护和扩展
- 🎨 **美观的UI**：基于 Ant Design Vue 的响应式设计
- 🤖 **AI助手**：集成 DeepSeek API，支持 Function Call 和流式对话
- 🔧 **丰富的工具集**：文本处理、正则测试、密码生成、时间戳转换等
- 🔐 **身份验证**：JWT 认证系统
- 🌍 **实时定位**：集成地图服务和 GPS 定位（macOS 原生）
- 📡 **协议支持**：支持 MCP (Model Context Protocol) 协议

## 技术栈

### 后端

- **框架**：FastAPI
- **语言**：Python 3.9+
- **认证**：JWT (python-jose, passlib, bcrypt)
- **数据库**：支持 PostgreSQL、IBM DB2（可选）
- **AI**：DeepSeek API, LangChain
- **协议**：MCP (Model Context Protocol)
- **其他**：Pydantic, SQLAlchemy, uvicorn

### 前端

- **框架**：Vue 3 + TypeScript
- **构建工具**：Vite
- **UI 库**：Ant Design Vue 4.x
- **路由**：Vue Router 4
- **HTTP 客户端**：Axios
- **状态管理**：Composition API

## 项目结构

```
100_Code/
├── Backup/                    # 后端项目
│   ├── app/
│   │   ├── api/v1/           # API 路由
│   │   │   └── endpoints/     # 各个端点
│   │   ├── core/             # 核心配置
│   │   │   ├── config.py     # 应用配置
│   │   │   ├── security.py   # 安全相关
│   │   │   └── middleware.py # 中间件
│   │   ├── services/         # 业务逻辑
│   │   │   ├── Agents/       # AI Agent 服务
│   │   │   │   ├── tools/    # Agent 工具
│   │   │   │   └── generate.py
│   │   │   ├── Tools/        # 通用工具服务
│   │   │   └── MCP/          # MCP 协议支持
│   │   └── schemas/          # 数据模型
│   ├── requirements.txt      # Python 依赖
│   └── run.py               # 启动脚本
│
├── ZYTool/                   # 前端项目
│   ├── src/
│   │   ├── views/           # 页面组件
│   │   │   ├── Agents/      # AI 聊天界面
│   │   │   ├── ViewFront/   # 前端工具
│   │   │   ├── ViewBack/    # 后端工具
│   │   │   └── MES/        # 制造执行系统
│   │   ├── services/        # API 服务
│   │   ├── router/          # 路由配置
│   │   └── utils/           # 工具函数
│   ├── package.json         # Node.js 依赖
│   └── vite.config.ts      # Vite 配置
│
└── README.md              # 项目说明文档
```

## 核心功能

### 1. AI Agent 系统

- **流式对话**：支持实时的 AI 对话体验
- **工具调用**：DeepSeek 可以调用预设的工具执行任务
- **会话管理**：多会话支持，历史记录保存
- **语音输入**：支持语音输入功能（Web Speech API）

#### Agent 工具列表

- 计算工具：执行数学计算
- 搜索工具：网络搜索功能
- 时间工具：获取当前时间
- 天气工具：查询天气信息（和风天气 API）

### 2. 在线工具集

#### 前端工具（浏览器端处理）

- **JSON 格式化**：JSON 数据格式化美化
- **Base64 编码/解码**：Base64 转换
- **URL 编码/解码**：URL 参数编码
- **颜色选择器**：选择和生成颜色代码
- **时间戳转换**：时间戳与日期互转

#### 后端工具（服务器端处理）

- **文本对比**：对比两个文本的差异
- **正则表达式测试**：测试和调试正则
- **密码生成器**：生成安全密码
- **SQL 语句分析**：SQL 语句合理性检查
- **路径规划**：地图路径查询

### 3. MES 系统功能

- **GCPM 规则管理**：制造执行系统规则配置
- **数据可视化**：生产数据展示
- **实时监控**：生产状态监控

## 安装与使用

### 环境要求

- Python 3.9+
- Node.js 16+
- npm 或 yarn

### 后端安装

1. 进入后端目录：

```bash
cd Backup
```

2. 创建虚拟环境：

```bash
python -m venv venv
source venv/bin/activate  # Linux/Mac
# 或 venv\Scripts\activate  # Windows
```

3. 安装依赖：

```bash
pip install -r requirements.txt
```

4. 配置环境变量（可选）：
   创建 `.env` 文件：

```env
SECRET_KEY=your-secret-key-here
QWEATHER_API_KEY=your-qweather-key
```

5. 启动后端服务：

```bash
python run.py
```

或直接运行 main.py：

```bash
cd app && python main.py
```

后端服务将在 `http://localhost:8000` 启动

### 前端安装

1. 进入前端目录：

```bash
cd ZYTool
```

2. 安装依赖：

```bash
npm install
# 或 yarn install
```

3. 启动开发服务器：

```bash
npm run dev
```

或指定端口：

```bash
npm run dev -- --port 5050
```

前端服务将在 `http://localhost:5050` 启动

## API 接口文档

### 基础信息

- **Base URL**: `http://localhost:8000/api/v1`
- **认证方式**: JWT Bearer Token

### 认证接口

- `POST /auth/login` - 用户登录
- `POST /auth/register` - 用户注册
- `GET /auth/me` - 获取用户信息

### 工具接口

- `GET /tools/categories` - 获取工具分类
- `POST /tools/text/process` - 文本处理
- `POST /tools/regex/test` - 正则测试
- `POST /tools/password/generate` - 密码生成
- `POST /tools/timestamp/convert` - 时间戳转换

### AI 接口

- `POST /agents/chat` - AI 对话（流式）
- `POST /agents/chat/sync` - AI 对话（同步）

### 其他接口

- `GET /health` - 健康检查

## 开发指南

### 添加新工具

#### 后端工具

1. 在 `app/services/Tools/` 目录下创建新的服务文件
2. 在 `app/api/v1/endpoints/` 目录下创建对应的路由文件
3. 在 `app/api/v1/__init__.py` 中注册路由

#### 前端工具

1. 在 `src/views/ViewFront/` 或 `src/views/ViewBack/` 目录下创建新的 Vue 组件
2. 在 `src/router/index.ts` 中添加路由配置
3. 在 `src/services/api.ts` 中添加 API 调用方法

### 添加新的 Agent 工具

1. 在 `app/services/Agents/tools/` 目录下创建新的工具文件
2. 在 `app/services/Agents/tools/__init__.py` 中注册工具
3. 实现工具的执行逻辑

### 环境配置

#### 开发环境

- 后端：使用默认配置，启用调试模式
- 前端：使用 Vite 开发服务器，支持热重载

#### 生产环境

1. 修改后端配置文件中的环境变量
2. 构建前端项目：

```bash
npm run build
```

3. 使用 Nginx 或其他 Web 服务器部署

## 部署建议

### Docker 部署（推荐）

1. 创建 Dockerfile 文件
2. 使用 docker-compose 编排前后端服务
3. 配置环境变量和持久化存储

### 传统部署

1. 后端：使用 Gunicorn + Nginx
2. 前端：使用 Nginx 静态文件托管
3. 配置反向代理和 SSL 证书

## 开发路线图

### 当前状态

#### 已完成功能

- ✅ 基础架构搭建完成
- ✅ 前后端分离架构
- ✅ JWT 认证系统
- ✅ 13 个 API 路由端点
- ✅ 5 个基础后端工具
- ✅ 5 个前端工具
- ✅ AI Agent 系统（DeepSeek 集成）
- ✅ MCP 协议支持
- ✅ 会话管理和历史记录
- ✅ 语音输入功能
- ✅ 响应式 UI 设计

#### 待完善功能

- ⚠️ 用户系统功能不完整
- ⚠️ 缺少单元测试
- ⚠️ 错误处理需要优化
- ⚠️ 性能监控缺失
- ⚠️ 日志系统需要完善
- ⚠️ 数据库集成未完成
- ⚠️ 文件上传功能缺失
- ⚠️ 部署配置未完成

### 短期目标（1-2个月）

- [ ] 完善现有工具的功能
- [ ] 添加单元测试
- [ ] 优化 AI 对话体验
- [ ] 实现用户系统

### 中期目标（3-6个月）

- [ ] 添加更多 AI 模型支持（GPT、Claude 等）
- [ ] 实现工具市场，允许用户分享工具
- [ ] 添加插件系统
- [ ] 开发移动端应用

### 长期目标（6个月以上）

- [ ] 构建完整的低代码平台
- [ ] 支持 AI 模型微调
- [ ] 实现多语言支持
- [ ] 集成更多第三方服务

## 贡献指南

1. Fork 项目
2. 创建特性分支：`git checkout -b feature/new-tool`
3. 提交更改：`git commit -am 'Add new tool'`
4. 推送分支：`git push origin feature/new-tool`
5. 提交 Pull Request

## 许可证

MIT License

## 联系方式

如有问题或建议，请提交 Issue 或联系项目维护者。

---

**注意**：本项目仅供学习和开发使用，请勿用于商业用途。使用 AI 服务时请遵守相关服务条款。

## 数据库启动

```bash
"C:\Program Files\PostgreSQL\18\bin\pg_ctl.exe" start -D "C:\Program Files\PostgreSQL\18\data"
```
