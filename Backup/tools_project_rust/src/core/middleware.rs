// ============================================================================
// 中间件模块 —— 请求日志与前端基础信息解析
// 职责：记录请求/响应耗时日志（4xx/5xx 升级为 warn），解析 X-Base-Info 头注入扩展上下文。
// 约定：在 main.rs 中以 from_fn 方式全局挂载，对所有路由生效。
// ============================================================================
use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::net::SocketAddr;
use std::time::Instant;
use tracing::{info, warn};

use crate::core::config::SETTINGS;
use crate::models::{BaseInfo, BaseResponse};

/// 请求日志中间件：记录方法与路径，响应头附加 X-Process-Time，错误状态用 warn 级别
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

/// 公开工具接口 IP 限流：按「IP + 分钟桶」固定窗口计数，超限返回 429；
/// 无 Redis 或 Redis 出错时放行（rate_limit_hit 内部已降级）
pub async fn tools_rate_limit_middleware(req: Request<Body>, next: Next) -> Response {
    let ip = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|c| c.0.ip().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let key = format!(
        "rl:tools:{}:{}",
        ip,
        chrono::Utc::now().format("%Y%m%d%H%M")
    );

    if !crate::core::redis::rate_limit_hit(&key, SETTINGS.tools_rate_limit_per_minute, 70).await {
        let body: BaseResponse<serde_json::Value> =
            BaseResponse::err("请求过于频繁，请稍后再试", None);
        return (StatusCode::TOO_MANY_REQUESTS, Json(body)).into_response();
    }

    next.run(req).await
}

/// tower 服务错误的兜底处理（当前未挂载，保留备用）
#[allow(dead_code)]
pub async fn handle_error(err: tower::BoxError) -> (StatusCode, String) {
    warn!("Unhandled error: {}", err);
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Unhandled error: {}", err),
    )
}
