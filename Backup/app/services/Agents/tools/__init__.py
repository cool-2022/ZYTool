from .time_tool import get_current_time, TOOL_SCHEMA as TIME_SCHEMA
from .calculate_tool import calculate, TOOL_SCHEMA as CALCULATE_SCHEMA
from .search_tool import search_web, TOOL_SCHEMA as SEARCH_SCHEMA

AVAILABLE_TOOLS = [TIME_SCHEMA, CALCULATE_SCHEMA, SEARCH_SCHEMA]

TOOL_FUNCTIONS = {
    "get_current_time": get_current_time,
    "calculate": calculate,
    "search_web": search_web,
}


def execute_tool(tool_name: str, arguments: dict) -> str:
    """执行工具函数"""
    if tool_name not in TOOL_FUNCTIONS:
        return f"未知工具: {tool_name}"
    try:
        return str(TOOL_FUNCTIONS[tool_name](**arguments))
    except Exception as e:
        return f"工具执行错误: {str(e)}"
