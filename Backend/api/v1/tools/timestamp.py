from fastapi import APIRouter
from typing import Dict, Any
from datetime import datetime

from schemas.request.timestamp import TimestampConvertRequest
from schemas.response.timestamp import TimestampConvertResponse
from services.timestamp_service import TimestampService
from core.exceptions import ValidationException

router = APIRouter(prefix="/timestamp", tags=["timestamp"])


@router.post("/convert", summary="转换时间戳", response_model=TimestampConvertResponse)
async def convert_timestamp(request: TimestampConvertRequest) -> Dict[str, Any]:
    """
    转换时间戳

    支持两种转换类型：
    - to_datetime: 将时间戳转换为日期时间字符串
    - to_timestamp: 将日期时间转换为时间戳
    """
    try:
        result = TimestampService.convert_timestamp(
            request.timestamp, request.action.value)
        return result
    except ValueError as e:
        raise ValidationException(str(e))
