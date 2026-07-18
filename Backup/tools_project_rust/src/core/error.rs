use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::models::BaseResponse;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub status_code: StatusCode,
    pub details: serde_json::Value,
}

impl AppError {
    pub fn new(message: impl Into<String>, status_code: StatusCode) -> Self {
        Self {
            message: message.into(),
            status_code,
            details: json!({}),
        }
    }

    #[allow(dead_code)]
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}

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

pub type AppResult<T> = Result<T, AppError>;

#[allow(dead_code)]
pub fn validation_error(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::UNPROCESSABLE_ENTITY)
}

pub fn bad_request(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::BAD_REQUEST)
}

pub fn unauthorized(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::UNAUTHORIZED)
}

#[allow(dead_code)]
pub fn not_found(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::NOT_FOUND)
}
