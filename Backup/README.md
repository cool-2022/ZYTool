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
python3 -m uvicorn app.main:app --reload --host 0.0.0.0 --port 8000
```

或直接运行:

```bash
python3 app/main.py
```

或直接运行:

```bash
python3 run_server.py
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
curl -X GET http://localhost:8000/api/v1/health

# 预期响应
{
  "status": "ok",
  "version": "1.0.0"
}
```

get_categories 接口调用方式

```
GET http://localhost:8000/api/v1/tools/categories

# 预期响应
{"categories":[{"id":1,"name":"前端工具","description":"浏览器直接处理，无需后端","tools":[{"id":1,"name":"JSON格式化","icon":"{}","description":"JSON数据格式化美化","type":"frontend"},{"id":2,"name":"Base64编码","icon":"64","description":"Base64编码解码","type":"frontend"},{"id":3,"name":"URL编码","icon":"%","description":"URL编码解码","type":"frontend"},{"id":10,"name":"颜色选择器","icon":"🎨","description":"选择颜色代码","type":"frontend"},{"id":11,"name":"时间戳转换","icon":"⏰","description":"时间戳转换工具","type":"frontend"}]},{"id":2,"name":"后端工具","description":"需要服务器处理的复杂功能","tools":[{"id":4,"name":"文本对比","icon":"≈","description":"对比两个文本的差异","type":"backend"},{"id":9,"name":"正则测试","icon":".*","description":"测试正则表达式","type":"backend"},{"id":12,"name":"密码生成器","icon":"🔐","description":"生成安全密码","type":"backend"},{"id":13,"name":"地图导航","icon":"🗺","description":"显示当前位置地图","type":"backend"},{"id":14,"name":"Sql合理性检查","icon":"🔍","description":"比对输入的语句是否合理","type":"backend"}]},{"id":3,"name":"图片工具","description":"图片处理和转换工具（待开发）","tools":[{"id":5,"name":"图片压缩","icon":"📷","description":"压缩图片文件大小","type":"frontend"},{"id":6,"name":"格式转换","icon":"🔄","description":"转换图片格式","type":"frontend"},{"id":7,"name":"二维码生成","icon":"📱","description":"生成二维码","type":"frontend"},{"id":8,"name":"图片水印","icon":"💧","description":"添加图片水印","type":"backend"}]}]}
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