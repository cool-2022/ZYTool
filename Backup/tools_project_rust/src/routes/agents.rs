use axum::{
    extract::Path,
    response::sse::{Event, Sse},
    routing::{delete, get, post},
    Extension, Json, Router,
};
use futures::stream::{self, Stream};
use futures::StreamExt;
use std::convert::Infallible;
use std::time::Duration;

use crate::core::auth::CurrentUser;
use crate::core::db::get_pool;
use crate::core::error::{bad_request, internal_error, unauthorized, AppResult};
use crate::models::{
    BaseInfo, BaseResponse, ChatRequest, ChatResponse, CreateSessionRequest,
    MessagesResponse, SessionResponse, SessionsResponse, UpdateSessionTitleRequest,
};
use crate::services::agents as ai_service;

pub fn router() -> Router {
    Router::new()
        .route("/sessions", get(list_sessions).post(create_session))
        .route(
            "/sessions/{session_id}",
            delete(delete_session).patch(update_session_title),
        )
        .route("/sessions/{session_id}/messages", get(list_messages))
        .route("/chat", post(chat))
        .route("/chat/sync", post(chat_sync))
}

fn require_pool() -> AppResult<&'static crate::core::db::DbPool> {
    get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))
}

async fn list_sessions(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<SessionsResponse>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    let sessions = ai_service::list_sessions(pool, user_id).await?;

    Ok(Json(BaseResponse::ok(
        SessionsResponse { sessions },
        base,
    )))
}

async fn create_session(
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

    let session = ai_service::create_session(pool, user_id, title, req.model_id).await?;

    Ok(Json(BaseResponse::ok_with_message(
        session,
        "会话创建成功",
        base,
    )))
}

async fn delete_session(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(session_id): Path<String>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    ai_service::delete_session(pool, user_id, &session_id).await?;

    Ok(Json(BaseResponse::ok_with_message(
        serde_json::json!({}),
        "会话删除成功",
        base,
    )))
}

async fn update_session_title(
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

    ai_service::update_session_title(pool, user_id, &session_id, req.title).await?;

    Ok(Json(BaseResponse::ok_with_message(
        serde_json::json!({}),
        "标题更新成功",
        base,
    )))
}

async fn list_messages(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(session_id): Path<String>,
) -> AppResult<Json<BaseResponse<MessagesResponse>>> {
    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;
    let messages = ai_service::list_messages(pool, user_id, &session_id).await?;

    Ok(Json(BaseResponse::ok(
        MessagesResponse { messages },
        base,
    )))
}

async fn chat(
    current_user: CurrentUser,
    Json(req): Json<ChatRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    // 确定会话：有 session_id 就用，没有则自动创建
    let (session_id, session_uuid_for_title) = match req.session_id.as_deref() {
        Some(sid) if !sid.is_empty() => {
            let id = ai_service::get_session_id_by_uuid(pool, user_id, sid).await?;
            (id, None)
        }
        _ => {
            let session = ai_service::create_session(pool, user_id, "新对话".to_string(), None).await?;
            let id = ai_service::get_session_id_by_uuid(pool, user_id, &session.id).await?;
            (id, Some(session.id))
        }
    };

    // 新会话用首条消息更新标题
    let title_for_update = session_uuid_for_title.as_ref().map(|_| trim_title(&req.message));

    // 保存用户消息
    ai_service::save_user_message(pool, session_id, &req.message, None).await?;

    // 生成助手回复（目前为占位回复）
    let reply = format!("AI 助手占位回复: {}", req.message);
    let reply_clone = reply.clone();

    // 在流结束后异步保存助手消息并更新统计
    let pool_for_save = pool.clone();
    let stream = stream::iter(vec![
        Ok(Event::default().data(reply_clone)),
        Ok(Event::default().data("[DONE]")),
    ])
    .chain(stream::once(async move {
        // 新会话更新标题
        if let (Some(title), Some(sid)) = (title_for_update, session_uuid_for_title) {
            let _ = ai_service::update_session_title(&pool_for_save, user_id, &sid, title).await;
        }
        // 保存助手消息
        let _ = ai_service::save_assistant_message(&pool_for_save, session_id, &reply, None, 0).await;
        let _ = ai_service::increment_session_stats(&pool_for_save, session_id, 0).await;
        Ok::<Event, Infallible>(Event::default().data(""))
    }));

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive"),
    ))
}

async fn chat_sync(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<ChatRequest>,
) -> AppResult<Json<BaseResponse<ChatResponse>>> {
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    // 确定会话
    let (session_id, is_new_session) = match req.session_id.as_deref() {
        Some(sid) if !sid.is_empty() => {
            let id = ai_service::get_session_id_by_uuid(pool, user_id, sid).await?;
            (id, false)
        }
        _ => {
            let session = ai_service::create_session(pool, user_id, "新对话".to_string(), None).await?;
            let id = ai_service::get_session_id_by_uuid(pool, user_id, &session.id).await?;
            (id, true)
        }
    };

    // 保存用户消息
    ai_service::save_user_message(pool, session_id, &req.message, None).await?;

    // 生成助手回复（占位）
    let reply = format!("AI 助手占位回复: {}", req.message);

    // 保存助手消息并更新统计
    ai_service::save_assistant_message(pool, session_id, &reply, None, 0).await?;
    ai_service::increment_session_stats(pool, session_id, 0).await?;

    // 新会话用首条消息更新标题
    if is_new_session {
        let title = trim_title(&req.message);
        if let Some(sid) = req.session_id {
            let _ = ai_service::update_session_title(pool, user_id, &sid, title).await;
        }
    }

    Ok(Json(BaseResponse::ok(
        ChatResponse { reply },
        base,
    )))
}

fn trim_title(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.len() > 20 {
        format!("{}...", &trimmed[..20])
    } else {
        trimmed.to_string()
    }
}
