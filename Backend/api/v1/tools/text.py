from fastapi import APIRouter
from typing import Dict, Any

from schemas.request.text import TextProcessRequest, TextCompareRequest
from schemas.response.text import TextProcessResponse, TextCompareResponse
from services.Tools.text_service import TextService
from core.exceptions import ValidationException

router = APIRouter(prefix="/text", tags=["text"])


@router.post("/process", summary="处理文本", response_model=TextProcessResponse)
async def process_text(request: TextProcessRequest) -> Dict[str, Any]:
    """
    处理文本，根据指定的操作类型进行相应处理

    支持的操作类型：
    - json_format: JSON格式化
    - base64_encode: Base64编码
    - base64_decode: Base64解码
    - url_encode: URL编码
    - url_decode: URL解码
    """
    try:
        result = TextService.process_text(request.action.value, request.text)
        return {"result": result, "success": True}
    except ValueError as e:
        raise ValidationException(str(e))


@router.post("/compare", summary="比较文本差异", response_model=TextCompareResponse)
async def compare_text(request: TextCompareRequest) -> Dict[str, Any]:
    """
    比较两个文本的差异

    返回差异列表和摘要信息
    """
    result = TextService.compare_text(request.text1, request.text2)
    return result
