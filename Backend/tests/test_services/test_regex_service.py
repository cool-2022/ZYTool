import pytest
from services.regex_service import RegexService


def test_regex_simple_match():
    """测试简单的正则表达式匹配"""
    result = RegexService.test_regex(r"\d+", "abc123def456")
    assert result["success"] is True
    assert result["match_count"] == 2
    assert "123" in result["matches"]
    assert "456" in result["matches"]


def test_regex_email_pattern():
    """测试邮箱正则表达式"""
    result = RegexService.test_regex(
        r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}", "Contact us at test@example.com")
    assert result["success"] is True
    assert result["match_count"] == 1
    assert "test@example.com" in result["matches"]


def test_regex_no_match():
    """测试没有匹配的情况"""
    result = RegexService.test_regex(r"\d+", "abcdef")
    assert result["success"] is True
    assert result["match_count"] == 0
    assert len(result["matches"]) == 0


def test_regex_invalid_pattern():
    """测试无效的正则表达式"""
    with pytest.raises(ValueError):
        RegexService.test_regex("[invalid(", "test")
