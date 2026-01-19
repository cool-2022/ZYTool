import re
from typing import Dict, Any, List


class RegexService:
    """正则表达式服务类，提供正则表达式测试功能"""

    @staticmethod
    def test_regex(pattern: str, text: str) -> Dict[str, Any]:
        """
        测试正则表达式

        Args:
            pattern: 正则表达式模式
            text: 要匹配的文本

        Returns:
            包含匹配结果的字典
        """
        try:
            compiled_pattern = re.compile(pattern)
            matches = compiled_pattern.findall(text)
            match_objects = []

            for match in compiled_pattern.finditer(text):
                match_objects.append({
                    "match": match.group(),
                    "start": match.start(),
                    "end": match.end(),
                    "groups": match.groups()
                })

            return {
                "success": True,
                "matches": matches,
                "match_count": len(matches),
                "match_details": match_objects
            }
        except re.error as e:
            raise ValueError(f"正则表达式错误: {str(e)}")
