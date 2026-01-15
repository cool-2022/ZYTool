from fastapi import APIRouter
from typing import Dict, Any
import secrets
import string

from schemas import PasswordGenerateRequest
from core import ValidationException

router = APIRouter(prefix="/api/password", tags=["password"])


@router.post("/generate", summary="生成安全密码", description="根据指定参数生成随机密码")
async def generate_password(request: PasswordGenerateRequest) -> Dict[str, Any]:
    """
    生成安全密码
    
    可以指定密码长度和包含的字符类型（大小写字母、数字、符号）
    """
    chars = ""
    if request.include_lowercase:
        chars += string.ascii_lowercase
    if request.include_uppercase:
        chars += string.ascii_uppercase
    if request.include_numbers:
        chars += string.digits
    if request.include_symbols:
        chars += "!@#$%^&*()_+-=[]{}|;:,.<>?"

    if not chars:
        raise ValidationException("至少需要选择一种字符类型")

    password = ''.join(secrets.choice(chars) for _ in range(request.length))

    return {
        "success": True,
        "password": password,
        "length": request.length,
        "character_types": {
            "lowercase": request.include_lowercase,
            "uppercase": request.include_uppercase,
            "numbers": request.include_numbers,
            "symbols": request.include_symbols
        }
    }
