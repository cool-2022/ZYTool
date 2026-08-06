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
    AuthUrlResponse, BaseInfo, BaseResponse, BindRequest, BindingInfoResponse, LoginRequest,
    ProviderInfo, QQLoginRequest, RegisterRequest, TokenResponse, UserInfoResponse,
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
        .route("/qq/auth-url", get(qq_auth_url))
        .route("/qq/login", post(qq_login))
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

// ===================== QQ OAuth 登录 =====================

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .expect("初始化 HTTP 客户端失败")
});

/// 读取 QQ 互联配置；未配置时返回清晰的 500 错误而不是 panic。
fn qq_oauth_config() -> AppResult<(&'static str, &'static str, &'static str)> {
    let app_id = SETTINGS.qq_app_id.as_str();
    let app_key = SETTINGS.qq_app_key.as_str();
    if app_id.is_empty() || app_key.is_empty() {
        return Err(internal_error(
            "QQ OAuth 未配置：请设置 QQ_APP_ID 与 QQ_APP_KEY 环境变量",
        ));
    }
    Ok((app_id, app_key, SETTINGS.qq_redirect_uri.as_str()))
}

/// 生成 QQ 授权链接。
async fn qq_auth_url(
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<AuthUrlResponse>>> {
    let (app_id, _, redirect_uri) = qq_oauth_config()?;
    let state = uuid::Uuid::new_v4().to_string();
    let url = format!(
        "https://graph.qq.com/oauth2.0/authorize?response_type=code&client_id={}&redirect_uri={}&state={}",
        app_id,
        urlencoding::encode(redirect_uri),
        state
    );
    Ok(Json(BaseResponse::ok(AuthUrlResponse { url }, base)))
}

/// QQ 回调登录：code -> access_token -> openid -> 自动注册/登录。
async fn qq_login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    user_agent: Option<TypedHeader<UserAgent>>,
    Extension(base): Extension<Option<BaseInfo>>,
    Json(req): Json<QQLoginRequest>,
) -> AppResult<Json<BaseResponse<TokenResponse>>> {
    if req.code.trim().is_empty() {
        return Err(bad_request("授权码 code 不能为空"));
    }
    let (app_id, app_key, redirect_uri) = qq_oauth_config()?;
    let pool = get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))?;

    let access_token = qq_exchange_token(app_id, app_key, redirect_uri, &req.code).await?;
    let open_id = qq_fetch_openid(&access_token).await?;
    // 昵称获取失败不阻塞登录，记日志后使用空昵称。
    let nickname = match qq_fetch_nickname(&access_token, app_id, &open_id).await {
        Ok(n) => n,
        Err(e) => {
            tracing::warn!("获取 QQ 用户昵称失败: {}", e.message);
            String::new()
        }
    };

    // 1) 已有 qq+openid 绑定：直接取对应用户。
    let bound = sqlx::query("SELECT user_id FROM user_auths WHERE provider = 'qq' AND open_id = $1")
        .bind(&open_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| internal_error(format!("查询 QQ 绑定失败: {}", e)))?;

    let user = if let Some(row) = bound {
        let user_id: i64 = row
            .try_get("user_id")
            .map_err(|e| internal_error(format!("读取绑定用户失败: {}", e)))?;
        sqlx::query_as::<_, CachedUser>(
            "SELECT id, nickname, password_hash, roles, status FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| internal_error(format!("查询用户失败: {}", e)))?
        .ok_or_else(|| internal_error("QQ 绑定的用户不存在"))?
    } else {
        // 2) 未绑定：自动创建用户 + 写入绑定。
        create_qq_user(pool, &open_id, &nickname).await?
    };

    if user.status != 1 {
        return Err(unauthorized("该账号已被禁用"));
    }

    // 写入/刷新内存缓存，保证后续账号密码登录也能命中缓存。
    {
        let mut cache = USER_CACHE.write().await;
        cache.insert(user.nickname.clone(), user.clone());
    }

    // 登录成功：最近登录信息 + 日志异步写入。
    let pool = pool.clone();
    let ip = addr.ip().to_string();
    let ua = user_agent.map(|h| h.to_string()).unwrap_or_default();
    tokio::spawn(async move {
        let _ = update_qq_login_info(&pool, user.id, &ip, &ua).await;
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
        "QQ 登录成功",
        base,
    )))
}

async fn qq_get_text(url: &str) -> AppResult<String> {
    let resp = HTTP_CLIENT
        .get(url)
        .send()
        .await
        .map_err(|e| internal_error(format!("请求 QQ 接口失败: {}", e)))?;
    resp.text()
        .await
        .map_err(|e| internal_error(format!("读取 QQ 接口响应失败: {}", e)))
}

