use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use once_cell::sync::Lazy;
use sqlx::Row;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;

use crate::core::auth::{create_access_token, get_password_hash, verify_password, CurrentUser};
use crate::core::config::SETTINGS;
use crate::core::db::{get_pool, DbPool};
use crate::core::error::{bad_request, unauthorized, AppError, AppResult};
use crate::models::{
    LoginRequest, LoginResponse, RegisterRequest, TokenResponse, UserInfoResponse,
};

/// 内存中的用户热缓存，避免每次登录都走数据库往返。
#[derive(Debug, Clone, sqlx::FromRow)]
struct CachedUser {
    id: i64,
    nickname: String,
    password_hash: String,
    roles: Vec<String>,
    status: i16,
}

static USER_CACHE: Lazy<RwLock<HashMap<String, CachedUser>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

fn internal_error(msg: impl Into<String>) -> AppError {
    AppError::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/me", get(me))
        .route("/logout", post(logout))
}

/// 服务启动时从数据库加载全部用户到内存缓存。
pub async fn init_user_cache() {
    let Some(pool) = get_pool() else {
        tracing::warn!("数据库连接池未初始化，跳过用户缓存加载");
        return;
    };

    match load_all_users(pool).await {
        Ok(users) => {
            let mut cache = USER_CACHE.write().await;
            for user in users {
                cache.insert(user.nickname.clone(), user);
            }
            tracing::info!("用户缓存加载完成，共 {} 人", cache.len());
        }
        Err(e) => {
            tracing::warn!("用户缓存加载失败: {}", e);
        }
    }
}

async fn load_all_users(pool: &DbPool) -> Result<Vec<CachedUser>, sqlx::Error> {
    sqlx::query_as::<_, CachedUser>(
        "SELECT id, nickname, password_hash, roles, status FROM users"
    )
    .fetch_all(pool)
    .await
}

async fn fetch_user_from_db(
    pool: &DbPool,
    username: &str,
) -> Result<Option<CachedUser>, sqlx::Error> {
    sqlx::query_as::<_, CachedUser>(
        r#"
        SELECT id, nickname, password_hash, roles, status
        FROM users
        WHERE nickname = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    user_agent: Option<TypedHeader<UserAgent>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;

    // 1) 优先读内存缓存，命中则完全避免数据库往返。
    let cached_user = {
        let cache = USER_CACHE.read().await;
        cache.get(&req.username).cloned()
    };

    let user = match cached_user {
        Some(u) => u,
        None => fetch_user_from_db(pool, &req.username)
            .await
            .map_err(|e| internal_error(format!("数据库查询失败: {}", e)))?
            .ok_or_else(|| unauthorized("用户名或密码错误"))?,
    };

    if user.status != 1 {
        return Err(unauthorized("用户名或密码错误"));
    }

    if !verify_password(&req.password, &user.password_hash) {
        // 记录失败日志（异步，不阻塞登录响应）
        let pool = pool.clone();
        let ip = addr.ip().to_string();
        let ua = user_agent.map(|h| h.to_string()).unwrap_or_default();
        tokio::spawn(async move {
            let _ = record_login_log(&pool, user.id, &ip, &ua, false, "密码错误").await;
        });
        return Err(unauthorized("用户名或密码错误"));
    }

    // 2) 登录成功：token 立即返回，数据库写操作（最近登录信息 + 日志）异步执行。
    let pool = pool.clone();
    let ip = addr.ip().to_string();
    let ua = user_agent.map(|h| h.to_string()).unwrap_or_default();
    tokio::spawn(async move {
        let _ = update_login_info(&pool, user.id, &ip, &ua).await;
    });

    let token = create_access_token(
        CurrentUser {
            username: user.nickname.clone(),
            user_id: Some(user.id),
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
    if req.username.is_empty() {
        return Err(bad_request("用户名不能为空"));
    }
    if req.password.len() < 6 {
        return Err(bad_request("密码长度不能小于 6 位"));
    }

    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;

    let hashed_password = get_password_hash(&req.password);
    let roles = vec!["user".to_string()];

    let result = sqlx::query(
        r#"
        INSERT INTO users (nickname, password_hash, roles, status)
        VALUES ($1, $2, $3, 1)
        RETURNING id
        "#,
    )
    .bind(&req.username)
    .bind(&hashed_password)
    .bind(&roles)
    .fetch_one(pool)
    .await;

    let row = match result {
        Ok(r) => r,
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            return Err(bad_request("用户名已存在"));
        }
        Err(e) => return Err(internal_error(format!("注册失败: {}", e))),
    };

    let user_id: i64 = row
        .try_get("id")
        .map_err(|e| internal_error(format!("读取用户 ID 失败: {}", e)))?;

    // 新用户同时写入内存缓存，保证注册后立即可快速登录。
    {
        let mut cache = USER_CACHE.write().await;
        cache.insert(
            req.username.clone(),
            CachedUser {
                id: user_id,
                nickname: req.username.clone(),
                password_hash: hashed_password,
                roles: roles.clone(),
                status: 1,
            },
        );
    }

    let token = create_access_token(
        CurrentUser {
            username: req.username.clone(),
            user_id: Some(user_id),
            roles: roles.clone(),
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

async fn update_login_info(
    pool: &DbPool,
    user_id: i64,
    ip: &str,
    user_agent: &str,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query(
        "UPDATE users SET last_login_at = NOW(), last_login_ip = $1::inet WHERE id = $2",
    )
    .bind(ip)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO user_login_logs (user_id, provider, ip, user_agent, device, success, fail_reason)
        VALUES ($1, 'password', $2::inet, $3, '', TRUE, '')
        "#,
    )
    .bind(user_id)
    .bind(ip)
    .bind(user_agent)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}

async fn record_login_log(
    pool: &DbPool,
    user_id: i64,
    ip: &str,
    user_agent: &str,
    success: bool,
    fail_reason: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_login_logs (user_id, provider, ip, user_agent, device, success, fail_reason)
        VALUES ($1, 'password', $2::inet, $3, '', $4, $5)
        "#,
    )
    .bind(user_id)
    .bind(ip)
    .bind(user_agent)
    .bind(success)
    .bind(fail_reason)
    .execute(pool)
    .await?;
    Ok(())
}
