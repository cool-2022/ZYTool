use axum::{routing::post, Json, Router};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::core::auth::{create_access_token, get_password_hash, verify_password, CurrentUser};
use crate::core::config::SETTINGS;
use crate::core::error::{bad_request, unauthorized, AppResult};
use crate::models::{
    LoginRequest, LoginResponse, RegisterRequest, TokenResponse, UserInfoResponse,
};

#[derive(Debug, Clone)]
struct User {
    username: String,
    user_id: i64,
    hashed_password: String,
    roles: Vec<String>,
}

static FAKE_USERS_DB: Lazy<Mutex<HashMap<String, User>>> = Lazy::new(|| {
    let mut db = HashMap::new();
    db.insert(
        "admin".to_string(),
        User {
            username: "admin".to_string(),
            user_id: 1,
            hashed_password: get_password_hash("admin123"),
            roles: vec!["admin".to_string(), "user".to_string()],
        },
    );
    db.insert(
        "user".to_string(),
        User {
            username: "user".to_string(),
            user_id: 2,
            hashed_password: get_password_hash("user123"),
            roles: vec!["user".to_string()],
        },
    );
    Mutex::new(db)
});

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/me", post(me))
        .route("/logout", post(logout))
}

async fn login(Json(req): Json<LoginRequest>) -> AppResult<Json<LoginResponse>> {
    let db = FAKE_USERS_DB.lock().unwrap();
    let user = db
        .get(&req.username)
        .ok_or_else(|| unauthorized("用户名或密码错误"))?;

    if !verify_password(&req.password, &user.hashed_password) {
        return Err(unauthorized("用户名或密码错误"));
    }

    let token = create_access_token(
        CurrentUser {
            username: user.username.clone(),
            user_id: Some(user.user_id),
            roles: user.roles.clone(),
        },
        None,
    );

    Ok(Json(LoginResponse {
        success: true,
        message: "登录成功".to_string(),
        data: Some(TokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: SETTINGS.access_token_expire_minutes * 60,
        }),
    }))
}

async fn register(Json(req): Json<RegisterRequest>) -> AppResult<Json<LoginResponse>> {
    let mut db = FAKE_USERS_DB.lock().unwrap();
    if db.contains_key(&req.username) {
        return Err(bad_request("用户名已存在"));
    }

    let new_user_id = db.len() as i64 + 1;
    let user = User {
        username: req.username.clone(),
        user_id: new_user_id,
        hashed_password: get_password_hash(&req.password),
        roles: vec!["user".to_string()],
    };
    db.insert(req.username.clone(), user.clone());

    let token = create_access_token(
        CurrentUser {
            username: user.username.clone(),
            user_id: Some(user.user_id),
            roles: user.roles.clone(),
        },
        None,
    );

    Ok(Json(LoginResponse {
        success: true,
        message: "注册成功".to_string(),
        data: Some(TokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: SETTINGS.access_token_expire_minutes * 60,
        }),
    }))
}

async fn me(current_user: CurrentUser) -> Json<UserInfoResponse> {
    Json(UserInfoResponse {
        username: current_user.username,
        user_id: current_user.user_id,
        roles: current_user.roles,
    })
}

async fn logout(current_user: CurrentUser) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "success": true,
        "message": "登出成功",
        "user": current_user.username
    }))
}
