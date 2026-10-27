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

    // 工具子命令：加密 API Key（cargo run -- encrypt-key "sk-xxxx"）
    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 3 && args[1] == "encrypt-key" {
        println!("{}", core::crypto::encrypt(&args[2]));
        return;
    }

    info!("Starting {}", SETTINGS.app_name);
    info!("Version {}", SETTINGS.app_version);

    match init_pool().await {
        Ok(_) => {
            info!("Database connection pool initialized");
            warn_plaintext_api_keys().await;
        }
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
    let allowed_origins = SETTINGS.cors_origins.clone();
    let cors = CorsLayer::new()
        // 放行：配置清单 + localhost/私网网段（192.168.*、10.*、172.16-31.*），IP 变化不影响开发使用
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            move |origin, _| {
                let Ok(origin) = origin.to_str() else { return false };
                if allowed_origins.iter().any(|o| o == origin) {
                    return true;
                }
                is_private_origin(origin)
            },
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

/// 判断是否本机/私网来源（http/https + localhost/127.*/192.168.*/10.*/172.16-31.*）
fn is_private_origin(origin: &str) -> bool {
    let host = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
        .unwrap_or("");
    let host = host.split(':').next().unwrap_or("");

    if host == "localhost" || host.ends_with(".local") {
        return true;
    }
    let parts: Vec<u8> = host.split('.').filter_map(|p| p.parse().ok()).collect();
    if parts.len() != 4 {
        return false;
    }
    match parts[0] {
        127 | 10 => true,
        192 => parts[1] == 168,
        172 => (16..=31).contains(&parts[1]),
        _ => false,
    }
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

/// 启动时检查 ai_models 中是否有明文存储的 API Key
async fn warn_plaintext_api_keys() {
    let Some(pool) = core::db::get_pool() else { return };
    let rows: Vec<(String,)> =
        match sqlx::query_as("SELECT model_name FROM ai_models WHERE api_key_encrypted NOT LIKE 'v1:%' AND api_key_encrypted <> ''")
            .fetch_all(pool)
            .await
        {
            Ok(r) => r,
            Err(_) => return,
        };
    if !rows.is_empty() {
        let names: Vec<String> = rows.into_iter().map(|r| r.0).collect();
        warn!(
            "检测到明文存储的 API Key（{:?}），请用 `cargo run -- encrypt-key <密钥>` 加密后更新",
            names
        );
    }
}
