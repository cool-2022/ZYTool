use axum::{routing::post, Json, Router};

use crate::core::error::AppResult;
use crate::models::{
    BaseResponse, TextCompareRequest, TextCompareResponse, TextProcessRequest,
    TextProcessResponse,
};
use crate::services::text;

pub fn router() -> Router {
    Router::new()
        .route("/process", post(process_text))
        .route("/compare", post(compare_text))
}

async fn process_text(Json(req): Json<TextProcessRequest>) -> AppResult<Json<TextProcessResponse>> {
    let result = text::process_text(&req.action, &req.text)?;
    Ok(Json(TextProcessResponse {
        base: BaseResponse {
            success: true,
            message: None,
        },
        result,
    }))
}

async fn compare_text(Json(req): Json<TextCompareRequest>) -> Json<TextCompareResponse> {
    let (differences, summary) = text::compare_text(&req.text1, &req.text2);
    Json(TextCompareResponse {
        differences,
        summary,
    })
}
