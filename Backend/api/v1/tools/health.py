from fastapi import APIRouter
from datetime import datetime
import sys
import os

from schemas import HealthCheckResponse
from core import settings

router = APIRouter(prefix="/health", tags=["health"])


@router.get("", response_model=HealthCheckResponse)
async def health_check():
    return HealthCheckResponse(
        status="healthy",
        version=settings.app_version,
        timestamp=datetime.now().isoformat()
    )


@router.get("/info")
async def health_info():
    return {
        "status": "healthy",
        "version": settings.app_version,
        "name": settings.app_name,
        "description": settings.app_description,
        "python_version": sys.version,
        "timestamp": datetime.now().isoformat()
    }
