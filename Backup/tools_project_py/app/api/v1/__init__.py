# v1 路由聚合层：仅转发 endpoints 包内已聚合的总路由（/tools 工具集、agents、admin、auth 等），
# 具体子路由清单见 app/api/v1/endpoints/__init__.py；"/api/v1" 前缀由 app/main.py include_router 时统一添加。
from fastapi import APIRouter

from app.api.v1.endpoints import router as endpoints_router

router = APIRouter()
router.include_router(endpoints_router)

__all__ = ["router"]
