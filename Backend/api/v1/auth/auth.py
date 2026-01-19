"""
认证路由
提供登录、注册、token 验证等功能
"""

from fastapi import APIRouter, Depends, HTTPException, status
from datetime import timedelta
from core.auth import (
    create_access_token,
    verify_password,
    get_password_hash,
    get_current_user
)
from core.config import settings
from schemas import (
    LoginRequest,
    RegisterRequest,
    TokenResponse,
    UserInfoResponse,
    LoginResponse
)
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/auth", tags=["authentication"])

# 模拟用户数据库（实际项目中应该使用真实数据库）
fake_users_db = {
    "admin": {
        "username": "admin",
        "user_id": 1,
        "hashed_password": get_password_hash("admin123"),  # 密码: admin123
        "roles": ["admin", "user"]
    },
    "user": {
        "username": "user",
        "user_id": 2,
        "hashed_password": get_password_hash("user123"),  # 密码: user123
        "roles": ["user"]
    }
}


@router.post("/login", response_model=LoginResponse)
async def login(request: LoginRequest):
    """
    用户登录
    
    - **username**: 用户名
    - **password**: 密码
    
    返回 JWT token
    """
    logger.info(f"Login attempt for user: {request.username}")
    
    # 查找用户
    user = fake_users_db.get(request.username)
    if not user:
        logger.warning(f"Login failed: user not found - {request.username}")
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="用户名或密码错误"
        )
    
    # 验证密码
    if not verify_password(request.password, user["hashed_password"]):
        logger.warning(f"Login failed: invalid password - {request.username}")
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="用户名或密码错误"
        )
    
    # 创建 token
    access_token_expires = timedelta(minutes=settings.access_token_expire_minutes)
    access_token = create_access_token(
        data={
            "sub": user["username"],
            "user_id": user["user_id"],
            "roles": user["roles"]
        },
        expires_delta=access_token_expires
    )
    
    logger.info(f"Login successful: {request.username}")
    
    return LoginResponse(
        success=True,
        message="登录成功",
        data=TokenResponse(
            access_token=access_token,
            token_type="bearer",
            expires_in=settings.access_token_expire_minutes * 60
        )
    )


@router.post("/register", response_model=LoginResponse)
async def register(request: RegisterRequest):
    """
    用户注册
    
    - **username**: 用户名
    - **password**: 密码
    - **email**: 邮箱（可选）
    
    返回 JWT token
    """
    logger.info(f"Register attempt for user: {request.username}")
    
    # 检查用户是否已存在
    if request.username in fake_users_db:
        logger.warning(f"Register failed: user already exists - {request.username}")
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="用户名已存在"
        )
    
    # 创建新用户
    new_user_id = len(fake_users_db) + 1
    fake_users_db[request.username] = {
        "username": request.username,
        "user_id": new_user_id,
        "hashed_password": get_password_hash(request.password),
        "roles": ["user"]
    }
    
    # 创建 token
    access_token_expires = timedelta(minutes=settings.access_token_expire_minutes)
    access_token = create_access_token(
        data={
            "sub": request.username,
            "user_id": new_user_id,
            "roles": ["user"]
        },
        expires_delta=access_token_expires
    )
    
    logger.info(f"Register successful: {request.username}")
    
    return LoginResponse(
        success=True,
        message="注册成功",
        data=TokenResponse(
            access_token=access_token,
            token_type="bearer",
            expires_in=settings.access_token_expire_minutes * 60
        )
    )


@router.get("/me", response_model=UserInfoResponse)
async def get_current_user_info(current_user: dict = Depends(get_current_user)):
    """
    获取当前用户信息
    
    需要在请求头中提供有效的 token:
    ```
    Authorization: Bearer <your_token>
    ```
    """
    return UserInfoResponse(
        username=current_user["username"],
        user_id=current_user.get("user_id"),
        roles=current_user.get("roles", [])
    )


@router.post("/logout")
async def logout(current_user: dict = Depends(get_current_user)):
    """
    用户登出
    
    注意：JWT token 是无状态的，实际的登出需要在客户端删除 token
    这个端点主要用于记录日志
    """
    logger.info(f"User logged out: {current_user['username']}")
    return {
        "success": True,
        "message": "登出成功"
    }
