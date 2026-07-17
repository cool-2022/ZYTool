from pydantic import BaseModel, Field, validator, constr
from typing import Literal
from enum import Enum


class TextAction(str, Enum):
    json_format = "json_format"
    base64_encode = "base64_encode"
    base64_decode = "base64_decode"
    url_encode = "url_encode"
    url_decode = "url_decode"


class TextProcessRequest(BaseModel):
    text: constr(min_length=1, max_length=100000) = Field(...,
                                                          description="要处理的文本内容")
    action: TextAction = Field(..., description="操作类型")

    @validator('text')
    def validate_text_not_empty(cls, v):
        if not v or not v.strip():
            raise ValueError("文本内容不能为空")
        return v


class TextCompareRequest(BaseModel):
    text1: constr(min_length=0, max_length=100000) = Field(...,
                                                           description="第一个文本")
    text2: constr(min_length=0, max_length=100000) = Field(...,
                                                           description="第二个文本")
