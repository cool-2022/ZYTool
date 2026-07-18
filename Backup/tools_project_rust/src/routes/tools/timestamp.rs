use axum::{routing::post, Extension, Json, Router};

use crate::core::error::AppResult;
use crate::models::{BaseInfo, BaseResponse, TimestampConvertRequest, TimestampConvertResponse};
use crate::services::timestamp;

pub fn router() -> Router {
    Router::new().route("/convert", post(convert_timestamp))
}

async fn convert_timestamp(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<TimestampConvertRequest>,
) -> AppResult<Json<BaseResponse<TimestampConvertResponse>>> {
    let (result, action) = timestamp::convert_timestamp(req.timestamp, &req.action)?;

    Ok(Json(BaseResponse::ok(
        TimestampConvertResponse {
            result,
            timestamp: req.timestamp,
            action,
        },
        base,
    )))
}
