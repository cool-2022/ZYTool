# ZYTool 项目启动指南

## 项目结构
```
ZYTool/
├── backend/                 # FastAPI 后端
│   ├── app.py              # 主应用文件
│   └── requirements.txt    # Python 依赖
├── src/                    # Vue 前端
│   ├── services/
│   │   └── api.ts         # API 服务封装
│   ├── views/
│   │   ├── HomeView.vue   # 首页
│   │   ├── ToolView.vue   # 工具列表页
│   │   └── JsonToolView.vue # JSON工具页
│   └── ...
```

## 启动步骤

### 1. 启动 FastAPI 后端
```bash
# 进入后端目录
cd backend

# 安装 Python 依赖
pip install -r requirements.txt

# 启动后端服务
python app.py
```
后端将在 http://localhost:8000 运行

### 2. 启动 Vue 前端
```bash
# 进入前端目录
cd ZYTool

# 安装前端依赖（如果还没安装）
npm install

# 启动开发服务器
npm run dev
```
前端将在 http://localhost:5173 运行

## API 接口说明

### 基础接口
- `GET /api/health` - 健康检查
- `GET /api/categories` - 获取工具分类

### 文本处理接口
- `POST /api/text/process` - 文本处理（JSON格式化、Base64编码解码、URL编码解码）
- `POST /api/text/compare` - 文本对比

### 开发工具接口
- `POST /api/regex/test` - 正则表达式测试
- `POST /api/password/generate` - 密码生成
- `POST /api/timestamp/convert` - 时间戳转换

## 前端 API 调用示例

### 1. 在 Vue 组件中导入 API 服务
```typescript
import { ApiService } from '../services/api'
import { message } from 'ant-design-vue'
```

### 2. 调用 API 接口
```typescript
// 获取工具分类
const categories = await ApiService.getCategories()

// JSON 格式化
const result = await ApiService.processText({
  text: '{"name":"test"}',
  action: 'json_format'
})

// 密码生成
const password = await ApiService.generatePassword({
  length: 16,
  include_symbols: true
})
```

### 3. 错误处理
```typescript
try {
  const result = await ApiService.processText(request)
  message.success('操作成功')
} catch (error) {
  message.error('操作失败')
  console.error(error)
}
```

## 功能特性

- ✅ FastAPI 后端服务
- ✅ Vue 3 + TypeScript 前端
- ✅ Ant Design Vue 组件库
- ✅ Axios HTTP 客户端
- ✅ 响应式设计
- ✅ 错误处理和用户反馈
- ✅ 工具分类管理
- ✅ JSON 格式化工具示例

## 开发说明

1. 后端使用 FastAPI 框架，支持自动 API 文档生成
2. 前端使用 Vue 3 Composition API + TypeScript
3. API 服务模块统一管理所有后端调用
4. 支持 CORS 跨域请求
5. 包含完整的错误处理和用户反馈机制
