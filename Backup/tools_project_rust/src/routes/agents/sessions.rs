use axum::{extract::Path, Extension, Json};

use crate::core::auth::CurrentUser;
use crate::core::db::{get_pool, DbPool};
use crate::core::error::{bad_request, internal_error, unauthorized, AppResult};
use crate::models::{
    BaseInfo, BaseResponse, CreateSessionRequest, MessagesResponse, SessionResponse,
    SessionsResponse, UpdateSessionTitleRequest,
};
use crate::services::agents::store;

pub fn require_pool() -> AppResult<&'static DbPool> {
    get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))
}

pub async fn list_sessions(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<SessionsResponse>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    let sessions = store::list_sessions(pool, user_id).await?;

    Ok(Json(BaseResponse::ok(SessionsResponse { sessions }, base)))
}

pub async fn create_session(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<CreateSessionRequest>,
) -> AppResult<Json<BaseResponse<SessionResponse>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    let title = req.title.unwrap_or_else(|| "新对话".to_string());

    if title.trim().is_empty() {
        return Err(bad_request("会话标题不能为空"));
    }

    let session = store::create_session(pool, user_id, title, req.model_id).await?;

    Ok(Json(BaseResponse::ok_with_message(
        session,
        "会话创建成功",
        base,
    )))
}

pub async fn delete_session(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(session_id): Path<String>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    store::delete_session(pool, user_id, &session_id).await?;

    Ok(Json(BaseResponse::ok_with_message(
        serde_json::json!({}),
        "会话删除成功",
        base,
    )))
}

pub async fn update_session_title(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(session_id): Path<String>,
    Json(req): Json<UpdateSessionTitleRequest>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    if req.title.trim().is_empty() {
        return Err(bad_request("会话标题不能为空"));
    }

    store::update_session_title(pool, user_id, &session_id, req.title).await?;

    Ok(Json(BaseResponse::ok_with_message(
        serde_json::json!({}),
        "标题更新成功",
        base,
    )))
}

pub async fn list_messages(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(session_id): Path<String>,
) -> AppResult<Json<BaseResponse<MessagesResponse>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    let messages = store::list_messages(pool, user_id, &session_id).await?;

    Ok(Json(BaseResponse::ok(MessagesResponse { messages }, base)))
}
