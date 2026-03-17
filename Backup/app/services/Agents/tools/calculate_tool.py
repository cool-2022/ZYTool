TOOL_SCHEMA = {
    "type": "function",
    "function": {
        "name": "calculate",
        "description": "计算数学表达式，当用户需要计算数学题时使用",
        "parameters": {
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "要计算的数学表达式，如 2+3*4"
                }
            },
            "required": ["expression"]
        }
    }
}


def calculate(expression: str) -> str:
    """计算数学表达式，只允许基本运算符"""
    try:
        allowed_chars = set("0123456789+-*/.() ")
        if all(c in allowed_chars for c in expression):
            result = eval(expression)
            return str(result)
        return "表达式包含不允许的字符"
    except Exception as e:
        return f"计算错误: {str(e)}"
