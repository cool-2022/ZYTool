from pydantic import BaseModel, Field
from typing import Optional


class BaseResponse(BaseModel):
    """基础响应模型"""
    success: bool = Field(..., description="操作是否成功")
    message: Optional[str] = Field(None, description="响应消息")
