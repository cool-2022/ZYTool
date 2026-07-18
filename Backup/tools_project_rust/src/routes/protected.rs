use axum::{routing::get, Extension, Json, Router};
use serde_json::json;

use crate::core::auth::CurrentUser;
use crate::models::{BaseInfo, BaseResponse};

pub fn router() -> Router {
    Router::new()
        .route("/data", get(get_protected_data))
        .route("/admin", get(admin_only))
}

async fn get_protected_data(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
) -> Json<BaseResponse<serde_json::Value>> {
    Json(BaseResponse::ok_with_message(
        json!({
            "user": current_user.username,
            "secret_info": "只有认证用户才能看到这些信息"
        }),
        "这是受保护的数据",
        base,
    ))
}

async fn admin_only(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
) -> Json<BaseResponse<serde_json::Value>> {
    if !current_user.roles.contains(&"admin".to_string()) {
        return Json(BaseResponse::err(
            "权限不足，仅管理员可访问",
            base,
        ));
    }

    Json(BaseResponse::ok_with_message(
        json!({
            "admin_info": "这是管理员专属信息"
        }),
        "欢迎，管理员！",
        base,
    ))
}
