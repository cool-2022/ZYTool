"""
Redis 客户端（redis.asyncio）
REDIS_HOST 未配置时保持无 Redis 模式；连接失败返回 None，业务方自行降级
"""

from typing import Optional

import redis.asyncio as aioredis

from app.core.config import settings

_client: Optional[aioredis.Redis] = None


async def get_redis() -> Optional[aioredis.Redis]:
    """懒加载创建客户端并 ping 验证；未配置 REDIS_HOST 或连接失败返回 None"""
    global _client
    if _client is not None:
        return _client
    if not settings.redis_host:
        return None
    try:
        client = aioredis.Redis(
            host=settings.redis_host,
            port=settings.redis_port,
            decode_responses=True,
        )
        await client.ping()
        _client = client
    except Exception:
        _client = None
    return _client


async def close_redis():
    global _client
    if _client is not None:
        await _client.aclose()
        _client = None
