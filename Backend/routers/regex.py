from fastapi import APIRouter
from typing import Dict, Any
import re

from schemas import RegexTestRequest
from core import ValidationException

router = APIRouter(prefix="/api/regex", tags=["regex"])


@router.post("/test", summary="测试正则表达式", description="测试正则表达式并返回匹配结果")
async def test_regex(request: RegexTestRequest) -> Dict[str, Any]:
    """
    测试正则表达式
    
    在指定文本中搜索匹配正则表达式的内容，返回所有匹配结果
    """
    try:
        pattern = re.compile(request.pattern)
        matches = pattern.findall(request.text)
        match_objects = []

        for match in pattern.finditer(request.text):
            match_objects.append({
                "match": match.group(),
                "start": match.start(),
                "end": match.end(),
                "groups": match.groups()
            })

        return {
            "success": True,
            "matches": matches,
            "match_count": len(matches),
            "match_details": match_objects
        }

    except re.error as e:
        raise ValidationException(f"正则表达式错误: {str(e)}")
