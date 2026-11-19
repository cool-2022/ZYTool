// 连接对象（对应 C++ CDbConnection）：持有 PgPool 与当前事务，提供全局"当前连接"。
use std::sync::Arc;

use once_cell::sync::OnceCell;
use sqlx::postgres::{PgArguments, PgConnectOptions, PgPool, PgPoolOptions, PgQueryResult, PgRow, Postgres};
use sqlx::query::Query;
use sqlx::Transaction;
use tokio::sync::Mutex;

use super::table::DataTable;
use super::value::{DbError, DbResult, DbValue};
use crate::core::config::SETTINGS;
use crate::core::db::get_pool;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbKind {
    Postgres,
}

/// 对应 C++ 静态成员 CDbConnection::DatabaseKind
pub const DATABASE_KIND: DbKind = DbKind::Postgres;

/// 对应 C++ Create(bIntergratedSecurity, strDatabase, strCatalog, strUsername, strPassword, strAppName)
/// database 为 host 或 host:port，catalog 为库名；integrated_security 为 true 时忽略用户名密码。
#[derive(Debug, Clone, Default)]
pub struct ConnectOptions {
    pub integrated_security: bool,
    pub database: String,
    pub catalog: String,
    pub username: String,
    pub password: String,
    pub app_name: String,
}

static CURRENT: OnceCell<Arc<DbConnection>> = OnceCell::new();

pub struct DbConnection {
    pool: PgPool,
    tx: Mutex<Option<Transaction<'static, Postgres>>>,
}

impl DbConnection {
    pub fn from_pool(pool: PgPool) -> Self {
        Self {
            pool,
            tx: Mutex::new(None),
        }
    }

    /// 按选项新建连接池；首次创建的连接会登记为全局当前连接
    pub async fn create(opts: &ConnectOptions) -> DbResult<Arc<Self>> {
        let (host, port) = match opts.database.rsplit_once(':') {
            Some((h, p)) => (
                h.to_string(),
                p.parse::<u16>()
                    .map_err(|_| DbError::msg(format!("无效端口: {}", p), "DbConnection::create"))?,
            ),
            None => (opts.database.clone(), 5432),
        };
        let mut pg = PgConnectOptions::new()
            .host(&host)
            .port(port)
            .database(&opts.catalog);
        if !opts.integrated_security {
            pg = pg.username(&opts.username).password(&opts.password);
        }
        if !opts.app_name.is_empty() {
            pg = pg.application_name(&opts.app_name);
        }

        let pool = PgPoolOptions::new()
            .max_connections(SETTINGS.db_pool_size)
            .connect_with(pg)
            .await?;
        let conn = Arc::new(Self::from_pool(pool));
        let _ = CURRENT.set(conn.clone());
        Ok(conn)
    }

    /// 对应 C++ GetCurrent：优先返回 create 登记的连接，否则包装 core::db 的全局连接池
    pub fn get_current() -> DbResult<Arc<Self>> {
        if let Some(c) = CURRENT.get() {
            return Ok(c.clone());
        }
        let pool = get_pool().ok_or_else(|| DbError::msg("数据库未连接", "DbConnection::get_current"))?;
        Ok(CURRENT
            .get_or_init(|| Arc::new(Self::from_pool(pool.clone())))
            .clone())
    }

