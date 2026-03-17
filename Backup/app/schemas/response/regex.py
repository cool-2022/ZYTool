from pydantic import Field
from typing import List, Any

from ..base import BaseResponse


class RegexTestResponse(BaseResponse):
    matches: list = Field(..., description="匹配结果")
    match_count: int = Field(..., description="匹配数量")
    match_details: List[dict] = Field(default=[], description="匹配详情")
