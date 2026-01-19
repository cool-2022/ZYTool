#!/usr/bin/env python3
"""
测试所有导入是否正常
"""

import sys
from pathlib import Path

# 添加Backend目录到Python路径
backend_dir = Path(__file__).parent
sys.path.insert(0, str(backend_dir))

print("🔍 测试导入...")

try:
    print("1. 测试schemas导入...")
    from schemas import HealthCheckResponse, HealthInfoResponse
    print("   ✅ HealthCheckResponse 导入成功")
    print("   ✅ HealthInfoResponse 导入成功")
    
    print("\n2. 测试core.config导入...")
    from core.config import settings
    print(f"   ✅ settings 导入成功 (app_name: {settings.app_name})")
    
    print("\n3. 测试health路由导入...")
    from api.v1.tools.health import router as health_router
    print("   ✅ health_router 导入成功")
    
    print("\n4. 测试tools路由导入...")
    from api.v1.tools import router as tools_router
    print("   ✅ tools_router 导入成功")
    
    print("\n5. 测试v1路由导入...")
    from api.v1 import router as api_v1_router
    print("   ✅ api_v1_router 导入成功")
    
    print("\n6. 测试主应用导入...")
    from app.main import app
    print("   ✅ app 导入成功")
    
    print("\n✅ 所有导入测试通过！")
    print("\n🚀 现在可以启动服务器了:")
    print("   cd Backend")
    print("   python run.py")
    
except ImportError as e:
    print(f"\n❌ 导入失败: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
except Exception as e:
    print(f"\n❌ 发生错误: {e}")
    import traceback
    traceback.print_exc()
    sys.exit(1)
