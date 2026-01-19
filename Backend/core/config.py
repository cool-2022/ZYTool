import os
from typing import List, Optional
from pydantic_settings import BaseSettings, SettingsConfigDict
from pydantic import Field


class Settings(BaseSettings):
    app_name: str = "ZYTool Backend API"
    app_version: str = "1.0.0"
    app_description: str = "后端工具集API，提供文本处理、正则表达式、密码生成等功能"

    debug: bool = Field(default=False, env="DEBUG")

    host: str = Field(default="0.0.0.0", env="HOST")
    port: int = Field(default=8000, env="PORT")

    cors_origins: List[str] = Field(
        default=["http://localhost:5173", "http://localhost:3000"],
        env="CORS_ORIGINS"
    )

    log_level: str = Field(default="INFO", env="LOG_LEVEL")
    log_file: Optional[str] = Field(default="app.log", env="LOG_FILE")
    log_dir: str = Field(default="logs", env="LOG_DIR")

    max_request_size: int = Field(
        default=10 * 1024 * 1024, env="MAX_REQUEST_SIZE")

    rate_limit_enabled: bool = Field(default=True, env="RATE_LIMIT_ENABLED")
    rate_limit_requests: int = Field(default=100, env="RATE_LIMIT_REQUESTS")
    rate_limit_period: int = Field(default=60, env="RATE_LIMIT_PERIOD")

    # JWT 配置
    secret_key: str = Field(
        default="your-secret-key-change-this-in-production-09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7",
        env="SECRET_KEY"
    )
    algorithm: str = Field(default="HS256", env="ALGORITHM")
    access_token_expire_minutes: int = Field(default=30, env="ACCESS_TOKEN_EXPIRE_MINUTES")

    model_config = SettingsConfigDict(
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False
    )


settings = Settings()
