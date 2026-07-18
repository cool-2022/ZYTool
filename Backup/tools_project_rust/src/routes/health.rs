use axum::{routing::get, Extension, Json, Router};
use chrono::Local;
use serde_json::json;

use crate::core::config::SETTINGS;
use crate::core::db;
use crate::models::{BaseInfo, BaseResponse, HealthCheckResponse, HealthInfoResponse};

pub fn router() -> Router {
    Router::new()
        .route("/", get(health_check))
        .route("/info", get(health_info))
        .route("/db", get(db_health_check))
}

async fn health_check(
    Extension(base): Extension<Option<BaseInfo>>,
) -> Json<BaseResponse<HealthCheckResponse>> {
    Json(BaseResponse::ok(
        HealthCheckResponse {
            status: "healthy".to_string(),
            version: SETTINGS.app_version.clone(),
            timestamp: Local::now().to_rfc3339(),
        },
        base,
    ))
}

async fn health_info(
    Extension(base): Extension<Option<BaseInfo>>,
) -> Json<BaseResponse<HealthInfoResponse>> {
    Json(BaseResponse::ok(
        HealthInfoResponse {
            status: "healthy".to_string(),
            version: SETTINGS.app_version.clone(),
            name: SETTINGS.app_name.clone(),
            description: SETTINGS.app_description.clone(),
            rust_version: format!("rustc {}", option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown")),
            timestamp: Local::now().to_rfc3339(),
        },
        base,
    ))
}

async fn db_health_check(
    Extension(base): Extension<Option<BaseInfo>>,
) -> Json<BaseResponse<serde_json::Value>> {
    match db::get_pool() {
        Some(pool) => match db::ping(pool).await {
            Ok(_) => Json(BaseResponse::ok_with_message(
                json!({ "status": "connected" }),
                "数据库连接正常",
                base,
            )),
            Err(e) => Json(BaseResponse::err(
                format!("数据库连接异常: {}", e),
                base,
            )),
        },
        None => Json(BaseResponse::err(
            "数据库连接池未初始化",
            base,
        )),
    }
}
