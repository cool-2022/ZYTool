use axum::Router;

use crate::core::middleware::tools_rate_limit_middleware;

pub mod map;
pub mod misc;
pub mod password;
pub mod regex;
pub mod text;
pub mod timestamp;

pub fn router() -> Router {
    Router::new()
        .merge(misc::router())
        .nest("/text", text::router())
        .nest("/regex", regex::router())
        .nest("/password", password::router())
        .nest("/timestamp", timestamp::router())
        .nest("/map", map::router())
        // 工具接口全部公开无认证，按 IP 限流防刷（无 Redis 时自动放行）
        .layer(axum::middleware::from_fn(tools_rate_limit_middleware))
}