    /// 对应 C++ GetXAConnection。PostgreSQL 下没有独立的 XA 连接，等同于 get_current，
    /// 事务仍为本地事务而非分布式事务。
    pub fn get_xa_connection() -> DbResult<Arc<Self>> {
        Self::get_current()
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn in_transaction(&self) -> bool {
        self.tx.lock().await.is_some()
    }

    pub async fn begin_transaction(&self) -> DbResult<()> {
        let mut guard = self.tx.lock().await;
        if guard.is_some() {
            return Err(DbError::msg("事务已开始，不支持嵌套事务", "DbConnection::begin_transaction"));
        }
        *guard = Some(self.pool.begin().await?);
        Ok(())
    }

    pub async fn commit(&self) -> DbResult<()> {
        let tx = self
            .tx
            .lock()
            .await
            .take()
            .ok_or_else(|| DbError::msg("没有进行中的事务", "DbConnection::commit"))?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn rollback(&self) -> DbResult<()> {
        let tx = self
            .tx
            .lock()
            .await
            .take()
            .ok_or_else(|| DbError::msg("没有进行中的事务", "DbConnection::rollback"))?;
        tx.rollback().await?;
        Ok(())
    }

    /// 回滚未提交事务并关闭连接池
    pub async fn close(&self) {
        if let Some(tx) = self.tx.lock().await.take() {
            let _ = tx.rollback().await;
        }
        self.pool.close().await;
    }

    /// 对应 C++ GetSchema(table, strCollectionName, cRestrictions, strRestrictionValues)。
    /// 支持的集合与限制项（按位置，None 表示不限制）：
    /// - Tables:    [catalog, schema, table, table_type]
    /// - Columns:   [catalog, schema, table, column]
    /// - Schemas:   [catalog, schema]
    /// - Databases: [name]
    pub async fn get_schema(&self, collection_name: &str, restrictions: &[Option<&str>]) -> DbResult<DataTable> {
        let (sql, count) = match collection_name.to_ascii_lowercase().as_str() {
            "tables" => (
                "SELECT table_catalog, table_schema, table_name, table_type \
                 FROM information_schema.tables \
                 WHERE ($1::text IS NULL OR table_catalog = $1) \
                   AND ($2::text IS NULL OR table_schema = $2) \
                   AND ($3::text IS NULL OR table_name = $3) \
                   AND ($4::text IS NULL OR table_type = $4) \
                 ORDER BY table_schema, table_name",
                4,
            ),
            "columns" => (
                "SELECT table_catalog, table_schema, table_name, column_name, ordinal_position, \
                        column_default, is_nullable, data_type, character_maximum_length, \
                        numeric_precision, numeric_scale \
                 FROM information_schema.columns \
                 WHERE ($1::text IS NULL OR table_catalog = $1) \
                   AND ($2::text IS NULL OR table_schema = $2) \
                   AND ($3::text IS NULL OR table_name = $3) \
                   AND ($4::text IS NULL OR column_name = $4) \
                 ORDER BY table_schema, table_name, ordinal_position",
                4,
            ),
            "schemas" => (
                "SELECT catalog_name, schema_name, schema_owner \
                 FROM information_schema.schemata \
                 WHERE ($1::text IS NULL OR catalog_name = $1) \
                   AND ($2::text IS NULL OR schema_name = $2) \
                 ORDER BY schema_name",
                2,
            ),
            "databases" => (
                "SELECT datname AS database_name, pg_encoding_to_char(encoding) AS encoding \
                 FROM pg_database \
                 WHERE NOT datistemplate AND ($1::text IS NULL OR datname = $1) \
                 ORDER BY datname",
                1,
            ),
            other => {
                return Err(DbError::msg(
                    format!("不支持的架构集合: {}", other),
                    "DbConnection::get_schema",
                ))
            }
        };

        let params: Vec<DbValue> = (0..count)
            .map(|i| restrictions.get(i).copied().flatten().map_or(DbValue::Null, DbValue::from))
            .collect();
        let rows = self.fetch_all(sql, &params).await?;
        let mut table = DataTable::from_rows(&rows)?;
        table.table_name = collection_name.to_string();
        Ok(table)
    }

    /// 在当前事务（若有）或连接池上查询全部行
    pub(super) async fn fetch_all(&self, sql: &str, params: &[DbValue]) -> DbResult<Vec<PgRow>> {
        let query = bind_all(sqlx::query(sql), params);
        let mut guard = self.tx.lock().await;
        let rows = match guard.as_mut() {
            Some(tx) => query.fetch_all(&mut **tx).await?,
            None => query.fetch_all(&self.pool).await?,
        };
        Ok(rows)
    }

    /// 在当前事务（若有）或连接池上执行并返回影响行数
    pub(super) async fn execute(&self, sql: &str, params: &[DbValue]) -> DbResult<u64> {
        let query = bind_all(sqlx::query(sql), params);
        let mut guard = self.tx.lock().await;
        let result: PgQueryResult = match guard.as_mut() {
            Some(tx) => query.execute(&mut **tx).await?,
            None => query.execute(&self.pool).await?,
        };
        Ok(result.rows_affected())
    }
}

fn bind_all<'q>(mut query: Query<'q, Postgres, PgArguments>, params: &[DbValue]) -> Query<'q, Postgres, PgArguments> {
    for p in params {
        query = match p {
            DbValue::Null => query.bind(None::<String>),
            DbValue::Bool(v) => query.bind(*v),
            DbValue::I16(v) => query.bind(*v),
            DbValue::I32(v) => query.bind(*v),
            DbValue::I64(v) => query.bind(*v),
            DbValue::F32(v) => query.bind(*v),
            DbValue::F64(v) => query.bind(*v),
            DbValue::Decimal(v) => query.bind(*v),
            DbValue::DateTime(v) => query.bind(*v),
            DbValue::Str(v) => query.bind(v.clone()),
            DbValue::Bytes(v) => query.bind(v.clone()),
        };
    }
    query
}
