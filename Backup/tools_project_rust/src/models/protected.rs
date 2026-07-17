use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProtectedDataResponse {
    pub success: bool,
    pub message: String,
    pub data: serde_json::Value,
}
