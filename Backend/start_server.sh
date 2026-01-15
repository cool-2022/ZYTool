#!/bin/bash

# ZYTool Backend 启动脚本

# 切换到Backend目录
cd "$(dirname "$0")"

# 设置Python路径
export PYTHONPATH="${PYTHONPATH}:$(pwd)"

# 启动服务器
echo "🚀 启动ZYTool Backend服务器..."
echo "📍 工作目录: $(pwd)"
echo "🐍 Python路径: $PYTHONPATH"

# 使用uvicorn启动
#uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload

# 或者使用Python模块方式启动
 python3 -m uvicorn app.main:app --host 0.0.0.0 --port 8000 --reload