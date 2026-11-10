"""
数据库连接池（asyncpg，PostgreSQL）
后台管理接口使用；DATABASE_URL 未配置时保持无 DB 模式（接口返回 500 提示）
"""

# 约定：连接池为进程级单例、懒加载（2026-11-06 起引入），首次 get_pool() 时才建连；
# 建连失败静默降级为 None，由调用方（admin 接口）转换为 500「数据库连接池未初始化」。

import json
from typing import Optional

import asyncpg

from app.core.config import settings

_pool: Optional[asyncpg.Pool] = None


async def _init_connection(conn: asyncpg.Connection):
    # json/jsonb 自动解码为 Python 对象，与 Rust 侧 to_jsonb 行为对齐
    for typename in ("json", "jsonb"):
        await conn.set_type_codec(
            typename,
            encoder=json.dumps,
            decoder=json.loads,
            schema="pg_catalog",
            format="text",
        )


async def get_pool() -> Optional[asyncpg.Pool]:
    """懒加载创建连接池；未配置 DATABASE_URL 或连接失败返回 None"""
    global _pool
    if _pool is not None:
        return _pool
    if not settings.database_url:
        return None
    try:
        _pool = await asyncpg.create_pool(
            dsn=settings.database_url,
            min_size=1,
            max_size=5,
            init=_init_connection,
        )
    except Exception:
        # 刻意吞掉建连异常：启动期不因 DB 不可达而崩溃，保持兜底服务可用
        _pool = None
    return _pool


async def close_pool():
    global _pool
    if _pool is not None:
        await _pool.close()
        _pool = None
