// 内存结果集（对应 C++ CDataTable 在 DbClient 中的用法）及 PgRow → DbValue 解码。
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use rust_decimal::Decimal;
use sqlx::postgres::PgRow;
use sqlx::{Column, Row, TypeInfo, ValueRef};

use super::value::{DbError, DbResult, DbValue};

#[derive(Debug, Clone)]
pub struct DataColumn {
    pub name: String,
    pub type_name: String,
    pub ordinal: usize,
}

pub type DataRow = Vec<DbValue>;

#[derive(Debug, Clone, Default)]
pub struct DataTable {
    pub table_name: String,
    pub columns: Vec<DataColumn>,
    pub rows: Vec<DataRow>,
}

impl DataTable {
    pub fn new(table_name: impl Into<String>) -> Self {
        Self {
            table_name: table_name.into(),
            ..Self::default()
        }
    }

    /// 由 sqlx 行集构建；列定义取自首行（空结果集无列信息）
    pub fn from_rows(rows: &[PgRow]) -> DbResult<Self> {
        let mut table = Self::default();
        let Some(first) = rows.first() else {
            return Ok(table);
        };
        table.columns = columns_of(first);
        for row in rows {
            table.rows.push(decode_row(row)?);
        }
        Ok(table)
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub fn column_index(&self, name: &str) -> Option<usize> {
        self.columns
            .iter()
            .position(|c| c.name.eq_ignore_ascii_case(name))
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&DbValue> {
        self.rows.get(row).and_then(|r| r.get(col))
    }

    pub fn get_by_name(&self, row: usize, name: &str) -> Option<&DbValue> {
        self.column_index(name).and_then(|c| self.get(row, c))
    }

    /// 追加一行；列数不匹配时报错
    pub fn add_row(&mut self, row: DataRow) -> DbResult<()> {
        if !self.columns.is_empty() && row.len() != self.columns.len() {
            return Err(DbError::msg(
                format!("行列数 {} 与表列数 {} 不一致", row.len(), self.columns.len()),
                "DataTable::add_row",
            ));
        }
        self.rows.push(row);
        Ok(())
    }

    pub fn set_row(&mut self, index: usize, row: DataRow) -> DbResult<()> {
        if index >= self.rows.len() {
            return Err(DbError::msg(
                format!("行索引 {} 超出范围（共 {} 行）", index, self.rows.len()),
                "DataTable::set_row",
            ));
        }
        if !self.columns.is_empty() && row.len() != self.columns.len() {
            return Err(DbError::msg(
                format!("行列数 {} 与表列数 {} 不一致", row.len(), self.columns.len()),
                "DataTable::set_row",
            ));
        }
        self.rows[index] = row;
        Ok(())
    }

    pub fn clear(&mut self) {
        self.rows.clear();
    }
}

pub(super) fn columns_of(row: &PgRow) -> Vec<DataColumn> {
    row.columns()
        .iter()
        .map(|c| DataColumn {
            name: c.name().to_string(),
            type_name: c.type_info().name().to_string(),
            ordinal: c.ordinal(),
        })
        .collect()
}

pub(super) fn decode_row(row: &PgRow) -> DbResult<DataRow> {
    (0..row.len()).map(|i| decode_column(row, i)).collect()
}

/// 按 PostgreSQL 类型名解码单列；TIMESTAMPTZ 转为本地时间，DATE 取当日零点，
/// UUID/JSON/TIME 文本化；数组等未覆盖类型直接报错而非静默丢数据。
fn decode_column(row: &PgRow, idx: usize) -> DbResult<DbValue> {
    let col = &row.columns()[idx];
    if row.try_get_raw(idx)?.is_null() {
        return Ok(DbValue::Null);
    }
    let value = match col.type_info().name() {
        "BOOL" => DbValue::Bool(row.try_get(idx)?),
        "INT2" => DbValue::I16(row.try_get(idx)?),
        "INT4" => DbValue::I32(row.try_get(idx)?),
        "INT8" => DbValue::I64(row.try_get(idx)?),
        "FLOAT4" => DbValue::F32(row.try_get(idx)?),
        "FLOAT8" => DbValue::F64(row.try_get(idx)?),
        "NUMERIC" => DbValue::Decimal(row.try_get::<Decimal, _>(idx)?),
        "TEXT" | "VARCHAR" | "BPCHAR" | "NAME" | "CITEXT" => DbValue::Str(row.try_get(idx)?),
        "TIMESTAMP" => DbValue::DateTime(row.try_get::<NaiveDateTime, _>(idx)?),
        "TIMESTAMPTZ" => {
            let t: DateTime<Utc> = row.try_get(idx)?;
            DbValue::DateTime(t.with_timezone(&Local).naive_local())
        }
        "DATE" => {
            let d: NaiveDate = row.try_get(idx)?;
            DbValue::DateTime(d.and_hms_opt(0, 0, 0).expect("midnight is always valid"))
        }
        "TIME" => DbValue::Str(row.try_get::<NaiveTime, _>(idx)?.to_string()),
        "BYTEA" => DbValue::Bytes(row.try_get(idx)?),
        "UUID" => DbValue::Str(row.try_get::<uuid::Uuid, _>(idx)?.to_string()),
        "JSON" | "JSONB" => DbValue::Str(row.try_get::<serde_json::Value, _>(idx)?.to_string()),
        other => {
            return Err(DbError::msg(
                format!("不支持的列类型 {}（列 {}）", other, col.name()),
                "decode_column",
            ))
        }
    };
    Ok(value)
}
