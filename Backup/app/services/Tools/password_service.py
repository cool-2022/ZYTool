import secrets
import string
from typing import Dict, Any


class PasswordService:
    """密码服务类，提供安全密码生成功能"""

    @staticmethod
    def generate_password(length: int, include_symbols: bool, include_numbers: bool,
                          include_uppercase: bool, include_lowercase: bool) -> str:
        """
        生成安全密码

        Args:
            length: 密码长度
            include_symbols: 是否包含符号
            include_numbers: 是否包含数字
            include_uppercase: 是否包含大写字母
            include_lowercase: 是否包含小写字母

        Returns:
            生成的安全密码字符串
        """
        chars = ""
        if include_lowercase:
            chars += string.ascii_lowercase
        if include_uppercase:
            chars += string.ascii_uppercase
        if include_numbers:
            chars += string.digits
        if include_symbols:
            chars += "!@#$%^&*()_+-=[]{}|;:,.<>?"

        if not chars:
            raise ValueError("至少需要选择一种字符类型")

        password = ''.join(secrets.choice(chars) for _ in range(length))
        return password
