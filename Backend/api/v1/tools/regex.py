from fastapi import APIRouter
from typing import Dict, Any
import re

from schemas.request.regex import RegexTestRequest
from schemas.response.regex import RegexTestResponse
from services.regex_service import RegexService
from core.exceptions import ValidationException

router = APIRouter(prefix="/regex", tags=["regex"])


@router.post("/test", summary="测试正则表达式", response_model=RegexTestResponse)
async def test_regex(request: RegexTestRequest) -> Dict[str, Any]:
    """
    测试正则表达式

    在指定文本中搜索匹配正则表达式的内容，返回所有匹配结果
    """
    try:
        result = RegexService.test_regex(request.pattern, request.text)
        return result
    except ValueError as e:
        raise ValidationException(str(e))
