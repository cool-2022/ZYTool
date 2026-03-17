TOOL_SCHEMA = {
    "type": "function",
    "function": {
        "name": "search_web",
        "description": "搜索网页信息，当用户询问需要查找信息时使用",
        "parameters": {
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "搜索关键词"
                }
            },
            "required": ["query"]
        }
    }
}


def search_web(query: str) -> str:
    """搜索网页（模拟）"""
    return f"搜索结果 for '{query}': 这是一个模拟的搜索结果"
