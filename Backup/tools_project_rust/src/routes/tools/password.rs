use axum::{routing::post, Json, Router};

use crate::core::error::AppResult;
use crate::models::{
    BaseResponse, CharacterTypes, PasswordGenerateRequest, PasswordGenerateResponse,
};
use crate::services::password;

pub fn router() -> Router {
    Router::new().route("/generate", post(generate_password))
}

async fn generate_password(
    Json(req): Json<PasswordGenerateRequest>,
) -> AppResult<Json<PasswordGenerateResponse>> {
    let password = password::generate_password(
        req.length,
        req.include_symbols,
        req.include_numbers,
        req.include_uppercase,
        req.include_lowercase,
    )?;

    Ok(Json(PasswordGenerateResponse {
        base: BaseResponse {
            success: true,
            message: None,
        },
        password,
        length: req.length,
        character_types: CharacterTypes {
            lowercase: req.include_lowercase,
            uppercase: req.include_uppercase,
            numbers: req.include_numbers,
            symbols: req.include_symbols,
        },
    }))
}
