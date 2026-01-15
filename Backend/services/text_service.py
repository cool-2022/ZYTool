from typing import Dict, Any
from utils.text_utils import (
    validate_json,
    format_json,
    base64_encode,
    base64_decode,
    url_encode,
    url_decode,
    compare_lines
)


class TextService:
    """文本处理服务类，提供各种文本处理功能"""

    @staticmethod
    def process_text(action: str, text: str) -> Dict[str, Any]:
        """
        处理文本，根据指定的操作类型进行相应处理

        Args:
            action: 操作类型，支持以下值:
                - json_format: JSON格式化
                - base64_encode: Base64编码
                - base64_decode: Base64解码
                - url_encode: URL编码
                - url_decode: URL解码
            text: 要处理的文本内容

        Returns:
            包含处理结果的字典，格式为:
            {
                "result": "处理后的结果",
                "success": True
            }

        Raises:
            ValueError: 当不支持的操作类型时
        """
        if action == "json_format":
            data = validate_json(text)
            result = format_json(data)
            return {"result": result, "success": True}

        elif action == "base64_encode":
            result = base64_encode(text)
            return {"result": result, "success": True}

        elif action == "base64_decode":
            result = base64_decode(text)
            return {"result": result, "success": True}

        elif action == "url_encode":
            result = url_encode(text)
            return {"result": result, "success": True}

        elif action == "url_decode":
            result = url_decode(text)
            return {"result": result, "success": True}

        else:
            raise ValueError(f"不支持的操作类型: {action}")

    @staticmethod
    def compare_text(text1: str, text2: str) -> Dict[str, Any]:
        """
        比较两个文本的差异

        Args:
            text1: 第一个文本
            text2: 第二个文本

        Returns:
            包含比较结果的字典，格式为:
            {
                "differences": [...],  # 差异列表
                "summary": {
                    "total_lines": 总行数,
                    "different_lines": 不同行数,
                    "identical": 是否完全相同
                }
            }
        """
        differences = compare_lines(text1, text2)

        lines1 = text1.splitlines()
        lines2 = text2.splitlines()
        max_lines = max(len(lines1), len(lines2))

        return {
            "differences": differences,
            "summary": {
                "total_lines": max_lines,
                "different_lines": len(differences),
                "identical": len(differences) == 0
            }
        }
