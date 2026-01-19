from pydantic import BaseModel, Field
from typing import Optional, List


class TokenResponse(BaseModel):
    """Token 响应"""
    access_token: str = Field(..., description="访问 token")
    token_type: str = Field(default="bearer", description="Token 类型")
    expires_in: int = Field(..., description="过期时间（秒）")


class UserInfoResponse(BaseModel):
    """用户信息响应"""
    username: str = Field(..., description="用户名")
    user_id: Optional[int] = Field(None, description="用户ID")
    roles: List[str] = Field(default=[], description="用户角色")


class LoginResponse(BaseModel):
    """登录响应"""
    success: bool = Field(..., description="是否成功")
    message: str = Field(..., description="响应消息")
    data: Optional[TokenResponse] = Field(None, description="Token 数据")
