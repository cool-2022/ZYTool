// 列值 / 参数值的统一载体（对应 C++ 的 CDataProxy + CDBNull）与 DbClient 错误类型。
use std::fmt;

use chrono::{NaiveDate, NaiveDateTime};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;

use crate::core::error::{internal_error, AppError};

/// 对应 C++ CDbException(nDbCode, strMessage, strSource)：code 为 PostgreSQL SQLSTATE（无则为空）
#[derive(Debug, Clone)]
pub struct DbError {
    pub code: String,
    pub message: String,
    pub source: String,
}

impl DbError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, source: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            source: source.into(),
        }
    }

    pub fn msg(message: impl Into<String>, source: &str) -> Self {
        Self::new("", message, source)
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.code.is_empty() {
            write!(f, "[{}] {}", self.source, self.message)
        } else {
            write!(f, "[{}][{}] {}", self.source, self.code, self.message)
        }
    }
}

impl std::error::Error for DbError {}

impl From<sqlx::Error> for DbError {
    fn from(e: sqlx::Error) -> Self {
        let code = e
            .as_database_error()
            .and_then(|d| d.code())
            .map(|c| c.to_string())
            .unwrap_or_default();
        Self::new(code, e.to_string(), "sqlx")
    }
}

impl From<DbError> for AppError {
    fn from(e: DbError) -> Self {
        internal_error(e.to_string())
    }
}

pub type DbResult<T> = Result<T, DbError>;

/// 统一列值。整数统一收敛为 i16/i32/i64，无符号与 i8 在转换时映射到更宽类型；u64 超出 i64 时落到 Decimal。
#[derive(Debug, Clone, PartialEq)]
pub enum DbValue {
    Null,
    Bool(bool),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Decimal(Decimal),
    DateTime(NaiveDateTime),
    Str(String),
    Bytes(Vec<u8>),
}

const DATETIME_FORMATS: &[&str] = &[
    "%Y-%m-%d %H:%M:%S%.f",
    "%Y-%m-%dT%H:%M:%S%.f",
    "%Y-%m-%d %H:%M:%S",
    "%Y-%m-%dT%H:%M:%S",
    "%Y%m%d%H%M%S",
];
const DATE_FORMATS: &[&str] = &["%Y-%m-%d", "%Y%m%d"];

impl DbValue {
    pub fn is_null(&self) -> bool {
        matches!(self, DbValue::Null)
    }

    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DbValue::Null => None,
            DbValue::Bool(b) => Some(*b as i64),
            DbValue::I16(n) => Some(*n as i64),
            DbValue::I32(n) => Some(*n as i64),
            DbValue::I64(n) => Some(*n),
            DbValue::F32(n) => Some(*n as i64),
            DbValue::F64(n) => Some(*n as i64),
            DbValue::Decimal(d) => d.trunc().to_i64(),
            DbValue::Str(s) => {
                let s = s.trim();
                s.parse::<i64>()
                    .ok()
                    .or_else(|| s.parse::<Decimal>().ok().and_then(|d| d.trunc().to_i64()))
            }
            DbValue::DateTime(_) | DbValue::Bytes(_) => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DbValue::Null => None,
            DbValue::Bool(b) => Some(*b as i64 as f64),
            DbValue::I16(n) => Some(*n as f64),
            DbValue::I32(n) => Some(*n as f64),
            DbValue::I64(n) => Some(*n as f64),
            DbValue::F32(n) => Some(*n as f64),
            DbValue::F64(n) => Some(*n),
            DbValue::Decimal(d) => d.to_f64(),
            DbValue::Str(s) => s.trim().parse::<f64>().ok(),
            DbValue::DateTime(_) | DbValue::Bytes(_) => None,
        }
    }

    pub fn as_decimal(&self) -> Option<Decimal> {
        match self {
            DbValue::Null => None,
            DbValue::Bool(b) => Some(Decimal::from(*b as i64)),
            DbValue::I16(n) => Some(Decimal::from(*n)),
            DbValue::I32(n) => Some(Decimal::from(*n)),
            DbValue::I64(n) => Some(Decimal::from(*n)),
            DbValue::F32(n) => Decimal::from_f32(*n),
            DbValue::F64(n) => Decimal::from_f64(*n),
            DbValue::Decimal(d) => Some(*d),
            DbValue::Str(s) => s.trim().parse::<Decimal>().ok(),
            DbValue::DateTime(_) | DbValue::Bytes(_) => None,
        }
    }

    pub fn as_datetime(&self) -> Option<NaiveDateTime> {
        match self {
            DbValue::DateTime(t) => Some(*t),
            DbValue::Str(s) => {
                let s = s.trim();
                DATETIME_FORMATS
                    .iter()
                    .find_map(|f| NaiveDateTime::parse_from_str(s, f).ok())
                    .or_else(|| {
                        DATE_FORMATS
                            .iter()
                            .find_map(|f| NaiveDate::parse_from_str(s, f).ok())
                            .and_then(|d| d.and_hms_opt(0, 0, 0))
                    })
            }
            _ => None,
        }
    }

    /// NULL 返回 None，其余按 Display 文本化
    pub fn as_string(&self) -> Option<String> {
        match self {
            DbValue::Null => None,
            other => Some(other.to_string()),
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        match self {
            DbValue::Null => None,
            DbValue::Bytes(b) => Some(b.clone()),
            DbValue::Str(s) => Some(s.as_bytes().to_vec()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DbValue::Null => None,
            DbValue::Bool(b) => Some(*b),
            DbValue::Str(s) => match s.trim().to_ascii_lowercase().as_str() {
                "true" | "t" | "y" | "yes" | "1" => Some(true),
                "false" | "f" | "n" | "no" | "0" | "" => Some(false),
                _ => None,
            },
            other => other.as_i64().map(|n| n != 0),
        }
    }
}

impl fmt::Display for DbValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbValue::Null => Ok(()),
            DbValue::Bool(b) => write!(f, "{}", b),
            DbValue::I16(n) => write!(f, "{}", n),
            DbValue::I32(n) => write!(f, "{}", n),
            DbValue::I64(n) => write!(f, "{}", n),
            DbValue::F32(n) => write!(f, "{}", n),
            DbValue::F64(n) => write!(f, "{}", n),
            DbValue::Decimal(d) => write!(f, "{}", d.normalize()),
            DbValue::DateTime(t) => write!(f, "{}", t.format("%Y-%m-%d %H:%M:%S")),
            DbValue::Str(s) => f.write_str(s),
            DbValue::Bytes(b) => {
                for byte in b {
                    write!(f, "{:02x}", byte)?;
                }
                Ok(())
            }
        }
    }
}

