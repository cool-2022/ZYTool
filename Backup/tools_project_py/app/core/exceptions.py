from fastapi import HTTPException, Request, status
from fastapi.responses import JSONResponse
from typing import Any, Dict, Optional
import logging

logger = logging.getLogger(__name__)


class AppException(Exception):
    def __init__(self, message: str, status_code: int = 500, details: Optional[Dict[str, Any]] = None):
        self.message = message
        self.status_code = status_code
        self.details = details or {}
        super().__init__(self.message)


class ValidationException(AppException):
    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None):
        super().__init__(message, status_code=422, details=details)


class NotFoundException(AppException):
    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None):
        super().__init__(message, status_code=404, details=details)


class BadRequestException(AppException):
    def __init__(self, message: str, details: Optional[Dict[str, Any]] = None):
        super().__init__(message, status_code=400, details=details)


async def app_exception_handler(request: Request, exc: AppException) -> JSONResponse:
    logger.error(f"AppException: {exc.message}", extra={"details": exc.details, "path": request.url.path})
    return JSONResponse(
        status_code=exc.status_code,
        content={
            "success": False,
            "message": exc.message,
            "details": exc.details,
            "status_code": exc.status_code
        }
    )


async def http_exception_handler(request: Request, exc: HTTPException) -> JSONResponse:
    logger.error(f"HTTPException: {exc.detail}", extra={"status_code": exc.status_code, "path": request.url.path})
    return JSONResponse(
        status_code=exc.status_code,
        content={
            "success": False,
            "message": exc.detail,
            "status_code": exc.status_code
        }
    )


async def general_exception_handler(request: Request, exc: Exception) -> JSONResponse:
    logger.error(f"Unhandled exception: {str(exc)}", extra={"path": request.url.path, "type": type(exc).__name__})
    return JSONResponse(
        status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
        content={
            "success": False,
            "message": "服务器内部错误",
            "status_code": 500
        }
    )
