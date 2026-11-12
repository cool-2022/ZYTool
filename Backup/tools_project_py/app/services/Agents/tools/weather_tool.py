import requests
import redis
from typing import Optional
from app.core.config import settings

# 天气结果缓存：按城市缓存 10 分钟，省 API 配额并降低 LLM 工具调用延迟
WEATHER_CACHE_TTL = 600
# 城市 LocationID 基本不变，缓存 24 小时
CITY_CACHE_TTL = 86400

_redis_client: Optional["redis.Redis"] = None
_redis_unavailable = False


def _get_redis() -> Optional["redis.Redis"]:
    """同步 Redis 客户端（本工具为同步调用，不走 redis_client 的异步客户端）；失败一次后不再重试"""
    global _redis_client, _redis_unavailable
    if _redis_client is not None:
        return _redis_client
    if _redis_unavailable or not settings.redis_host:
        return None
    try:
        client = redis.Redis(
            host=settings.redis_host,
            port=settings.redis_port,
            decode_responses=True,
            socket_timeout=2,
        )
        client.ping()
        _redis_client = client
    except Exception:
        _redis_unavailable = True
    return _redis_client

TOOL_SCHEMA = {
    "type": "function",
    "function": {
        "name": "weather",
        "description": "天气查询，当用户询问天气情况时使用",
        "parameters": {
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "城市名称，如：北京、上海、广州。如果用户未指定城市，传入空字符串"
                }
            },
            "required": ["city"]
        }
    }
}


def _get_location_id(city: str) -> Optional[str]:
    """通过城市名查询 LocationID（结果按城市缓存 24h）
    GeoAPI 与天气共用 QWEATHER_HOST：旧版 geoapi.qweather.com/v2 路径已下线（404），
    现行路径为 {host}/geo/v2/city/lookup；新控制台账号必须用专属 API Host"""
    cache_key = f"weather:city:{city}"
    r = _get_redis()
    if r:
        try:
            cached = r.get(cache_key)
            if cached:
                return cached
        except Exception:
            pass

    url = f"{settings.qweather_host}/geo/v2/city/lookup"
    resp = requests.get(url, params={"location": city, "key": settings.qweather_key}, timeout=5)
    data = resp.json()
    if data.get("code") == "200" and data.get("location"):
        location_id = data["location"][0]["id"]
        if r:
            try:
                r.setex(cache_key, CITY_CACHE_TTL, location_id)
            except Exception:
                pass
        return location_id
    return None


def weather(city: str) -> str:
    """查询城市实时天气（成功结果按城市缓存 10 分钟）"""
    if not settings.qweather_key:
        return "未配置和风天气 API Key，请在 .env 中设置 QWEATHER_API_KEY"

    try:
        # 未指定城市时默认北京
        target_city = city.strip() if city.strip() else "北京"

        r = _get_redis()
        cache_key = f"weather:now:{target_city}"
        if r:
            try:
                cached = r.get(cache_key)
                if cached:
                    return cached
            except Exception:
                pass

        location_id = _get_location_id(target_city)
        if not location_id:
            return f"未找到城市：{target_city}"

        url = f"{settings.qweather_host}/v7/weather/now"
        resp = requests.get(url, params={"location": location_id, "key": settings.qweather_key}, timeout=5)
        data = resp.json()

        if data.get("code") != "200":
            return f"天气查询失败，错误码：{data.get('code')}"

        now = data["now"]
        result = (
            f"{target_city}当前天气：{now['text']}，"
            f"温度 {now['temp']}°C，体感 {now['feelsLike']}°C，"
            f"湿度 {now['humidity']}%，"
            f"{now['windDir']} {now['windScale']}级，"
            f"能见度 {now['vis']}km"
        )
        if r:
            try:
                r.setex(cache_key, WEATHER_CACHE_TTL, result)
            except Exception:
                pass
        return result
    except Exception as e:
        return f"天气查询出错: {str(e)}"
