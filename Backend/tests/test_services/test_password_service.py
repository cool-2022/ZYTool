import pytest
from services.password_service import PasswordService


def test_generate_password_default():
    """测试生成默认密码"""
    password = PasswordService.generate_password(12, True, True, True, True)
    assert len(password) == 12


def test_generate_password_custom_length():
    """测试生成自定义长度密码"""
    password = PasswordService.generate_password(20, True, True, True, True)
    assert len(password) == 20


def test_generate_password_only_lowercase():
    """测试只包含小写字母的密码"""
    password = PasswordService.generate_password(10, False, False, False, True)
    assert password.islower()
    assert password.isalpha()


def test_generate_password_no_character_types():
    """测试没有选择任何字符类型"""
    with pytest.raises(ValueError):
        PasswordService.generate_password(12, False, False, False, False)
