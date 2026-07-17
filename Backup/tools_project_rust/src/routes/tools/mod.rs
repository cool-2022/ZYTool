use axum::Router;

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
}
