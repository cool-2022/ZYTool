use axum::{routing::post, Json, Router};

use crate::core::error::AppResult;
use crate::models::{BaseResponse, TimestampConvertRequest, TimestampConvertResponse};
use crate::services::timestamp;

pub fn router() -> Router {
    Router::new().route("/convert", post(convert_timestamp))
}

async fn convert_timestamp(
    Json(req): Json<TimestampConvertRequest>,
) -> AppResult<Json<TimestampConvertResponse>> {
    let (result, action) = timestamp::convert_timestamp(req.timestamp, &req.action)?;

    Ok(Json(TimestampConvertResponse {
        base: BaseResponse {
            success: true,
            message: None,
        },
        result,
        timestamp: req.timestamp,
        action,
    }))
}
