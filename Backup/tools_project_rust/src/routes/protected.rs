use axum::{routing::get, Json, Router};

use crate::core::auth::CurrentUser;
use crate::models::ProtectedDataResponse;

pub fn router() -> Router {
    Router::new()
        .route("/data", get(get_protected_data))
        .route("/admin", get(admin_only))
}

async fn get_protected_data(current_user: CurrentUser) -> Json<ProtectedDataResponse> {
    Json(ProtectedDataResponse {
        success: true,
        message: "这是受保护的数据".to_string(),
        data: serde_json::json!({
            "user": current_user.username,
            "secret_info": "只有认证用户才能看到这些信息"
        }),
    })
}

async fn admin_only(current_user: CurrentUser) -> Json<ProtectedDataResponse> {
    if !current_user.roles.contains(&"admin".to_string()) {
        return Json(ProtectedDataResponse {
            success: false,
            message: "权限不足，仅管理员可访问".to_string(),
            data: serde_json::json!({}),
        });
    }

    Json(ProtectedDataResponse {
        success: true,
        message: "欢迎，管理员！".to_string(),
        data: serde_json::json!({
            "admin_info": "这是管理员专属信息"
        }),
    })
}
