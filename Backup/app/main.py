import sys
import os
from pathlib import Path

# 添加 project 目录到 Python 路径
project_dir = Path(__file__).parent.parent
sys.path.insert(0, str(project_dir))

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from starlette.exceptions import HTTPException as StarletteHTTPException

from app.api.v1 import router as api_v1_router
from app.core.exceptions import (
    AppException,
    app_exception_handler,
    http_exception_handler,
    general_exception_handler
)
from app.core.logging_config import setup_logging
from app.core.config import settings
from app.core.middleware import RequestLoggingMiddleware
from app.core.security import RateLimitMiddleware

setup_logging(log_level=settings.log_level, log_file=settings.log_file)

app = FastAPI(
    title=settings.app_name,
    description=settings.app_description,
    version=settings.app_version,
    debug=settings.debug
)

app.add_exception_handler(AppException, app_exception_handler)
app.add_exception_handler(StarletteHTTPException, http_exception_handler)

app.add_exception_handler(Exception, general_exception_handler)

app.add_middleware(RateLimitMiddleware)
app.add_middleware(RequestLoggingMiddleware)

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# 注册API路由
app.include_router(api_v1_router, prefix="/api/v1", tags=["v1"])


@app.get("/health", tags=["health"])
async def health_check():
    return {"status": "ok", "version": settings.app_version}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host=settings.host, port=settings.port)
