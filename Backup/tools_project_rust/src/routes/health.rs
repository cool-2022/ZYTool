use axum::{routing::get, Json, Router};
use chrono::Local;

use crate::core::config::SETTINGS;
use crate::core::db;
use crate::models::{HealthCheckResponse, HealthInfoResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/info", get(health_info))
        .route("/db", get(db_health_check))
}

async fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "healthy".to_string(),
        version: SETTINGS.app_version.clone(),
        timestamp: Local::now().to_rfc3339(),
    })
}

async fn health_info() -> Json<HealthInfoResponse> {
    Json(HealthInfoResponse {
        status: "healthy".to_string(),
        version: SETTINGS.app_version.clone(),
        name: SETTINGS.app_name.clone(),
        description: SETTINGS.app_description.clone(),
        rust_version: format!("rustc {}", option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown")),
        timestamp: Local::now().to_rfc3339(),
    })
}

async fn db_health_check() -> Json<serde_json::Value> {
    match db::get_pool() {
        Some(pool) => match db::ping(pool).await {
            Ok(_) => Json(serde_json::json!({
                "success": true,
                "status": "connected",
                "message": "数据库连接正常"
            })),
            Err(e) => Json(serde_json::json!({
                "success": false,
                "status": "error",
                "message": format!("数据库连接异常: {}", e)
            })),
        },
        None => Json(serde_json::json!({
            "success": false,
            "status": "not_initialized",
            "message": "数据库连接池未初始化"
        })),
    }
}
