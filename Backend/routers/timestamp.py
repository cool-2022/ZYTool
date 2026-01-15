from fastapi import APIRouter
from typing import Dict, Any
from datetime import datetime

from schemas import TimestampConvertRequest
from core import ValidationException

router = APIRouter(prefix="/api/timestamp", tags=["timestamp"])


@router.post("/convert", summary="转换时间戳", description="在时间戳和日期时间之间进行转换")
async def convert_timestamp(request: TimestampConvertRequest) -> Dict[str, Any]:
    """
    转换时间戳
    
    支持两种转换类型：
    - to_datetime: 将时间戳转换为日期时间字符串
    - to_timestamp: 将日期时间转换为时间戳
    """
    try:
        if request.action.value == "to_datetime":
            dt = datetime.fromtimestamp(request.timestamp)
            return {
                "success": True,
                "result": dt.strftime("%Y-%m-%d %H:%M:%S"),
                "timestamp": request.timestamp,
                "action": request.action.value
            }

        elif request.action.value == "to_timestamp":
            return {
                "success": True,
                "result": str(request.timestamp),
                "timestamp": request.timestamp,
                "action": request.action.value
            }

        else:
            raise ValidationException("不支持的操作类型")

    except Exception as e:
        raise ValidationException(f"时间戳转换错误: {str(e)}")
