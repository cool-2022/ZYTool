from fastapi import APIRouter
from typing import Dict, Any
import secrets
import string

from schemas.request.password import PasswordGenerateRequest
from schemas.response.password import PasswordGenerateResponse
from services.password_service import PasswordService
from core.exceptions import ValidationException

router = APIRouter(prefix="/password", tags=["password"])


@router.post("/generate", summary="生成安全密码", response_model=PasswordGenerateResponse)
async def generate_password(request: PasswordGenerateRequest) -> Dict[str, Any]:
    """
    生成安全密码

    可以指定密码长度和包含的字符类型（大小写字母、数字、符号）
    """
    password = PasswordService.generate_password(
        length=request.length,
        include_symbols=request.include_symbols,
        include_numbers=request.include_numbers,
        include_uppercase=request.include_uppercase,
        include_lowercase=request.include_lowercase
    )

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
