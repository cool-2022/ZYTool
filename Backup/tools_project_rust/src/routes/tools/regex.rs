use axum::{routing::post, Json, Router};

use crate::core::error::AppResult;
use crate::models::{BaseResponse, RegexTestRequest, RegexTestResponse};
use crate::services::regex;

pub fn router() -> Router {
    Router::new().route("/test", post(test_regex))
}

async fn test_regex(Json(req): Json<RegexTestRequest>) -> AppResult<Json<RegexTestResponse>> {
    let (matches, match_details) = regex::test_regex(&req.pattern, &req.text, req.flags.as_deref())?;

    Ok(Json(RegexTestResponse {
        base: BaseResponse {
            success: true,
            message: None,
        },
        matches,
        match_count: match_details.len(),
        match_details,
    }))
}
