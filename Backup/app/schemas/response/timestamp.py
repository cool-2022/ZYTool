from pydantic import Field
from typing import Optional

from ..base import BaseResponse


class TimestampConvertResponse(BaseResponse):
    result: str = Field(..., description="转换结果")
    timestamp: int = Field(default=0, description="原始时间戳")
    action: str = Field(default="", description="操作类型")
