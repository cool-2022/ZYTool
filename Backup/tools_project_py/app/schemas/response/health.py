from pydantic import BaseModel, Field


class HealthCheckResponse(BaseModel):
    """健康检查响应模型"""
    status: str = Field(..., description="服务状态", example="healthy")
    version: str = Field(..., description="应用版本", example="1.0.0")
    timestamp: str = Field(..., description="检查时间戳", example="2024-01-01T12:00:00")


class HealthInfoResponse(BaseModel):
    """详细健康信息响应模型"""
    status: str = Field(..., description="服务状态", example="healthy")
    version: str = Field(..., description="应用版本", example="1.0.0")
    name: str = Field(..., description="应用名称", example="ZYTool API")
    description: str = Field(..., description="应用描述", example="ZYTool Backend API")
    python_version: str = Field(..., description="Python版本", example="3.9.0")
    timestamp: str = Field(..., description="检查时间戳", example="2024-01-01T12:00:00")
