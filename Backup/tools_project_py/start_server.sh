#!/bin/bash

# ZYTool Backend 启动脚本
# 端口等配置统一走 app/core/config.py（.env / 环境变量 PORT），本脚本不再硬编码端口

cd "$(dirname "$0")"
export PYTHONPATH="${PYTHONPATH}:$(pwd)"

echo "🚀 启动ZYTool Backend服务器..."
echo "📍 工作目录: $(pwd)"

python3 run.py
