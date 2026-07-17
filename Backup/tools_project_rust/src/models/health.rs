use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthCheckResponse {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthInfoResponse {
    pub status: String,
    pub version: String,
    pub name: String,
    pub description: String,
    pub rust_version: String,
    pub timestamp: String,
}
