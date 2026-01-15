import pytest
from fastapi.testclient import TestClient
from app.main import app


def test_generate_password_default(client):
    """测试生成默认密码"""
    response = client.post(
        "/api/v1/tools/password/generate",
        json={}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert "password" in data
    assert len(data["password"]) == 12


def test_generate_password_custom_length(client):
    """测试生成自定义长度密码"""
    response = client.post(
        "/api/v1/tools/password/generate",
        json={"length": 20}
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    assert len(data["password"]) == 20


def test_generate_password_only_lowercase(client):
    """测试只包含小写字母的密码"""
    response = client.post(
        "/api/v1/tools/password/generate",
        json={
            "length": 10,
            "include_lowercase": True,
            "include_uppercase": False,
            "include_numbers": False,
            "include_symbols": False
        }
    )
    assert response.status_code == 200
    data = response.json()
    assert data["success"] == True
    password = data["password"]
    assert password.islower()
    assert password.isalpha()


def test_generate_password_no_character_types(client):
    """测试没有选择任何字符类型"""
    response = client.post(
        "/api/v1/tools/password/generate",
        json={
            "include_lowercase": False,
            "include_uppercase": False,
            "include_numbers": False,
            "include_symbols": False
        }
    )
    assert response.status_code == 422


def test_generate_password_invalid_length(client):
    """测试无效的密码长度"""
    response = client.post(
        "/api/v1/tools/password/generate",
        json={"length": 3}
    )
    assert response.status_code == 422
