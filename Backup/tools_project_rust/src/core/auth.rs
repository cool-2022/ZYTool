use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::core::config::SETTINGS;
use crate::core::error::{unauthorized, AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Option<i64>,
    pub roles: Vec<String>,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub username: String,
    pub user_id: Option<i64>,
    pub roles: Vec<String>,
}

pub fn verify_password(plain_password: &str, hashed_password: &str) -> bool {
    verify(plain_password, hashed_password).unwrap_or(false)
}

pub fn get_password_hash(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_default()
}

pub fn create_access_token(data: CurrentUser, expires_minutes: Option<i64>) -> String {
    let exp_minutes = expires_minutes.unwrap_or(SETTINGS.access_token_expire_minutes);
    let exp = (Utc::now() + Duration::minutes(exp_minutes)).timestamp();
    let claims = Claims {
        sub: data.username.clone(),
        user_id: data.user_id,
        roles: data.roles.clone(),
        exp,
    };
    let header = Header::new(Algorithm::HS256);
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(SETTINGS.secret_key.as_bytes()),
    )
    .unwrap_or_default()
}

pub fn decode_access_token(token: &str) -> Option<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SETTINGS.secret_key.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .ok()
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let TypedHeader(Authorization(bearer)) = parts
                .extract::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|_| unauthorized("无法验证凭证"))?;

            let claims = decode_access_token(bearer.token())
                .ok_or_else(|| unauthorized("无效或过期的 token"))?;

            Ok(CurrentUser {
                username: claims.sub,
                user_id: claims.user_id,
                roles: claims.roles,
            })
        }
    }
}


