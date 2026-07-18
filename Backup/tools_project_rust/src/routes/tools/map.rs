use axum::{routing::post, Extension, Json, Router};
use serde_json::json;

use crate::core::error::AppResult;
use crate::models::{BaseInfo, BaseResponse, RouteRequest, RouteResponse};
use crate::services::map;

pub fn router() -> Router {
    Router::new()
        .route("/route", post(get_route))
        .route("/route/test", axum::routing::get(test_route))
}

async fn get_route(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<RouteRequest>,
) -> AppResult<Json<BaseResponse<RouteResponse>>> {
    Ok(Json(BaseResponse::ok(map::generate_mock_route(&req), base)))
}

async fn test_route(
    Extension(base): Extension<Option<BaseInfo>>,
) -> Json<BaseResponse<serde_json::Value>> {
    Json(BaseResponse::ok(
        json!({ "message": "Route API is working" }),
        base,
    ))
}
