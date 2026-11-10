// ============================================================================
// 统一错误模块 —— AppError 与便捷构造函数
// 职责：定义业务错误类型并实现 IntoResponse，保证错误响应同样遵循统一响应结构
//       （BaseResponse：success/message/data/base），与 Python 兜底后端保持一致。
// 约定：handler 内使用 AppResult<T> 作为返回类型，用 ? 传播错误。
// ============================================================================
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::models::BaseResponse;

/// 应用统一错误：HTTP 状态码 + 业务消息 + 可选详情
#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
    pub details: serde_json::Value,
}

impl AppError {
    /// 构造错误（无附加详情）
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status_code,
            details: json!({}),
        }
    }

    /// 附加错误详情（链式调用）
    #[allow(dead_code)]
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}

/// 错误响应序列化：统一包装为 BaseResponse，details/status_code 放入 data 字段
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = BaseResponse {
            success: false,
            message: Some(self.message),
            base: None,
            data: json!({
                "details": self.details,
                "status_code": self.status_code.as_u16()
            }),
        };
        (self.status_code, Json(body)).into_response()
    }
}

/// handler 统一返回类型别名
pub type AppResult<T> = Result<T, AppError>;

/// 参数校验错误（422）
#[allow(dead_code)]
pub fn validation_error(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::UNPROCESSABLE_ENTITY)
}

/// 请求参数错误（400）
pub fn bad_request(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::BAD_REQUEST)
}

/// 未认证/凭证无效（401）
pub fn unauthorized(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::UNAUTHORIZED)
}

/// 资源不存在（404）
#[allow(dead_code)]
pub fn not_found(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::NOT_FOUND)
}

/// 服务器内部错误（500）
pub fn internal_error(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
}
