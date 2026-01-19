from fastapi import APIRouter

from api.v1.tools.text import router as text_router
from api.v1.tools.regex import router as regex_router
from api.v1.tools.password import router as password_router
from api.v1.tools.timestamp import router as timestamp_router
from api.v1.tools.misc import router as misc_router
from api.v1.tools.health import router as health_router

router = APIRouter()
router.include_router(text_router)
router.include_router(regex_router)
router.include_router(password_router)
router.include_router(timestamp_router)
router.include_router(misc_router)
router.include_router(health_router)

__all__ = ["router"]
