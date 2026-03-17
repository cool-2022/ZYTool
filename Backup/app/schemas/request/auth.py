from pydantic import BaseModel, Field


class LoginRequest(BaseModel):
    """登录请求"""
    username: str = Field(..., description="用户名", min_length=3, max_length=50)
    password: str = Field(..., description="密码", min_length=6, max_length=100)


class RegisterRequest(BaseModel):
    """注册请求"""
    username: str = Field(..., description="用户名", min_length=3, max_length=50)
    password: str = Field(..., description="密码", min_length=6, max_length=100)
    email: str = Field(None, description="邮箱（可选）")
