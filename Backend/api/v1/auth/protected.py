"""
受保护的路由示例
演示如何使用 token 验证保护 API 端点
"""

from fastapi import APIRouter, Depends
from core.auth import get_current_user
import logging

logger = logging.getLogger(__name__)

router = APIRouter(prefix="/protected", tags=["protected"])


@router.get("/data")
async def get_protected_data(current_user: dict = Depends(get_current_user)):
    """
    获取受保护的数据
    
    需要在请求头中提供有效的 token:
    ```
    Authorization: Bearer <your_token>
    ```
    """
    logger.info(f"Protected data accessed by: {current_user['username']}")
    
    return {
        "success": True,
        "message": "这是受保护的数据",
        "data": {
            "user": current_user["username"],
            "secret_info": "只有认证用户才能看到这些信息"
        }
    }


@router.get("/admin")
async def admin_only(current_user: dict = Depends(get_current_user)):
    """
    仅管理员可访问
    
    检查用户角色是否包含 'admin'
    """
    if "admin" not in current_user.get("roles", []):
        logger.warning(f"Unauthorized admin access attempt by: {current_user['username']}")
        return {
            "success": False,
            "message": "权限不足，仅管理员可访问"
        }
    
    logger.info(f"Admin endpoint accessed by: {current_user['username']}")
    
    return {
        "success": True,
        "message": "欢迎，管理员！",
        "data": {
            "admin_info": "这是管理员专属信息"
        }
    }
