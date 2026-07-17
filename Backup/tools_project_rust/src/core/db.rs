use crate::core::config::SETTINGS;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub type DbPool = PgPool;

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
        .acquire_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;

    tracing::info!("PostgreSQL connection pool initialized");
    DB_POOL.set(pool).map_err(|_| {
        sqlx::Error::Configuration("数据库连接池已经初始化过".into())
    })?;

    Ok(DB_POOL.get().unwrap())
}

/// 获取全局数据库连接池
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
