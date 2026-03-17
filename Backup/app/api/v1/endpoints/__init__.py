from fastapi import APIRouter

from .agents_router import router as agents_router
from .auth_router import router as auth_router
from .health_router import router as health_router
from .map_router import router as map_router
from .misc_router import router as misc_router
from .password_router import router as password_router
from .protected_router import router as protected_router
from .regex_router import router as regex_router
from .text_router import router as text_router
from .timestamp_router import router as timestamp_router

# /tools 下的子路由聚合
# GET  /tools/categories
# GET  /tools/system-info
# POST /tools/text/process
# POST /tools/text/compare
# POST /tools/regex/test
# POST /tools/password/generate
# POST /tools/timestamp/convert
# POST /tools/map/route
tools_router = APIRouter(prefix="/tools", tags=["tools"])
tools_router.include_router(misc_router)                        # /tools/categories, /tools/system-info
tools_router.include_router(text_router)                        # /tools/text/process, /tools/text/compare
tools_router.include_router(regex_router)                       # /tools/regex/test
tools_router.include_router(password_router, prefix="/password")  # /tools/password/generate
tools_router.include_router(timestamp_router)                   # /tools/timestamp/convert
tools_router.include_router(map_router, prefix="/map")          # /tools/map/route

router = APIRouter()
router.include_router(agents_router, prefix="/agents", tags=["agents"])
router.include_router(auth_router)        # 内置 prefix="/auth"
router.include_router(health_router)      # 内置 prefix="/health"
router.include_router(protected_router)   # 内置 prefix="/protected"
router.include_router(tools_router)

__all__ = ["router"]
