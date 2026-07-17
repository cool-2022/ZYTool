import re
import json
import base64
import urllib.parse
from typing import Any, Dict, List, Optional


def validate_json(text: str) -> Dict[str, Any]:
    """
    验证并解析JSON字符串
    
    Args:
        text: JSON格式的字符串
        
    Returns:
        解析后的字典
        
    Raises:
        ValueError: 当JSON格式无效时
    """
    try:
        return json.loads(text)
    except json.JSONDecodeError as e:
        raise ValueError(f"无效的JSON格式: {str(e)}")


def format_json(data: Any, indent: int = 2, ensure_ascii: bool = False) -> str:
    """
    格式化JSON数据为字符串
    
    Args:
        data: 要格式化的数据
        indent: 缩进空格数，默认为2
        ensure_ascii: 是否确保ASCII编码，默认为False
        
    Returns:
        格式化后的JSON字符串
    """
    return json.dumps(data, indent=indent, ensure_ascii=ensure_ascii)


def base64_encode(text: str) -> str:
    """
    将文本编码为Base64
    
    Args:
        text: 要编码的文本
        
    Returns:
        Base64编码后的字符串
    """
    return base64.b64encode(text.encode('utf-8')).decode('utf-8')


def base64_decode(text: str) -> str:
    """
    解码Base64字符串为文本
    
    Args:
        text: Base64编码的字符串
        
    Returns:
        解码后的文本
        
    Raises:
        ValueError: 当Base64解码失败时
    """
    try:
        return base64.b64decode(text).decode('utf-8')
    except Exception as e:
        raise ValueError(f"Base64解码失败: {str(e)}")


def url_encode(text: str) -> str:
    """
    对文本进行URL编码
    
    Args:
        text: 要编码的文本
        
    Returns:
        URL编码后的字符串
    """
    return urllib.parse.quote(text)


def url_decode(text: str) -> str:
    """
    解码URL编码的文本
    
    Args:
        text: URL编码的字符串
        
    Returns:
        解码后的文本
    """
    return urllib.parse.unquote(text)


def validate_regex(pattern: str) -> bool:
    """
    验证正则表达式是否有效
    
    Args:
        pattern: 正则表达式模式
        
    Returns:
        如果正则表达式有效返回True，否则返回False
    """
    try:
        re.compile(pattern)
        return True
    except re.error:
        return False


def split_text_lines(text: str) -> List[str]:
    """
    将文本分割为行列表
    
    Args:
        text: 要分割的文本
        
    Returns:
        行列表
    """
    return text.splitlines()


def compare_lines(text1: str, text2: str) -> List[Dict[str, Any]]:
    """
    比较两个文本的差异
    
    Args:
        text1: 第一个文本
        text2: 第二个文本
        
    Returns:
        差异列表，每个差异包含行号、文本内容和类型
    """
    lines1 = split_text_lines(text1)
    lines2 = split_text_lines(text2)
    
    differences = []
    max_lines = max(len(lines1), len(lines2))
    
    for i in range(max_lines):
        line1 = lines1[i] if i < len(lines1) else ""
        line2 = lines2[i] if i < len(lines2) else ""
        
        if line1 != line2:
            differences.append({
                "line": i + 1,
                "text1": line1,
                "text2": line2,
                "type": "modified" if line1 and line2 else ("added" if line2 else "removed")
            })
    
    return differences


def truncate_string(text: str, max_length: int = 100, suffix: str = "...") -> str:
    """
    截断字符串到指定长度
    
    Args:
        text: 要截断的文本
        max_length: 最大长度，默认为100
        suffix: 截断后添加的后缀，默认为"..."
        
    Returns:
        截断后的字符串
    """
    if len(text) <= max_length:
        return text
    return text[:max_length - len(suffix)] + suffix


def sanitize_filename(filename: str) -> str:
    """
    清理文件名，移除非法字符
    
    Args:
        filename: 原始文件名
        
    Returns:
        清理后的文件名
    """
    return re.sub(r'[<>:"/\\|?*]', '_', filename)