/// 用 code 换取 access_token。QQ 返回 urlencoded 文本而非 JSON。
async fn qq_exchange_token(
    app_id: &str,
    app_key: &str,
    redirect_uri: &str,
    code: &str,
) -> AppResult<String> {
    let url = format!(
        "https://graph.qq.com/oauth2.0/token?grant_type=authorization_code&client_id={}&client_secret={}&code={}&redirect_uri={}",
        app_id,
        app_key,
        code,
        urlencoding::encode(redirect_uri)
    );
    let body = qq_get_text(&url).await?;

    let mut token: Option<String> = None;
    let mut err_desc: Option<String> = None;
    for pair in body.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            match k {
                "access_token" => token = Some(v.to_string()),
                "error_description" => err_desc = Some(v.to_string()),
                _ => {}
            }
        }
    }
    token.ok_or_else(|| {
        internal_error(format!(
            "QQ 换取 access_token 失败: {}",
            err_desc.unwrap_or(body)
        ))
    })
}

/// 用 access_token 获取 openid。QQ 返回 `callback( {...} );` 形式的 JS 回调包裹文本。
async fn qq_fetch_openid(access_token: &str) -> AppResult<String> {
    let url = format!("https://graph.qq.com/oauth2.0/me?access_token={}", access_token);
    let body = qq_get_text(&url).await?;

    let start = body
        .find('(')
        .ok_or_else(|| internal_error(format!("QQ openid 响应格式异常: {}", body)))?;
    let end = body
        .rfind(')')
        .ok_or_else(|| internal_error(format!("QQ openid 响应格式异常: {}", body)))?;
    let inner = body[start + 1..end].trim();
    let value: serde_json::Value = serde_json::from_str(inner)
        .map_err(|e| internal_error(format!("解析 QQ openid 响应失败: {}", e)))?;

    if let Some(desc) = value.get("error_description").and_then(|d| d.as_str()) {
        return Err(internal_error(format!("QQ 获取 openid 失败: {}", desc)));
    }
    value
        .get("openid")
        .and_then(|o| o.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| internal_error("QQ 获取 openid 失败：响应缺少 openid"))
}

/// 用 openid 获取 QQ 用户昵称。
async fn qq_fetch_nickname(access_token: &str, app_id: &str, open_id: &str) -> AppResult<String> {
    let url = format!(
        "https://graph.qq.com/user/get_user_info?access_token={}&oauth_consumer_key={}&openid={}",
        access_token, app_id, open_id
    );
    let body = qq_get_text(&url).await?;
    let value: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| internal_error(format!("解析 QQ 用户信息响应失败: {}", e)))?;

    let ret = value.get("ret").and_then(|r| r.as_i64()).unwrap_or(-1);
    if ret != 0 {
        let msg = value.get("msg").and_then(|m| m.as_str()).unwrap_or("未知错误");
        return Err(internal_error(format!("QQ 获取用户信息失败: {}", msg)));
    }
    Ok(value
        .get("nickname")
        .and_then(|n| n.as_str())
        .unwrap_or_default()
        .to_string())
}

/// 为 QQ 用户自动创建账号（用户名 `qq_{openid 前 16 位}`，随机密码）并写入绑定。
async fn create_qq_user(pool: &DbPool, open_id: &str, nickname: &str) -> AppResult<CachedUser> {
    let password_hash = get_password_hash(&uuid::Uuid::new_v4().to_string());
    let roles = vec!["user".to_string()];
    let prefix: String = open_id.chars().take(16).collect();

    // 用户名冲突时追加随机后缀重试。
    for attempt in 0..5 {
        let username = if attempt == 0 {
            format!("qq_{}", prefix)
        } else {
            format!("qq_{}_{:x}", prefix, rand::random::<u16>())
        };
        let result = sqlx::query(
            r#"
            INSERT INTO users (nickname, password_hash, roles, status)
            VALUES ($1, $2, $3, 1)
            RETURNING id
            "#,
        )
        .bind(&username)
        .bind(&password_hash)
        .bind(&roles)
        .fetch_one(pool)
        .await;

        let row = match result {
            Ok(r) => r,
            Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => continue,
            Err(e) => return Err(internal_error(format!("创建 QQ 用户失败: {}", e))),
        };

        let user_id: i64 = row
            .try_get("id")
            .map_err(|e| internal_error(format!("读取用户 ID 失败: {}", e)))?;

        sqlx::query(
            r#"
            INSERT INTO user_auths (user_id, provider, open_id, union_id, nickname)
            VALUES ($1, 'qq', $2, NULL, $3)
            "#,
        )
        .bind(user_id)
        .bind(open_id)
        .bind(nickname)
        .execute(pool)
        .await
        .map_err(|e| internal_error(format!("写入 QQ 绑定失败: {}", e)))?;

        return Ok(CachedUser {
            id: user_id,
            nickname: username,
            password_hash,
            roles,
            status: 1,
        });
    }
    Err(internal_error("创建 QQ 用户失败：用户名冲突"))
}

async fn update_qq_login_info(
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
        VALUES ($1, 'qq', $2::inet, $3, '', TRUE, '')
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
