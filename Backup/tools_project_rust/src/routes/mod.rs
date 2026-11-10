// ============================================================================
// 路由聚合模块 —— /api/v1 下所有业务子路由的统一挂载点
// 职责：声明并组装 admin/agents/auth/health/protected/tools 各业务路由。
// 约定：新增业务模块时，在此 pub mod 声明并在 api_router() 中 nest 挂载。
// ============================================================================
use axum::Router;

pub mod admin;
pub mod agents;
pub mod auth;
pub mod health;
pub mod protected;
pub mod tools;

/// 构建 /api/v1 根路由，统一 nest 各业务子路由
pub fn api_router() -> Router {
    Router::new()
        .nest("/admin", admin::router())
        .nest("/agents", agents::router())
        .nest("/auth", auth::router())
        .nest("/health", health::router())
        .nest("/protected", protected::router())
        .nest("/tools", tools::router())
}
