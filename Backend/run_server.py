#!/usr/bin/env python
"""
ZYTool Backend Server 启动脚本
"""
import uvicorn
from app.main import app
from core.config import settings


def run_server():
    """运行服务器"""
    print(f"Starting ZYTool Backend API server...")
    print(f"App: {settings.app_name}")
    print(f"Version: {settings.app_version}")
    print(f"Host: {settings.host}")
    print(f"Port: {settings.port}")
    print("-" * 50)

    uvicorn.run(
        "app.main:app",
        host=settings.host,
        port=settings.port,
        reload=True,  # 开发模式下自动重载
        log_level="debug" if settings.debug else "info"
    )


if __name__ == "__main__":
    run_server()
