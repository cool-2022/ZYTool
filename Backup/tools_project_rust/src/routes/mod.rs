use axum::Router;

pub mod admin;
pub mod agents;
pub mod auth;
pub mod health;
pub mod protected;
pub mod tools;

pub fn api_router() -> Router {
    Router::new()
        .nest("/admin", admin::router())
        .nest("/agents", agents::router())
        .nest("/auth", auth::router())
        .nest("/health", health::router())
        .nest("/protected", protected::router())
        .nest("/tools", tools::router())
}
