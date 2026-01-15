import pytest
from services.text_service import TextService


def test_text_process_json_format():
    """测试JSON格式化功能"""
    result = TextService.process_text(
        "json_format", '{"name":"test","value":123}')
    assert isinstance(result, str)
    assert "test" in result
    assert "value" in result


def test_text_process_base64_encode():
    """测试Base64编码功能"""
    result = TextService.process_text("base64_encode", "Hello World")
    assert result == "SGVsbG8gV29ybGQ="


def test_text_process_base64_decode():
    """测试Base64解码功能"""
    result = TextService.process_text("base64_decode", "SGVsbG8gV29ybGQ=")
    assert result == "Hello World"


def test_text_process_url_encode():
    """测试URL编码功能"""
    result = TextService.process_text("url_encode", "hello world")
    assert result == "hello%20world"


def test_text_process_url_decode():
    """测试URL解码功能"""
    result = TextService.process_text("url_decode", "hello%20world")
    assert result == "hello world"


def test_text_process_invalid_action():
    """测试无效操作类型"""
    with pytest.raises(ValueError):
        TextService.process_text("invalid_action", "test")


def test_text_compare_identical():
    """测试相同文本比较"""
    result = TextService.compare_text("line1\nline2", "line1\nline2")
    assert result["summary"]["identical"] is True
    assert result["summary"]["different_lines"] == 0


def test_text_compare_different():
    """测试不同文本比较"""
    result = TextService.compare_text("line1\nline2", "line1\nline3")
    assert result["summary"]["identical"] is False
    assert result["summary"]["different_lines"] > 0
