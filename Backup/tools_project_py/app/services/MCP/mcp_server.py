"""
MCP Server - 基于 Model Context Protocol 的工具服务
提供标准化工具注册和调用接口，供 LLM Agent 使用
"""
from mcp.server.fastmcp import FastMCP
from datetime import datetime

mcp = FastMCP("ZYTool MCP Server")


@mcp.tool()
def get_current_time() -> str:
    """获取当前时间，当用户问现在几点或者日期时使用"""
    return datetime.now().strftime("%Y-%m-%d %H:%M:%S")


@mcp.tool()
def calculate(expression: str) -> str:
    """
    计算数学表达式，当用户需要计算数学题时使用
    
    Args:
        expression: 要计算的数学表达式，如 2+3*4
    """
    try:
        allowed_chars = set("0123456789+-*/.() ")
        if all(c in allowed_chars for c in expression):
            return str(eval(expression))
        return "表达式包含不允许的字符"
    except Exception as e:
        return f"计算错误: {str(e)}"


@mcp.tool()
def search_web(query: str) -> str:
    """
    搜索网页信息，当用户询问需要查找信息时使用
    
    Args:
        query: 搜索关键词
    """
    return f"搜索结果 for '{query}': 这是一个模拟的搜索结果"
