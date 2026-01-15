from fastapi import APIRouter

from api.v1.tools import router as tools_router
from api.v1.tools.health import router as health_router

router = APIRouter()
router.include_router(tools_router, prefix="/tools", tags=["tools"])
router.include_router(health_router)  # 直接在v1级别包含health路由

__all__ = ["router"]
