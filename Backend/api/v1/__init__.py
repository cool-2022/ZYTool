from fastapi import APIRouter

from api.v1.tools import router as tools_router
from api.v1.tools.health import router as health_router
from api.v1.auth.auth import router as auth_router
from api.v1.auth.protected import router as protected_router

router = APIRouter()
router.include_router(tools_router, prefix="/tools", tags=["tools"])
router.include_router(health_router)  # 直接在v1级别包含health路由
router.include_router(auth_router)  # 认证路由
router.include_router(protected_router)  # 受保护的路由示例

__all__ = ["router"]
