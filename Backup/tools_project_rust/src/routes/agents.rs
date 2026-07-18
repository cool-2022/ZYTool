use axum::{
    response::sse::{Event, Sse},
    routing::post,
    Extension, Json, Router,
};
use futures::stream::{self, Stream};
use std::convert::Infallible;
use std::time::Duration;

use crate::core::error::{bad_request, AppResult};
use crate::models::{BaseInfo, BaseResponse, ChatRequest, ChatResponse};

pub fn router() -> Router {
    Router::new()
        .route("/chat", post(chat))
        .route("/chat/sync", post(chat_sync))
}

async fn chat(Json(req): Json<ChatRequest>) -> AppResult<Sse<impl Stream<Item = Result<Event, Infallible>>>> {
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    let message = req.message.clone();
    let stream = stream::iter(vec![
        Ok(Event::default().data(format!("AI 助手占位回复: {}", message))),
        Ok(Event::default().data("[DONE]")),
    ]);

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive"),
    ))
}

async fn chat_sync(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<ChatRequest>,
) -> AppResult<Json<BaseResponse<ChatResponse>>> {
    if req.message.trim().is_empty() {
        return Err(bad_request("message is required"));
    }

    Ok(Json(BaseResponse::ok(
        ChatResponse {
            reply: format!("AI 助手占位回复: {}", req.message),
        },
        base,
    )))
}
