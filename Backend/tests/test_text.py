import pytest
from fastapi.testclient import TestClient
from app.main import app


def test_text_process_json_format(client):
    """测试JSON格式化"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": '{"name":"test","value":123}', "action": "json_format"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert "result" in data


def test_text_process_base64_encode(client):
    """测试Base64编码"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": "Hello World", "action": "base64_encode"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert "result" in data


def test_text_process_base64_decode(client):
    """测试Base64解码"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": "SGVsbG8gV29ybGQ=", "action": "base64_decode"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["result"] == "Hello World"


def test_text_process_url_encode(client):
    """测试URL编码"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": "hello world", "action": "url_encode"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert "result" in data


def test_text_process_url_decode(client):
    """测试URL解码"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": "hello%20world", "action": "url_decode"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["result"] == "hello world"


def test_text_process_invalid_action(client):
    """测试无效的操作类型"""
    response = client.post(
        "/api/v1/tools/text/process",
        json={"text": "test", "action": "invalid_action"}
    )
    assert response.status_code == 422


def test_text_compare(client):
    """测试文本比较"""
    response = client.post(
        "/api/v1/tools/text/compare",
        json={"text1": "line1\nline2", "text2": "line1\nline3"}
    )
    assert response.status_code == 200
    data = response.json()
    assert "differences" in data
    assert "summary" in data
    assert data["summary"]["total_lines"] == 2
