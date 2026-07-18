use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};

use crate::models::BaseInfo;

pub async fn request_logging_middleware(req: Request<Body>, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    info!("Request started: {} {}", method, path);

    let mut response = next.run(req).await;
    let duration = start.elapsed();
    let status = response.status();

    response
        .headers_mut()
        .insert("X-Process-Time", format!("{:.4}", duration.as_secs_f64()).parse().unwrap());

    if status.is_server_error() || status.is_client_error() {
        warn!(
            "Request completed: {} {} - Status: {} - Time: {:.4}s",
            method,
            path,
            status.as_u16(),
            duration.as_secs_f64()
        );
    } else {
        info!(
            "Request completed: {} {} - Status: {} - Time: {:.4}s",
            method,
            path,
            status.as_u16(),
            duration.as_secs_f64()
        );
    }

    response
}

/// 从请求头 `X-Base-Info` 中解析前端传入的基础信息，放入扩展上下文供后续使用。
pub async fn base_info_middleware(mut req: Request<Body>, next: Next) -> Response {
    let base_info: Option<BaseInfo> = req
        .headers()
        .get("X-Base-Info")
        .and_then(|value| {
            // HTTP header 值可能是 UTF-8（含中文），使用 to_bytes 而非 to_str
            std::str::from_utf8(value.as_bytes()).ok()
        })
        .and_then(|value| {
            serde_json::from_str::<BaseInfo>(value)
                .map_err(|e| {
                    warn!("X-Base-Info 解析失败: {}", e);
                })
                .ok()
        });

    req.extensions_mut().insert(base_info);
    next.run(req).await
}

#[allow(dead_code)]
pub async fn handle_error(err: tower::BoxError) -> (StatusCode, String) {
    warn!("Unhandled error: {}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled error: {}", err),
    )
}
