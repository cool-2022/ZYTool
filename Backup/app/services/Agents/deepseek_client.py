import os
from typing import Generator, Optional
from dotenv import load_dotenv
from openai import OpenAI

# 加载 .env 文件
env_path = os.path.join(os.path.dirname(__file__), "..", "..", ".env.development")
load_dotenv(env_path)


class DeepSeekClient:
    """DeepSeek AI Client using OpenAI SDK with streaming support."""

    def __init__(self):
        self.api_key: Optional[str] = os.getenv("DEEPSEEK_API_KEY")
        self.client: Optional[OpenAI] = None
        self._init_client()

    def _init_client(self) -> None:
        """初始化 DeepSeek 客户端"""
        if not self.api_key:
            print("Warning: DEEPSEEK_API_KEY not found")
            return

        try:
            self.client = OpenAI(
                api_key=self.api_key,
                base_url="https://api.deepseek.com"
            )
        except Exception as e:
            print(f"Failed to initialize DeepSeek client: {e}")

    def chat(
        self,
        prompt: str,
        tools: Optional[list] = None,
        tool_choice: Optional[str] = None
    ) -> dict:
        """同步对话"""
        if not self.client:
            return {"content": self._mock_response(prompt), "tool_call": None}

        try:
            messages = [
                {"role": "system", "content": "You are a helpful assistant"},
                {"role": "user", "content": prompt},
            ]

            extra_params = {}
            if tools:
                extra_params["tools"] = tools
            if tool_choice:
                extra_params["tool_choice"] = {"type": "function", "function": {"name": tool_choice}}

            response = self.client.chat.completions.create(
                model="deepseek-chat",
                messages=messages,
                stream=False,
                **extra_params
            )

            # 检查是否有 tool calls
            tool_call = None
            if response.choices[0].message.tool_calls:
                tool_call = response.choices[0].message.tool_calls[0]

            return {
                "content": response.choices[0].message.content,
                "tool_call": tool_call
            }
        except Exception as e:
            print(f"DeepSeek API error: {e}")
            return {"content": self._mock_response(prompt), "tool_call": None}

    def chat_stream(self, prompt: str) -> Generator[str, None, None]:
        """流式对话（不包含 function call）"""
        if not self.client:
            yield from self._mock_stream(prompt)
            return

        try:
            response = self.client.chat.completions.create(
                model="deepseek-chat",
                messages=[
                    {"role": "system", "content": "You are a helpful assistant"},
                    {"role": "user", "content": prompt},
                ],
                stream=True
            )
            for chunk in response:
                if chunk.choices[0].delta.content:
                    yield chunk.choices[0].delta.content
        except Exception as e:
            print(f"DeepSeek API error: {e}")
            yield f"错误: {e}"

    def _mock_response(self, prompt: str) -> str:
        """模拟回复"""
        return f"这是模拟回复: 我收到了你的消息 -> {prompt}"

    def _mock_stream(self, prompt: str) -> Generator[str, None, None]:
        """模拟流式回复"""
        mock_response = f"这是模拟回复: 我收到了你的消息 -> {prompt}"
        for char in mock_response:
            yield char


# 全局单例
deepseek_client = DeepSeekClient()
