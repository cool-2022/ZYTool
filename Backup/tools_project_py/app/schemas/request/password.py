from pydantic import BaseModel, Field, validator
from typing import Literal


class PasswordGenerateRequest(BaseModel):
    length: int = Field(default=12, ge=4, le=128, description="密码长度")
    include_symbols: bool = Field(default=True, description="是否包含符号")
    include_numbers: bool = Field(default=True, description="是否包含数字")
    include_uppercase: bool = Field(default=True, description="是否包含大写字母")
    include_lowercase: bool = Field(default=True, description="是否包含小写字母")

    @validator('length')
    def validate_length(cls, v, values):
        if not any([
            values.get('include_symbols', True),
            values.get('include_numbers', True),
            values.get('include_uppercase', True),
            values.get('include_lowercase', True)
        ]):
            raise ValueError("至少需要选择一种字符类型")
        return v
