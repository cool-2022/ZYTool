// ============================================================================
// 数据库模块 —— PostgreSQL 连接池全局单例
// 职责：构建连接串、初始化并全局持有 sqlx PgPool，提供健康 ping。
// 约定：连接池只在启动时初始化一次；init_pool 失败时服务以无 DB 模式运行，
//       业务代码一律用 get_pool() 判空降级，不得假设池必然存在。
// ============================================================================
use crate::core::config::SETTINGS;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// 连接池类型别名（sqlx PostgreSQL）
pub type DbPool = PgPool;

// 全局单例连接池：仅 init_pool 写入一次，之后全程只读
static DB_POOL: OnceCell<DbPool> = OnceCell::new();

/// 构造 PostgreSQL 连接字符串
///
/// 优先使用 `DATABASE_URL`；如果为空，则用独立配置项拼接。
pub fn build_database_url() -> String {
    if !SETTINGS.database_url.is_empty() {
        return SETTINGS.database_url.clone();
    }

    format!(
        "postgres://{}:{}@{}:{}/{}",
        SETTINGS.db_user,
        SETTINGS.db_password,
        SETTINGS.db_host,
        SETTINGS.db_port,
        SETTINGS.db_name
    )
}

/// 初始化数据库连接池并保存为全局单例
pub async fn init_pool() -> Result<&'static DbPool, sqlx::Error> {
    let database_url = build_database_url();

    if database_url.is_empty() {
        return Err(sqlx::Error::Configuration(
            "数据库连接字符串为空，请配置 DATABASE_URL 或 DB_HOST/DB_USER/DB_PASSWORD/DB_NAME".into(),
        ));
    }

    tracing::info!("Connecting to PostgreSQL: {}", mask_url(&database_url));

    let pool = PgPoolOptions::new()
        .max_connections(SETTINGS.db_pool_size)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(10))
        // 远程 DB RTT 较高（~280ms），保持长连接避免每次请求重建 TCP+认证
        .idle_timeout(None)
        .max_lifetime(None)
        .connect(&database_url)
        .await?;

    tracing::info!("PostgreSQL connection pool initialized");
    DB_POOL.set(pool).map_err(|_| {
        sqlx::Error::Configuration("数据库连接池已经初始化过".into())
    })?;

    Ok(DB_POOL.get().unwrap())
}

/// 获取全局数据库连接池
/// 未初始化（无 DB 模式）时返回 None，调用方需自行降级处理。
pub fn get_pool() -> Option<&'static DbPool> {
    DB_POOL.get()
}

/// 简单 ping 数据库，验证连接可用
pub async fn ping(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await.map(|_| ())
}

/// 隐藏密码，用于日志输出
fn mask_url(url: &str) -> String {
    if SETTINGS.db_password.is_empty() {
        return url.to_string();
    }
    url.replace(&SETTINGS.db_password, "***")
}
