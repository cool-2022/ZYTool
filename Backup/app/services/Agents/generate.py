import json
import asyncio


from app.services.Agents.deepseek_client import deepseek_client
from app.services.Agents.tools import AVAILABLE_TOOLS, execute_tool


async def generate_stream(message: str):
    """流式生成回复，支持 Function Call"""
    result = deepseek_client.chat(message, tools=AVAILABLE_TOOLS)

    if result["tool_call"]:
        tool_call = result["tool_call"]
        tool_name = tool_call.function.name
        arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}

        tool_result = execute_tool(tool_name, arguments)

        messages_history = [
            {"role": "system", "content": "You are a helpful assistant"},
            {"role": "user", "content": message},
            {
                "role": "assistant",
                "tool_calls": [{
                    "id": tool_call.id,
                    "type": "function",
                    "function": {
                        "name": tool_name,
                        "arguments": tool_call.function.arguments or "{}"
                    }
                }]
            },
            {
                "role": "tool",
                "tool_call_id": tool_call.id,
                "content": tool_result
            }
        ]

        try:
            final_response = deepseek_client.client.chat.completions.create(
                model="deepseek-chat",
                messages=messages_history,
                stream=True
            )
            for chunk in final_response:
                if chunk.choices[0].delta.content:
                    yield f"data: {chunk.choices[0].delta.content}\n\n"
                    await asyncio.sleep(0.01)
        except Exception as e:
            yield f"data: 工具执行失败: {str(e)}\n\n"
    else:
        for char in deepseek_client.chat_stream(message):
            yield f"data: {char}\n\n"
            await asyncio.sleep(0.01)

    yield "data: [DONE]\n\n"



def generate_sync(message: str) -> str:
    """同步生成回复，支持多轮 Function Call"""
    result = deepseek_client.chat(message, tools=AVAILABLE_TOOLS)

    if result["tool_call"]:
        tool_call = result["tool_call"]
        tool_name = tool_call.function.name
        arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}
        tool_result = execute_tool(tool_name, arguments)

        final_result = deepseek_client.chat(
            f"用户问题: {message}\n\n工具结果: {tool_result}",
            tools=AVAILABLE_TOOLS
        )

        while final_result["tool_call"]:
            tool_call = final_result["tool_call"]
            tool_name = tool_call.function.name
            arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}
            tool_result = execute_tool(tool_name, arguments)

            final_result = deepseek_client.chat(
                f"用户问题: {message}\n\n工具结果: {tool_result}",
                tools=AVAILABLE_TOOLS
            )
        return final_result["content"]

    return result["content"]