use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 前端传入的基础信息，后端只负责透传。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BaseInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_name: Option<String>,
}

/// 统一 API 响应结构：base（基础信息）+ data（业务数据）。
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<BaseInfo>,
    pub data: T,
}

impl<T> BaseResponse<T> {
    pub fn ok(data: T, base: Option<BaseInfo>) -> Self {
        Self {
            success: true,
            message: None,
            base,
            data,
        }
    }

    pub fn ok_with_message(data: T, message: impl Into<String>, base: Option<BaseInfo>) -> Self {
        Self {
            success: true,
            message: Some(message.into()),
            base,
            data,
        }
    }
}

impl BaseResponse<Value> {
    pub fn err(message: impl Into<String>, base: Option<BaseInfo>) -> Self {
        Self {
            success: false,
            message: Some(message.into()),
            base,
            data: serde_json::json!({}),
        }
    }
}
