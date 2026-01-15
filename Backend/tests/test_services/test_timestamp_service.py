import pytest
from services.timestamp_service import TimestampService


def test_timestamp_to_datetime():
    """测试时间戳转换为日期时间"""
    result = TimestampService.convert_timestamp(1609459200, "to_datetime")
    assert result["success"] is True
    assert "result" in result
    assert result["action"] == "to_datetime"


def test_timestamp_to_timestamp():
    """测试时间戳转换"""
    result = TimestampService.convert_timestamp(1609459200, "to_timestamp")
    assert result["success"] is True
    assert result["result"] == "1609459200"
    assert result["action"] == "to_timestamp"


def test_timestamp_invalid_action():
    """测试无效的操作类型"""
    with pytest.raises(ValueError):
        TimestampService.convert_timestamp(1609459200, "invalid_action")
