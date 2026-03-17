from pydantic import BaseModel, Field
from typing import List, Dict, Any

from ..base import BaseResponse


class TextProcessResponse(BaseResponse):
    result: str = Field(..., description="处理结果")


class TextCompareResponse(BaseModel):
    differences: list = Field(..., description="差异列表")
    summary: dict = Field(..., description="摘要信息")
