use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponse {
    pub reply: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSessionTitleRequest {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub id: String,
    pub title: String,
    pub date: String,
    pub message_count: i32,
    pub total_tokens: i32,
    pub model_id: Option<i64>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: String,
    pub role: String,
    pub content: String,
    pub content_type: String,
    pub tokens_used: i32,
    pub model_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionsResponse {
    pub sessions: Vec<SessionResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessagesResponse {
    pub messages: Vec<MessageResponse>,
}
