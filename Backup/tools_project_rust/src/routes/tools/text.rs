use axum::{routing::post, Extension, Json, Router};

use crate::core::error::AppResult;
use crate::models::{
    BaseInfo, BaseResponse, TextCompareRequest, TextCompareResponse, TextProcessRequest,
    TextProcessResponse,
};
use crate::services::text;

pub fn router() -> Router {
    Router::new()
        .route("/process", post(process_text))
        .route("/compare", post(compare_text))
}

async fn process_text(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<TextProcessRequest>,
) -> AppResult<Json<BaseResponse<TextProcessResponse>>> {
    let result = text::process_text(&req.action, &req.text)?;
    Ok(Json(BaseResponse::ok(
        TextProcessResponse { result },
        base,
    )))
}

async fn compare_text(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<TextCompareRequest>,
) -> Json<BaseResponse<TextCompareResponse>> {
    let (differences, summary) = text::compare_text(&req.text1, &req.text2);
    Json(BaseResponse::ok(
        TextCompareResponse {
            differences,
            summary,
        },
        base,
    ))
}
