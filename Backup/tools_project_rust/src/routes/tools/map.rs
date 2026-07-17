use axum::{routing::post, Json, Router};

use crate::core::error::AppResult;
use crate::models::{RouteRequest, RouteResponse};
use crate::services::map;

pub fn router() -> Router {
    Router::new()
        .route("/route", post(get_route))
        .route("/route/test", axum::routing::get(test_route))
}

async fn get_route(Json(req): Json<RouteRequest>) -> AppResult<Json<RouteResponse>> {
    Ok(Json(map::generate_mock_route(&req)))
}

async fn test_route() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "message": "Route API is working"
    }))
}
