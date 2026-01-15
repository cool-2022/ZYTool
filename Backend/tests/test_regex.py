import pytest
from fastapi.testclient import TestClient
from app.main import app


def test_regex_simple_match(client):
    """测试简单的正则表达式匹配"""
    response = client.post(
        "/api/v1/tools/regex/test",
        json={"pattern": r"\d+", "text": "abc123def456"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["match_count"] == 2
    assert "123" in data["matches"]
    assert "456" in data["matches"]


def test_regex_email_pattern(client):
    """测试邮箱正则表达式"""
    response = client.post(
        "/api/v1/tools/regex/test",
        json={"pattern": r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}",
              "text": "Contact us at test@example.com"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["match_count"] == 1
    assert "test@example.com" in data["matches"]


def test_regex_no_match(client):
    """测试没有匹配的情况"""
    response = client.post(
        "/api/v1/tools/regex/test",
        json={"pattern": r"\d+", "text": "abcdef"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["match_count"] == 0
    assert len(data["matches"]) == 0


def test_regex_invalid_pattern(client):
    """测试无效的正则表达式"""
    response = client.post(
        "/api/v1/tools/regex/test",
        json={"pattern": "[invalid(", "text": "test"}
    )
    assert response.status_code == 422
