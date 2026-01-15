from pydantic import BaseModel, Field
from enum import Enum


class TimestampAction(str, Enum):
    to_datetime = "to_datetime"
    to_timestamp = "to_timestamp"


class TimestampConvertRequest(BaseModel):
    timestamp: int = Field(..., ge=0, le=9999999999, description="时间戳")
    action: TimestampAction = Field(..., description="转换类型")
