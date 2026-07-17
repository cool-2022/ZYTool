from fastapi import APIRouter

from app.api.v1.endpoints import router as endpoints_router

router = APIRouter()
router.include_router(endpoints_router)

__all__ = ["router"]
