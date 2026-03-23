"""
MCP Client - 通过 MCP 协议调用工具，并将结果桥接给 DeepSeek
"""
import json
import asyncio
from typing import AsyncGenerator
from mcp import ClientSession, StdioServerParameters
from mcp.client.stdio import stdio_client
from mcp.types import Tool

from app.services.Agents.deepseek_client import deepseek_client


def _tools_to_openai_schema(tools: list[Tool]) -> list[dict]:
    """将 MCP tool 列表转换为 OpenAI function calling schema"""
    return [
        {
            "type": "function",
            "function": {
                "name": tool.name,
                "description": tool.description or "",
                "parameters": tool.inputSchema,
            }
        }
        for tool in tools
    ]


async def run_mcp_agent(message: str) -> str:
    """
    通过 MCP 协议运行 Agent（同步模式）
    1. 启动 MCP server
    2. 获取可用工具列表
    3. 调用 DeepSeek，传入工具 schema
    4. 若有 tool call，通过 MCP 执行工具
    5. 将结果返回给 DeepSeek 生成最终回复
    """
    server_params = StdioServerParameters(
        command="python",
        args=["-m", "app.services.MCP.mcp_server"],
    )

    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()

            # 获取 MCP server 提供的工具列表
            tools_result = await session.list_tools()
            openai_tools = _tools_to_openai_schema(tools_result.tools)

            # 第一轮：让 DeepSeek 判断是否需要调用工具
            result = deepseek_client.chat(message, tools=openai_tools)

            # 多轮 tool call 循环
            while result["tool_call"]:
                tool_call = result["tool_call"]
                tool_name = tool_call.function.name
                arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}

                # 通过 MCP 执行工具
                tool_result = await session.call_tool(tool_name, arguments)
                tool_content = tool_result.content[0].text if tool_result.content else ""

                # 将工具结果传回 DeepSeek
                result = deepseek_client.chat(
                    f"用户问题: {message}\n\n工具结果: {tool_content}",
                    tools=openai_tools
                )

            return result["content"] or ""


async def run_mcp_agent_stream(message: str) -> AsyncGenerator[str, None]:
    """
    通过 MCP 协议运行 Agent（流式模式）
    tool call 阶段同步执行，最终回复流式输出
    """
    server_params = StdioServerParameters(
        command="python",
        args=["-m", "app.services.MCP.mcp_server"],
    )

    async with stdio_client(server_params) as (read, write):
        async with ClientSession(read, write) as session:
            await session.initialize()

            tools_result = await session.list_tools()
            openai_tools = _tools_to_openai_schema(tools_result.tools)

            # 第一轮同步判断是否需要 tool call
            result = deepseek_client.chat(message, tools=openai_tools)

            if result["tool_call"]:
                tool_call = result["tool_call"]
                tool_name = tool_call.function.name
                arguments = json.loads(tool_call.function.arguments) if tool_call.function.arguments else {}

                tool_result = await session.call_tool(tool_name, arguments)
                tool_content = tool_result.content[0].text if tool_result.content else ""

                # 最终回复流式输出
                for chunk in deepseek_client.chat_stream(
                    f"用户问题: {message}\n\n工具结果: {tool_content}"
                ):
                    yield chunk
                    await asyncio.sleep(0.01)
            else:
                for chunk in deepseek_client.chat_stream(message):
                    yield chunk
                    await asyncio.sleep(0.01)
