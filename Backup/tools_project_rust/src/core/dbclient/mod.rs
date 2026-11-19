// ============================================================================
// DbClient 模块 —— BM2::Data::DbClient（C++）的 Rust 移植
// 职责：在 sqlx PgPool 之上提供 CDbConnection / CDbCommand 风格的命令式 DB 访问：
//       @name 命名参数、事务、ExecuteNonQuery/ExecuteQuery/ExecuteReader/ExecuteScalar、
//       Read + 按列取值、GetSchema。
// 约定：列索引 0 基；NULL 列的 get_xxx 返回类型默认值，用 is_db_null 判空；
//       execute_reader 一次性拉取结果集到内存，read() 只移动游标。
// 当前尚未接入路由/服务，接入前统一允许 dead_code。
// ============================================================================
#![allow(dead_code)]

mod command;
mod connection;
mod parameter;
mod table;
mod value;

pub use command::{CommandType, DbCommand};
pub use connection::{ConnectOptions, DbConnection, DbKind, DATABASE_KIND};
pub use parameter::{DataParameter, ParamDirection, ParameterCollection};
pub use table::{DataColumn, DataRow, DataTable};
pub use value::{DbError, DbResult, DbValue, FromDbValue};
