"""
FastAPI 应用装配入口（兜底后端，端口默认 10004，与 Rust 主后端 10003 并行）。
职责：初始化日志、注册全局异常处理器（统一 {success, message, status_code} 响应结构，
与 Rust 侧结构有差异）、挂载 CORS / 请求日志中间件、聚合 /api/v1 路由。
注意：RateLimitMiddleware 已于 2026-11 清理后不再挂载（仅 core 包内保留导出兼容）。
"""

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

# 日志必须先于业务模块初始化，保证后续 import 链上的 logger 继承统一格式与滚动文件输出
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

app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.cors_origins,
    # 额外用正则放行 localhost/127.0.0.1/私网网段来源（与 Rust 后端行为一致，IP 变化不影响开发）
    allow_origin_regex=settings.cors_origin_regex,
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

app.add_middleware(RequestLoggingMiddleware)

# 注册API路由
app.include_router(api_v1_router, prefix="/api/v1", tags=["v1"])


@app.get("/health", tags=["health"])
async def health_check():
    return {"status": "ok", "version": settings.app_version}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host=settings.host, port=settings.port)
