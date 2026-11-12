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
pub fn get_connection() -> Option<MultiplexedConnection> {
    REDIS_CONN.get().cloned()
}

/// 简单 ping Redis，验证连接可用
#[allow(dead_code)] // 暂无业务消费，后续健康检查接入时使用
pub async fn ping(conn: &mut MultiplexedConnection) -> Result<(), redis::RedisError> {
    redis::cmd("PING").query_async::<()>(conn).await
}

// ===================== 通用缓存助手（全部静默降级：无 Redis 或出错时视为未命中/写入忽略） =====================

/// 读取缓存字符串；未命中/无 Redis/出错均返回 None
pub async fn cache_get(key: &str) -> Option<String> {
    let mut conn = get_connection()?;
    redis::cmd("GET")
        .arg(key)
        .query_async::<Option<String>>(&mut conn)
        .await
        .ok()
        .flatten()
}

/// 写入缓存并设置 TTL（秒）；失败静默忽略
pub async fn cache_set(key: &str, value: &str, ttl_secs: u64) {
    let Some(mut conn) = get_connection() else { return };
    let _ = redis::cmd("SET")
        .arg(key)
        .arg(value)
        .arg("EX")
        .arg(ttl_secs)
        .query_async::<()>(&mut conn)
        .await;
}

/// 删除缓存；失败静默忽略
pub async fn cache_del(key: &str) {
    let Some(mut conn) = get_connection() else { return };
    let _ = redis::cmd("DEL")
        .arg(key)
        .query_async::<()>(&mut conn)
        .await;
}

/// 计数器自增并在首次创建时设置 TTL（秒），返回自增后的值；无 Redis 或出错返回 None
pub async fn incr_with_ttl(key: &str, ttl_secs: u64) -> Option<u32> {
    let mut conn = get_connection()?;
    let count = redis::cmd("INCR")
        .arg(key)
        .query_async::<u32>(&mut conn)
        .await
        .ok()?;
    if count == 1 {
        let _ = redis::cmd("EXPIRE")
            .arg(key)
            .arg(ttl_secs)
            .query_async::<()>(&mut conn)
            .await;
    }
    Some(count)
}

/// 固定窗口限流：key 在窗口内第 max+1 次起拒绝。返回 true 表示放行；
/// 无 Redis 或出错时放行（可用性优先，限流静默失效）
pub async fn rate_limit_hit(key: &str, max: u32, window_secs: u64) -> bool {
    match incr_with_ttl(key, window_secs).await {
        Some(count) => count <= max,
        None => true,
    }
}

fn config_error(msg: &'static str) -> redis::RedisError {
    redis::RedisError::from((redis::ErrorKind::InvalidClientConfig, msg))
}
