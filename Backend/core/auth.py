"""
JWT 认证模块
提供 token 生成、验证和用户认证功能
"""

from datetime import datetime, timedelta
from typing import Optional
from jose import JWTError, jwt
from passlib.context import CryptContext
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPBearer, HTTPAuthorizationCredentials
from core.config import settings
import logging

logger = logging.getLogger(__name__)

# 密码加密上下文
pwd_context = CryptContext(schemes=["bcrypt"], deprecated="auto")

# HTTP Bearer 认证方案
security = HTTPBearer()


def verify_password(plain_password: str, hashed_password: str) -> bool:
    """验证密码"""
    return pwd_context.verify(plain_password, hashed_password)


def get_password_hash(password: str) -> str:
    """生成密码哈希"""
    return pwd_context.hash(password)


def create_access_token(data: dict, expires_delta: Optional[timedelta] = None) -> str:
    """
    创建访问 token
    
    Args:
        data: 要编码的数据（通常包含用户信息）
        expires_delta: token 过期时间
        
    Returns:
        JWT token 字符串
    """
    to_encode = data.copy()
    
    if expires_delta:
        expire = datetime.utcnow() + expires_delta
    else:
        expire = datetime.utcnow() + timedelta(minutes=settings.access_token_expire_minutes)
    
    to_encode.update({"exp": expire})
    encoded_jwt = jwt.encode(to_encode, settings.secret_key, algorithm=settings.algorithm)
    
    logger.info(f"Created access token for: {data.get('sub', 'unknown')}")
    return encoded_jwt


def decode_access_token(token: str) -> Optional[dict]:
    """
    解码访问 token
    
    Args:
        token: JWT token 字符串
        
    Returns:
        解码后的数据，如果失败返回 None
    """
    try:
        payload = jwt.decode(token, settings.secret_key, algorithms=[settings.algorithm])
        return payload
    except JWTError as e:
        logger.error(f"Token decode error: {str(e)}")
        return None


async def get_current_user(credentials: HTTPAuthorizationCredentials = Depends(security)) -> dict:
    """
    获取当前用户（依赖注入）
    
    从请求头中提取 token 并验证，返回用户信息
    
    Args:
        credentials: HTTP Authorization 凭证
        
    Returns:
        用户信息字典
        
    Raises:
        HTTPException: 如果 token 无效或过期
    """
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="无法验证凭证",
        headers={"WWW-Authenticate": "Bearer"},
    )
    
    token = credentials.credentials
    payload = decode_access_token(token)
    
    if payload is None:
        raise credentials_exception
    
    username: str = payload.get("sub")
    if username is None:
        raise credentials_exception
    
    # 这里可以从数据库查询用户信息
    # 目前返回 token 中的信息
    return {
        "username": username,
        "user_id": payload.get("user_id"),
        "roles": payload.get("roles", [])
    }


async def get_optional_user(credentials: Optional[HTTPAuthorizationCredentials] = Depends(security)) -> Optional[dict]:
    """
    获取当前用户（可选）
    
    如果提供了 token 则验证，否则返回 None
    用于可选认证的端点
    
    Args:
        credentials: HTTP Authorization 凭证（可选）
        
    Returns:
        用户信息字典或 None
    """
    if credentials is None:
        return None
    
    try:
        return await get_current_user(credentials)
    except HTTPException:
        return None
