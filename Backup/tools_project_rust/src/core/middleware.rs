use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::{info, warn};

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

#[allow(dead_code)]
pub async fn handle_error(err: tower::BoxError) -> (StatusCode, String) {
    warn!("Unhandled error: {}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled error: {}", err),
    )
}
