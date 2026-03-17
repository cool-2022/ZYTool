import json
import asyncio


from app.services.public import deepseek_client
from app.api.v1.endpoints.agents.tools import AVAILABLE_TOOLS, execute_tool


def generate_stream(message: str):
    """流式生成回复，支持 Function Call"""
    print('1:message',message)
    print('2:tools',AVAILABLE_TOOLS)
    # 第一轮：判断是否需要调用工具
    result = deepseek_client.chat(message, tools=AVAILABLE_TOOLS)

    print('3:result',result)
    # 如果有 tool call，执行工具
    if result["tool_call"]:
        tool_call = result["tool_call"]
        tool_name = tool_call.function.name
        arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}

        # 执行工具
        tool_result = execute_tool(tool_name, arguments)

        # 构建消息历史
        messages_history = [
            {"role": "system", "content": "You are a helpful assistant"},
            {"role": "user", "content": message},
        ]

        # 添加 assistant 的 tool call
        messages_history.append({
            "role": "assistant",
            "tool_calls": [{
                "id": tool_call.id,
                "type": "function",
                "function": {
                    "name": tool_name,
                    "arguments": tool_call.function.arguments or "{}"
                }
            }]
        })

        # 添加 tool 结果
        messages_history.append({
            "role": "tool",
            "tool_call_id": tool_call.id,
            "content": tool_result
        })

        # 流式调用最终回复
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
        # 无需工具调用，直接流式返回
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