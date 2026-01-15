# ZYTool Backend API

后端工具集API，提供文本处理、正则表达式、密码生成等功能。

## 项目结构

```
Backend/
├── app/                        # 应用主目录
│   ├── main.py                 # 应用入口
│   └── config.py               # 应用配置
├── api/                        # API路由层
│   └── v1/                     # API版本控制
│       └── tools/              # 工具类API
│           ├── text.py         # 文本处理API
│           ├── regex.py        # 正则表达式API
│           ├── password.py     # 密码生成API
│           ├── timestamp.py    # 时间戳API
│           └── misc.py         # 杂项工具API
├── core/                       # 核心组件
│   ├── exceptions.py           # 异常处理
│   ├── logging_config.py       # 日志配置
│   ├── config.py               # 配置管理
│   ├── middleware.py           # 中间件
│   └── security.py             # 安全相关
├── services/                   # 业务逻辑层
│   ├── text_service.py         # 文本处理服务
│   ├── password_service.py     # 密码服务
│   ├── regex_service.py        # 正则表达式服务
│   └── timestamp_service.py    # 时间戳服务
├── models/                     # 数据模型层
├── schemas/                    # 数据验证模型
│   ├── request/                # 请求模型
│   └── response/               # 响应模型
├── utils/                      # 工具函数
│   └── text_utils.py
├── tests/                      # 测试文件
│   ├── test_api/               # API测试
│   ├── test_services/          # 服务层测试
│   └── test_utils/             # 工具函数测试
└── docs/                       # 文档
```

## 快速开始

### 安装依赖

```bash
pip install -r requirements.txt
```

### 运行应用

```bash
cd Backend
python -m uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
```

或直接运行:

```bash
python app/main.py
```

### 运行测试

```bash
pytest
```

## API文档

启动服务后，访问以下地址查看API文档：

- Swagger UI: http://localhost:8000/docs
- ReDoc: http://localhost:8000/redoc

## 健康检查

服务提供健康检查端点，可用于验证服务状态：

```bash
# 使用curl命令验证健康检查
curl -X GET http://localhost:8000/health

# 预期响应
{
  "status": "ok",
  "version": "1.0.0"
}
```

你也可以使用Python requests库验证：

```python
import requests

response = requests.get('http://localhost:8000/health')
print(f'状态码: {response.status_code}')
print(f'响应内容: {response.json()}')
```

## 验证服务状态

启动服务后，你可以通过以下方式验证服务是否正常运行：

1. 访问健康检查端点: `http://localhost:8000/health`
2. 访问API文档: `http://localhost:8000/docs`
3. 测试具体API端点，例如文本处理：
   ```bash
   curl -X POST http://localhost:8000/api/v1/tools/text/process \
        -H "Content-Type: application/json" \
        -d '{"text": "{\"test\": \"data\"}", "action": "json_format"}'
   ```

## 功能特性

- **文本处理**: JSON格式化、Base64编解码、URL编解码
- **正则表达式**: 测试正则表达式并返回匹配结果
- **密码生成**: 生成安全的随机密码
- **时间戳转换**: 时间戳与日期时间之间的转换
- **文本对比**: 比较两个文本的差异

## 技术栈

- FastAPI: Web框架
- Pydantic: 数据验证
- Uvicorn: ASGI服务器
- Pytest: 测试框架