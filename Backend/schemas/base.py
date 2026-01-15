from pydantic import BaseModel, Field
from typing import Optional


class BaseResponse(BaseModel):
    success: bool = Field(..., description="操作是否成功")
    message: Optional[str] = Field(None, description="响应消息")
