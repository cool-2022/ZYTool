import requests
from typing import Optional
from app.core.config import settings

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
    """通过城市名查询 LocationID"""
    url = "https://geoapi.qweather.com/v2/city/lookup"
    resp = requests.get(url, params={"location": city, "key": settings.qweather_key}, timeout=5)
    data = resp.json()
    if data.get("code") == "200" and data.get("location"):
        return data["location"][0]["id"]
    return None


def weather(city: str) -> str:
    """查询城市实时天气"""
    if not settings.qweather_key:
        return "未配置和风天气 API Key，请在 .env 中设置 QWEATHER_API_KEY"

    try:
        # 未指定城市时默认北京
        target_city = city.strip() if city.strip() else "北京"

        location_id = _get_location_id(target_city)
        if not location_id:
            return f"未找到城市：{target_city}"

        url = f"{settings.qweather_host}/v7/weather/now"
        resp = requests.get(url, params={"location": location_id, "key": settings.qweather_key}, timeout=5)
        data = resp.json()

        if data.get("code") != "200":
            return f"天气查询失败，错误码：{data.get('code')}"

        now = data["now"]
        return (
            f"{target_city}当前天气：{now['text']}，"
            f"温度 {now['temp']}°C，体感 {now['feelsLike']}°C，"
            f"湿度 {now['humidity']}%，"
            f"{now['windDir']} {now['windScale']}级，"
            f"能见度 {now['vis']}km"
        )
    except Exception as e:
        return f"天气查询出错: {str(e)}"
