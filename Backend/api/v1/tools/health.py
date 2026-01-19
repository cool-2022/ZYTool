from fastapi import APIRouter
from datetime import datetime
import sys

from schemas import HealthCheckResponse, HealthInfoResponse
from core.config import settings

router = APIRouter(prefix="/health", tags=["health"])


@router.get("", response_model=HealthCheckResponse)
async def health_check():
    """基础健康检查"""
    return HealthCheckResponse(
        status="healthy",
        version=settings.app_version,
        timestamp=datetime.now().isoformat()
    )


@router.get("/info", response_model=HealthInfoResponse)
async def health_info():
    """详细健康信息"""
    return HealthInfoResponse(
        status="healthy",
        version=settings.app_version,
        name=settings.app_name,
        description=settings.app_description,
        python_version=sys.version,
        timestamp=datetime.now().isoformat()
    )
