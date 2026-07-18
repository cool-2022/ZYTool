mod core;
mod models;
mod routes;
mod services;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, warn};

use crate::core::config::SETTINGS;
use crate::core::db::init_pool;
use crate::core::middleware::{base_info_middleware, request_logging_middleware};
use crate::models::{BaseInfo, BaseResponse};
use crate::routes::{api_router, auth::init_user_cache};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&SETTINGS.log_level)),
        )
        .init();

    info!("Starting {}", SETTINGS.app_name);
    info!("Version {}", SETTINGS.app_version);

    match init_pool().await {
        Ok(_) => info!("Database connection pool initialized"),
        Err(e) => warn!("Database not connected: {}. Server will continue without DB.", e),
    }

    init_user_cache().await;

    let app = create_app();

    let addr: SocketAddr = format!("{}:{}", SETTINGS.host, SETTINGS.port)
        .parse()
        .expect("Invalid bind address");

    info!("Server listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Server error");
}

fn create_app() -> Router {
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::list(
            SETTINGS
                .cors_origins
                .iter()
                .map(|origin| origin.parse().expect("Invalid CORS origin"))
                .collect::<Vec<_>>(),
        ))
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(root_health_check))
        .nest("/api/v1", api_router())
        .layer(axum::middleware::from_fn(base_info_middleware))
        .layer(axum::middleware::from_fn(request_logging_middleware))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .fallback(handler_404)
}

async fn root_health_check(
    axum::Extension(base): axum::Extension<Option<BaseInfo>>,
) -> Json<BaseResponse<serde_json::Value>> {
    Json(BaseResponse::ok(
        serde_json::json!({
            "status": "ok",
            "version": SETTINGS.app_version
        }),
        base,
    ))
}

async fn handler_404(axum::Extension(base): axum::Extension<Option<BaseInfo>>) -> Response {
    let body = BaseResponse::err("资源未找到", base);
    (StatusCode::NOT_FOUND, Json(body)).into_response()
}
