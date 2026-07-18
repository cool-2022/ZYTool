use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    #[serde(default = "default_bearer")]
    pub token_type: String,
    pub expires_in: i64,
}

fn default_bearer() -> String { "bearer".to_string() }

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoResponse {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TokenResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProviderInfo {
    pub provider: String,
    pub open_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub union_id: Option<String>,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindingInfoResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub phone_verified: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub providers: Vec<ProviderInfo>,
}
