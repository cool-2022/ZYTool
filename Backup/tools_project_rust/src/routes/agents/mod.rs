use axum::{
    routing::{delete, get, post},
    Router,
};

pub mod chat;
pub mod sessions;

pub fn router() -> Router {
    Router::new()
        .route("/sessions", get(sessions::list_sessions).post(sessions::create_session))
        .route(
            "/sessions/{session_id}",
            delete(sessions::delete_session).patch(sessions::update_session_title),
        )
        .route("/sessions/{session_id}/messages", get(sessions::list_messages))
        .route("/chat", post(chat::chat))
        .route("/chat/sync", post(chat::chat_sync))
}
