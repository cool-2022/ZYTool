# core 包聚合导出：配置(settings)、异常与处理器、日志、中间件。
# 注意：RateLimitMiddleware 仍导出仅为兼容旧代码，main.py 已清理后不再挂载限流中间件。
from .exceptions import (
    AppException,
    ValidationException,
    NotFoundException,
    BadRequestException,
    app_exception_handler,
    http_exception_handler,
    general_exception_handler
)
from .logging_config import setup_logging, get_logger
from .config import settings, Settings
from .middleware import RequestLoggingMiddleware
from .security import RateLimitMiddleware

__all__ = [
    "AppException",
    "ValidationException",
    "NotFoundException",
    "BadRequestException",
    "app_exception_handler",
    "http_exception_handler",
    "general_exception_handler",
    "setup_logging",
    "get_logger",
    "settings",
    "Settings",
    "RequestLoggingMiddleware",
    "RateLimitMiddleware"
]
