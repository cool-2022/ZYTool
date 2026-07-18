use axum::{
    extract::ConnectInfo,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::headers::UserAgent;
use axum_extra::TypedHeader;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;
use sqlx::Row;
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::RwLock;

use crate::core::auth::{create_access_token, get_password_hash, verify_password, CurrentUser};
use crate::core::config::SETTINGS;
use crate::core::db::{get_pool, DbPool};
use crate::core::error::{bad_request, unauthorized, AppError, AppResult};
use crate::models::{
    BaseInfo, BaseResponse, BindRequest, BindingInfoResponse, LoginRequest, ProviderInfo,
    RegisterRequest, TokenResponse, UserInfoResponse,
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
        .route("/bindings", get(get_bindings))
        .route("/bind/phone", post(bind_phone))
        .route("/bind/qq", post(bind_qq))
        .route("/bind/wechat", post(bind_wechat))
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
    sqlx::query_as::<_, CachedUser>("SELECT id, nickname, password_hash, roles, status FROM users")
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
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<BaseResponse<TokenResponse>>> {
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

    Ok(Json(BaseResponse::ok_with_message(
        TokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: SETTINGS.access_token_expire_minutes * 60,
        },
        "登录成功",
        base,
    )))
}

async fn register(
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<BaseResponse<TokenResponse>>> {
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

    Ok(Json(BaseResponse::ok_with_message(
        TokenResponse {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: SETTINGS.access_token_expire_minutes * 60,
        },
        "注册成功",
        base,
    )))
}

async fn me(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
) -> Json<BaseResponse<UserInfoResponse>> {
    Json(BaseResponse::ok(
        UserInfoResponse {
            username: current_user.username,
            user_id: current_user.user_id,
            roles: current_user.roles,
        },
        base,
    ))
}

async fn logout(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
) -> Json<BaseResponse<serde_json::Value>> {
    Json(BaseResponse::ok_with_message(
        json!({ "user": current_user.username }),
        "登出成功",
        base,
    ))
}

// ===================== 账号绑定 =====================

async fn get_bindings(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
) -> AppResult<Json<BaseResponse<BindingInfoResponse>>> {
    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    let row = sqlx::query("SELECT phone, phone_verified, email FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .map_err(|e| internal_error(format!("查询用户信息失败: {}", e)))?;

    let phone: Option<String> = row.try_get("phone").unwrap_or(None);
    let phone_verified: bool = row.try_get("phone_verified").unwrap_or(false);
    let email: Option<String> = row.try_get("email").unwrap_or(None);

    let providers = sqlx::query_as::<_, ProviderInfo>(
        "SELECT provider, open_id, union_id, nickname FROM user_auths WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询第三方绑定失败: {}", e)))?;

    Ok(Json(BaseResponse::ok(
        BindingInfoResponse {
            phone,
            phone_verified,
            email,
            providers,
        },
        base,
    )))
}

async fn bind_phone(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
    Json(req): Json<BindRequest>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let phone = req.phone.ok_or_else(|| bad_request("手机号不能为空"))?;
    if !is_valid_phone(&phone) {
        return Err(bad_request("手机号格式不正确"));
    }

    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    sqlx::query("UPDATE users SET phone = $1, phone_verified = TRUE WHERE id = $2")
        .bind(&phone)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| internal_error(format!("绑定手机号失败: {}", e)))?;

    Ok(Json(BaseResponse::ok_with_message(
        json!({}),
        "手机号绑定成功",
        base,
    )))
}

async fn bind_qq(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
    Json(req): Json<BindRequest>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let open_id = req.open_id.ok_or_else(|| bad_request("QQ openid 不能为空"))?;
    let nickname = req.nickname.unwrap_or_default();
    bind_third_party(
        base,
        current_user,
        "qq",
        &open_id,
        None,
        &nickname,
        "QQ 绑定成功",
    )
    .await
}

async fn bind_wechat(
    Extension(base): Extension<Option<BaseInfo>>,
    current_user: CurrentUser,
    Json(req): Json<BindRequest>,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let open_id = req.open_id.ok_or_else(|| bad_request("微信 openid 不能为空"))?;
    let nickname = req.nickname.unwrap_or_default();
    bind_third_party(
        base,
        current_user,
        "wechat",
        &open_id,
        req.union_id.as_deref(),
        &nickname,
        "微信绑定成功",
    )
    .await
}

async fn bind_third_party(
    base: Option<BaseInfo>,
    current_user: CurrentUser,
    provider: &str,
    open_id: &str,
    union_id: Option<&str>,
    nickname: &str,
    success_message: &str,
) -> AppResult<Json<BaseResponse<serde_json::Value>>> {
    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;
    let user_id = current_user.user_id.ok_or_else(|| unauthorized("无效的用户信息"))?;

    // 检查该 open_id 是否已被其他用户绑定
    let existing = sqlx::query(
        "SELECT user_id FROM user_auths WHERE provider = $1 AND open_id = $2",
    )
    .bind(provider)
    .bind(open_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("查询第三方绑定失败: {}", e)))?;

    if let Some(row) = existing {
        let bound_user_id: i64 = row
            .try_get("user_id")
            .map_err(|e| internal_error(format!("读取绑定用户失败: {}", e)))?;
        if bound_user_id != user_id {
            return Err(bad_request("该账号已被其他用户绑定"));
        }

        // 已绑定到当前用户，执行更新
        sqlx::query(
            "UPDATE user_auths SET union_id = $1, nickname = $2, updated_at = NOW() WHERE provider = $3 AND open_id = $4",
        )
        .bind(union_id)
        .bind(nickname)
        .bind(provider)
        .bind(open_id)
        .execute(pool)
        .await
        .map_err(|e| internal_error(format!("更新第三方绑定失败: {}", e)))?;
    } else {
        sqlx::query(
            r#"
            INSERT INTO user_auths (user_id, provider, open_id, union_id, nickname)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(user_id)
        .bind(provider)
        .bind(open_id)
        .bind(union_id)
        .bind(nickname)
        .execute(pool)
        .await
        .map_err(|e| internal_error(format!("写入第三方绑定失败: {}", e)))?;
    }

    Ok(Json(BaseResponse::ok_with_message(
        json!({}),
        success_message,
        base,
    )))
}

fn is_valid_phone(phone: &str) -> bool {
    Regex::new(r"^1[3-9]\d{9}$").map(|re| re.is_match(phone)).unwrap_or(false)
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
