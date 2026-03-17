from pydantic import BaseModel, Field, validator, constr
from typing import Optional


class RegexTestRequest(BaseModel):
    pattern: constr(min_length=1, max_length=1000) = Field(...,
                                                           description="正则表达式模式")
    text: constr(min_length=0, max_length=100000) = Field(...,
                                                          description="要匹配的文本")
    flags: Optional[str] = Field(default=None, description="正则表达式标志")

    @validator('pattern')
    def validate_pattern(cls, v):
        if not v or not v.strip():
            raise ValueError("正则表达式不能为空")
        return v