impl From<bool> for DbValue {
    fn from(v: bool) -> Self {
        DbValue::Bool(v)
    }
}
impl From<i8> for DbValue {
    fn from(v: i8) -> Self {
        DbValue::I16(v as i16)
    }
}
impl From<i16> for DbValue {
    fn from(v: i16) -> Self {
        DbValue::I16(v)
    }
}
impl From<i32> for DbValue {
    fn from(v: i32) -> Self {
        DbValue::I32(v)
    }
}
impl From<i64> for DbValue {
    fn from(v: i64) -> Self {
        DbValue::I64(v)
    }
}
impl From<u8> for DbValue {
    fn from(v: u8) -> Self {
        DbValue::I16(v as i16)
    }
}
impl From<u16> for DbValue {
    fn from(v: u16) -> Self {
        DbValue::I32(v as i32)
    }
}
impl From<u32> for DbValue {
    fn from(v: u32) -> Self {
        DbValue::I64(v as i64)
    }
}
impl From<u64> for DbValue {
    fn from(v: u64) -> Self {
        match i64::try_from(v) {
            Ok(n) => DbValue::I64(n),
            Err(_) => DbValue::Decimal(Decimal::from(v)),
        }
    }
}
impl From<f32> for DbValue {
    fn from(v: f32) -> Self {
        DbValue::F32(v)
    }
}
impl From<f64> for DbValue {
    fn from(v: f64) -> Self {
        DbValue::F64(v)
    }
}
impl From<Decimal> for DbValue {
    fn from(v: Decimal) -> Self {
        DbValue::Decimal(v)
    }
}
impl From<NaiveDateTime> for DbValue {
    fn from(v: NaiveDateTime) -> Self {
        DbValue::DateTime(v)
    }
}
impl From<NaiveDate> for DbValue {
    fn from(v: NaiveDate) -> Self {
        DbValue::DateTime(v.and_hms_opt(0, 0, 0).expect("midnight is always valid"))
    }
}
impl From<&str> for DbValue {
    fn from(v: &str) -> Self {
        DbValue::Str(v.to_string())
    }
}
impl From<String> for DbValue {
    fn from(v: String) -> Self {
        DbValue::Str(v)
    }
}
impl From<Vec<u8>> for DbValue {
    fn from(v: Vec<u8>) -> Self {
        DbValue::Bytes(v)
    }
}
impl<T: Into<DbValue>> From<Option<T>> for DbValue {
    fn from(v: Option<T>) -> Self {
        v.map_or(DbValue::Null, Into::into)
    }
}

/// 从 DbValue 取值的类型转换（对应 C++ 的 GetInt32/GetString/... 重载族）。
/// 返回 None 表示 NULL 或无法转换。
pub trait FromDbValue: Sized {
    fn from_db_value(v: &DbValue) -> Option<Self>;
}

macro_rules! impl_from_db_value_int {
    ($($t:ty),*) => {$(
        impl FromDbValue for $t {
            fn from_db_value(v: &DbValue) -> Option<Self> {
                v.as_i64().and_then(|n| <$t>::try_from(n).ok())
            }
        }
    )*};
}
impl_from_db_value_int!(i8, i16, i32, i64, u8, u16, u32, u64);

impl FromDbValue for f32 {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_f64().map(|n| n as f32)
    }
}
impl FromDbValue for f64 {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_f64()
    }
}
impl FromDbValue for Decimal {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_decimal()
    }
}
impl FromDbValue for NaiveDateTime {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_datetime()
    }
}
impl FromDbValue for String {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_string()
    }
}
impl FromDbValue for Vec<u8> {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_bytes()
    }
}
impl FromDbValue for bool {
    fn from_db_value(v: &DbValue) -> Option<Self> {
        v.as_bool()
    }
}
