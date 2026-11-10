use crate::core::config::SETTINGS;
use once_cell::sync::OnceCell;
use redis::aio::MultiplexedConnection;

static REDIS_CONN: OnceCell<MultiplexedConnection> = OnceCell::new();

/// 初始化 Redis 连接并保存为全局单例。
/// 多路复用连接内部共享同一 TCP 连接，克隆代价低，无需自建连接池。
pub async fn init_redis() -> Result<&'static MultiplexedConnection, redis::RedisError> {
    if SETTINGS.redis_host.is_empty() {
        return Err(config_error("未配置 REDIS_HOST"));
    }

    let url = format!("redis://{}:{}", SETTINGS.redis_host, SETTINGS.redis_port);
    tracing::info!("Connecting to Redis: {}", url);

    let client = redis::Client::open(url)?;
    let conn = client.get_multiplexed_async_connection().await?;

    tracing::info!("Redis connection initialized");
    REDIS_CONN
        .set(conn)
        .map_err(|_| config_error("Redis 已经初始化过"))?;

    Ok(REDIS_CONN.get().unwrap())
}

/// 获取全局 Redis 连接；未初始化（未配置或连接失败）时返回 None
#[allow(dead_code)] // 暂无业务消费，后续缓存/限流接入时使用
pub fn get_connection() -> Option<MultiplexedConnection> {
    REDIS_CONN.get().cloned()
}

/// 简单 ping Redis，验证连接可用
#[allow(dead_code)] // 暂无业务消费，后续健康检查接入时使用
pub async fn ping(conn: &mut MultiplexedConnection) -> Result<(), redis::RedisError> {
    redis::cmd("PING").query_async::<()>(conn).await
}

fn config_error(msg: &'static str) -> redis::RedisError {
    redis::RedisError::from((redis::ErrorKind::InvalidClientConfig, msg))
}
