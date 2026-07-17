from pydantic import Field
from typing import Optional

from ..base import BaseResponse


class PasswordGenerateResponse(BaseResponse):
    password: str = Field(..., description="生成的密码")
    length: int = Field(default=0, description="密码长度")
    character_types: dict = Field(default={}, description="字符类型")
