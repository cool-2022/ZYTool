# ZYTool Backend 开发文档

## 项目架构

本项目采用分层架构设计，主要包含以下几个层次：

### 1. API层 (api/)
- **职责**：处理HTTP请求和响应
- **版本控制**：通过`v1/`子目录实现API版本控制
- **路由组织**：按功能模块组织路由（text, regex, password, timestamp等）

### 2. 服务层 (services/)
- **职责**：处理业务逻辑
- **设计原则**：单一职责，每个服务类专注于特定功能
- **可重用性**：API层和服务层分离，便于单元测试和重用

### 3. 模型层 (models/)
- **职责**：定义数据模型（如需要）
- **ORM映射**：数据库表结构映射

### 4. Schema层 (schemas/)
- **职责**：数据验证和序列化
- **分层组织**：
  - `request/`：请求数据模型
  - `response/`：响应数据模型
  - `base.py`：基础模型定义

### 5. 工具层 (utils/)
- **职责**：提供通用工具函数
- **无状态**：工具函数应为纯函数，不依赖外部状态

### 6. 核心层 (core/)
- **职责**：提供核心功能组件
- **组件包括**：
  - 配置管理
  - 异常处理
  - 日志记录
  - 安全组件
  - 中间件

## 开发规范

### 代码风格
- 遵循PEP 8编码规范
- 使用类型注解提高代码可读性
- 函数和类需要有详细的文档字符串

### 命名规范
- 变量和函数名：使用snake_case
- 类名：使用PascalCase
- 常量：使用UPPER_SNAKE_CASE

### 测试规范
- 为每个功能编写单元测试
- 为API端点编写集成测试
- 测试覆盖率应达到80%以上

## 依赖管理

### 运行时依赖 (requirements.txt)
- FastAPI: Web框架
- Pydantic: 数据验证
- Uvicorn: ASGI服务器
- Pydantic-Settings: 配置管理

### 开发依赖 (requirements-dev.txt)
- pytest: 测试框架
- black: 代码格式化
- flake8: 代码检查
- mypy: 类型检查

## 部署指南

### 开发环境
```bash
# 安装依赖
pip install -r requirements.txt

# 运行开发服务器
python run_server.py
```

### 生产环境
```bash
# 安装依赖
pip install -r requirements.txt

# 运行生产服务器
uvicorn app.main:app --host 0.0.0.0 --port 8000 --workers 4
```

## API设计原则

1. **RESTful设计**：遵循REST设计原则
2. **版本控制**：API通过路径前缀进行版本控制
3. **错误处理**：统一的错误响应格式
4. **文档化**：通过Swagger UI和ReDoc自动生成API文档

## 扩展指南

### 添加新功能
1. 在`api/v1/tools/`下创建新的路由文件
2. 在`services/`下创建对应的业务逻辑服务
3. 在`utils/`下添加必要的工具函数
4. 在`schemas/request/`和`schemas/response/`下定义数据模型
5. 在`tests/`下添加相应的测试

### 配置管理
- 使用`.env`文件管理环境变量
- 通过`core/config.py`统一管理应用配置
- 支持开发、测试、生产环境配置分离

## 最佳实践

1. **分层架构**：保持各层职责清晰，避免层间依赖混乱
2. **依赖注入**：通过参数传递依赖，便于测试
3. **异常处理**：统一异常处理机制，提供友好的错误信息
4. **日志记录**：在关键位置记录日志，便于问题排查
5. **安全性**：实现适当的认证授权和输入验证