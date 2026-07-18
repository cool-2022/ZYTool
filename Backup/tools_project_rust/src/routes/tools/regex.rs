use axum::{routing::post, Extension, Json, Router};

use crate::core::error::AppResult;
use crate::models::{BaseInfo, BaseResponse, RegexTestRequest, RegexTestResponse};
use crate::services::regex;

pub fn router() -> Router {
    Router::new().route("/test", post(test_regex))
}

async fn test_regex(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<RegexTestRequest>,
) -> AppResult<Json<BaseResponse<RegexTestResponse>>> {
    let (matches, match_details) = regex::test_regex(&req.pattern, &req.text, req.flags.as_deref())?;

    Ok(Json(BaseResponse::ok(
        RegexTestResponse {
            matches,
            match_count: match_details.len(),
            match_details,
        },
        base,
    )))
}
