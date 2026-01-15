import pytest
from fastapi.testclient import TestClient
from app.main import app


def test_timestamp_to_datetime(client):
    """测试时间戳转换为日期时间"""
    response = client.post(
        "/api/v1/tools/timestamp/convert",
        json={"timestamp": 1609459200, "action": "to_datetime"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert "result" in data
    assert data["action"] == "to_datetime"


def test_timestamp_to_timestamp(client):
    """测试时间戳转换"""
    response = client.post(
        "/api/v1/tools/timestamp/convert",
        json={"timestamp": 1609459200, "action": "to_timestamp"}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert data["result"] == "1609459200"
    assert data["action"] == "to_timestamp"


def test_timestamp_invalid_value(client):
    """测试无效的时间戳"""
    response = client.post(
        "/api/v1/tools/timestamp/convert",
        json={"timestamp": -1, "action": "to_datetime"}
    )
    assert response.status_code == 422


def test_timestamp_too_large(client):
    """测试过大的时间戳"""
    response = client.post(
        "/api/v1/tools/timestamp/convert",
        json={"timestamp": 99999999999, "action": "to_datetime"}
    )
    assert response.status_code == 422
