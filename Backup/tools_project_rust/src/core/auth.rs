// ============================================================================
// 认证模块 —— JWT 签发/校验、bcrypt 密码哈希、CurrentUser 提取器
// 职责：提供登录令牌创建与解析，以及 axum 请求级的用户身份注入。
// 约定：JWT 算法固定 HS256，签名密钥来自 SETTINGS.secret_key；
//       令牌过期时间默认取全局配置 access_token_expire_minutes。
// ============================================================================
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

/// JWT 载荷：sub=用户名，exp 为过期时间戳（秒）
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub user_id: Option<i64>,
    pub roles: Vec<String>,
    pub exp: i64,
}

/// 当前登录用户：由 JWT 解析而来，可直接作为 axum handler 参数注入
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub username: String,
    pub user_id: Option<i64>,
    pub roles: Vec<String>,
}

/// 校验明文密码与 bcrypt 哈希是否匹配；哈希非法时返回 false 而非报错
pub fn verify_password(plain_password: &str, hashed_password: &str) -> bool {
    verify(plain_password, hashed_password).unwrap_or(false)
}

/// 生成 bcrypt 密码哈希
pub fn get_password_hash(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap_or_default()
}

/// 签发 JWT 访问令牌；expires_minutes 为 None 时使用全局默认过期时间
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
    // 注意：编码失败时静默返回空字符串，调用方应检查 token 非空
    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(SETTINGS.secret_key.as_bytes()),
    )
    .unwrap_or_default()
}

/// 解码并校验 JWT；过期或签名不符时返回 None
pub fn decode_access_token(token: &str) -> Option<Claims> {
    let mut validation = Validation::new(Algorithm::HS256);
    // 不校验 aud：本系统未使用受众声明，保持默认会导致校验失败
    validation.validate_aud = false;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SETTINGS.secret_key.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .ok()
}

/// axum 提取器实现：从 Authorization: Bearer 头解析并校验 JWT，失败返回 401
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


