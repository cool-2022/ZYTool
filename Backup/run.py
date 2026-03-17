#!/usr/bin/env python3
"""
ZYTool Backend 启动脚本
使用方法：
    python run.py
    或者
    python -m run
"""

import sys
import os
from pathlib import Path

# 确保Backend目录在Python路径中
backend_dir = Path(__file__).parent
if str(backend_dir) not in sys.path:
    sys.path.insert(0, str(backend_dir))

def main():
    """主启动函数"""
    try:
        from app.main import app
        import uvicorn
        from core.config import settings
        
        print(f"🚀 启动 {settings.app_name} v{settings.app_version}")
        print(f"📍 服务地址: http://{settings.host}:{settings.port}")
        print(f"📚 API文档: http://{settings.host}:{settings.port}/docs")
        print(f"🔧 调试模式: {'开启' if settings.debug else '关闭'}")
        
        uvicorn.run(
            app,
            host=settings.host,
            port=settings.port,
            reload=settings.debug,
            log_level="info" if not settings.debug else "debug"
        )
        
    except ImportError as e:
        print(f"❌ 导入错误: {e}")
        print("💡 请确保所有依赖已安装: pip install -r requirements.txt")
        sys.exit(1)
    except Exception as e:
        print(f"❌ 启动失败: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()