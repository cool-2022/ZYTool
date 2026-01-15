from fastapi import APIRouter
from typing import Dict, Any

from schemas import TextProcessRequest, TextCompareRequest
from services.text_service import TextService
from core import ValidationException

router = APIRouter(prefix="/api/text", tags=["text"])


@router.post("/process", summary="处理文本", description="支持JSON格式化、Base64编码解码、URL编码解码")
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
        return result
    except ValueError as e:
        raise ValidationException(str(e))


@router.post("/compare", summary="比较文本差异", description="对比两个文本的差异并返回详细结果")
async def compare_text(request: TextCompareRequest) -> Dict[str, Any]:
    """
    比较两个文本的差异
    
    返回差异列表和摘要信息
    """
    result = TextService.compare_text(request.text1, request.text2)
    return result
