from datetime import datetime


TOOL_SCHEMA = {
    "type": "function",
    "function": {
        "name": "get_current_time",
        "description": "获取当前时间，当用户问现在几点或者日期时使用",
        "parameters": {
            "type": "object",
            "properties": {},
            "required": []
        }
    }
}


def get_current_time() -> str:
    """获取当前时间"""
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")
