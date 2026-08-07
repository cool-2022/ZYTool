use axum::{
    response::sse::{Event, Sse},
    Extension, Json,
};
use futures::stream::{self, Stream, StreamExt};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

use crate::core::auth::CurrentUser;
use crate::core::db::DbPool;
use crate::core::error::{bad_request, unauthorized, AppResult};
use crate::models::{BaseInfo, BaseResponse, ChatRequest, ChatResponse};
use crate::services::agents::{llm, store};

use super::sessions::require_pool;

// #region debug-point helper
fn debug_log(hypothesis_id: &str, location: &str, msg: &str, data: serde_json::Value) {
    let payload = serde_json::json!({
        "sessionId": "ai-chat-network-error",
        "runId": "pre-fix",
        "hypothesisId": hypothesis_id,
        "location": location,
        "msg": format!("[DEBUG] {}", msg),
        "data": data,
        "ts": chrono::Utc::now().timestamp_millis(),
    });
    tokio::spawn(async move {
        let _ = reqwest::Client::new()
            .post("http://127.0.0.1:7777/event")
            .json(&payload)
            .send()
            .await;
    });
}
// #endregion

const HISTORY_LIMIT: i64 = 20;

/// 解析会话：有 session_id 则校验归属，没有则自动创建；返回 (内部 session_id, 新会话 uuid)
async fn resolve_session(
    pool: &DbPool,
    user_id: i64,
    session_id: Option<&str>,
) -> AppResult<(i64, Option<String>)> {
    match session_id {
        Some(sid) if !sid.is_empty() => {
            let id = store::get_session_id_by_uuid(pool, user_id, sid).await?;
            Ok((id, None))
        }
        _ => {
            let session = store::create_session(pool, user_id, "新对话".to_string(), None).await?;
            let id = store::get_session_id_by_uuid(pool, user_id, &session.id).await?;
            Ok((id, Some(session.id)))
        }
    }
}

fn trim_title(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.chars().count() > 20 {
        format!("{}...", trimmed.chars().take(20).collect::<String>())
    } else {
        trimmed.to_string()
    }
}

/// SSE 流式聊天：优先调用 Kimi，未配置时回退占位回复
pub async fn chat(
    current_user: CurrentUser,
    Json(req): Json<ChatRequest>,
) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    // #region debug-point A:chat-handler-start
    debug_log("A", "chat.rs:chat-handler-start", "chat handler invoked", serde_json::json!({ "message_len": req.message.len(), "session_id": req.session_id }));
    // #endregion
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    let (session_id, new_session_uuid) =
        resolve_session(pool, user_id, req.session_id.as_deref()).await?;

    // 保存用户消息
    store::save_user_message(pool, session_id, &req.message, None).await?;
    // #region debug-point D:user-message-saved
    debug_log("D", "chat.rs:user-message-saved", "user message saved", serde_json::json!({ "session_id": session_id }));
    // #endregion

    // 拼接历史上下文（含刚保存的用户消息）
    let history = store::list_history_messages(pool, session_id, HISTORY_LIMIT).await?;

    // 新会话用首条消息更新标题
    let title_for_update = new_session_uuid.as_ref().map(|_| trim_title(&req.message));

    // 生成回复流：Kimi 或占位
    // #region debug-point A:llm-config-check
    debug_log("A", "chat.rs:llm-config-check", "llm configured check", serde_json::json!({ "configured": llm::is_configured(), "history_len": history.len() }));
    // #endregion
    let reply_stream: std::pin::Pin<Box<dyn Stream<Item = Result<String, crate::core::error::AppError>> + Send>> =
        if llm::is_configured() {
            Box::pin(llm::chat_stream(history).await?)
        } else {
            Box::pin(stream::once(async move {
                Ok(format!("AI 助手占位回复: {}", req.message))
            }))
        };

    // 边转发边累积完整回复
    let accumulated = Arc::new(Mutex::new(String::new()));
    let acc = accumulated.clone();
    let data_stream = reply_stream.then(move |item| {
        let acc = acc.clone();
        async move {
            match item {
                Ok(text) => {
                    acc.lock().await.push_str(&text);
                    Ok(Event::default().data(text))
                }
                Err(e) => Ok(Event::default().data(format!("\n[调用模型失败] {}", e.message))),
            }
        }
    });

    // 流结束后异步保存助手消息并更新统计
    let pool_for_save = pool.clone();
    let done_stream = stream::once(async move {
        let reply = accumulated.lock().await.clone();
        if let (Some(title), Some(sid)) = (title_for_update, new_session_uuid) {
            let _ = store::update_session_title(&pool_for_save, user_id, &sid, title).await;
        }
        if !reply.is_empty() {
            let _ = store::save_assistant_message(&pool_for_save, session_id, &reply, None, 0).await;
            let _ = store::increment_session_stats(&pool_for_save, session_id, 0).await;
        }
        Ok::<Event, Infallible>(Event::default().data("[DONE]"))
    });

    Ok(Sse::new(data_stream.chain(done_stream)).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}

/// 非流式聊天（一次性返回完整回复）
pub async fn chat_sync(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<ChatRequest>,
) -> AppResult<Json<BaseResponse<ChatResponse>>> {
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    let pool = require_pool()?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    let (session_id, new_session_uuid) =
        resolve_session(pool, user_id, req.session_id.as_deref()).await?;

    store::save_user_message(pool, session_id, &req.message, None).await?;
    let history = store::list_history_messages(pool, session_id, HISTORY_LIMIT).await?;

    let (reply, tokens) = if llm::is_configured() {
        llm::chat_complete(history).await?
    } else {
        (format!("AI 助手占位回复: {}", req.message), 0)
    };

    store::save_assistant_message(pool, session_id, &reply, None, tokens).await?;
    store::increment_session_stats(pool, session_id, tokens).await?;

    if let Some(sid) = new_session_uuid {
        let _ = store::update_session_title(pool, user_id, &sid, trim_title(&req.message)).await;
    }

    Ok(Json(BaseResponse::ok(ChatResponse { reply }, base)))
}
