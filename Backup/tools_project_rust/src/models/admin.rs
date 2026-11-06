use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct AdminTableInfo {
    pub name: String,
    pub comment: Option<String>,
    pub row_estimate: i64,
}

#[derive(Debug, Serialize)]
pub struct AdminTablesResponse {
    pub tables: Vec<AdminTableInfo>,
}

#[derive(Debug, Serialize)]
pub struct AdminColumnInfo {
    pub name: String,
    pub data_type: String,
    pub nullable: bool,
    pub is_primary: bool,
    pub column_default: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminColumnsResponse {
    pub table: String,
    pub columns: Vec<AdminColumnInfo>,
}

#[derive(Debug, Deserialize)]
pub struct AdminRowsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminRowsResponse {
    pub table: String,
    pub columns: Vec<String>,
    pub rows: Vec<Value>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

// ===================== 字根管理（字段数据字典） =====================

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FieldRootRow {
    pub id: i64,
    pub field_name: String,
    pub data_type: String,
    pub data_length: Option<i32>,
    pub data_precision: Option<i32>,
    pub data_scale: Option<i32>,
    pub comment: String,
    pub remark: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 引用该字根的表字段（按字段名从系统表实时关联）
#[derive(Debug, Serialize)]
pub struct FieldRootRef {
    pub table_name: String,
    pub data_type: String,
    pub data_length: Option<i32>,
    pub data_precision: Option<i32>,
    pub data_scale: Option<i32>,
    pub type_expr: String,
    pub comment: String,
    pub type_synced: bool,
    pub comment_synced: bool,
    pub synced: bool,
    pub diffs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct FieldRootInfo {
    /// None 表示未入库（仅实时字段，字典表无记录）
    pub id: Option<i64>,
    pub registered: bool,
    pub field_name: String,
    pub data_type: String,
    pub data_length: Option<i32>,
    pub data_precision: Option<i32>,
    pub data_scale: Option<i32>,
    pub type_expr: String,
    pub comment: String,
    pub remark: String,
    pub updated_at: Option<DateTime<Utc>>,
    pub refs: Vec<FieldRootRef>,
}

#[derive(Debug, Deserialize)]
pub struct FieldRootsQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FieldRootsResponse {
    pub rows: Vec<FieldRootInfo>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Deserialize)]
pub struct FieldRootUpsertRequest {
    pub field_name: String,
    pub data_type: Option<String>,
    pub data_length: Option<i32>,
    pub data_precision: Option<i32>,
    pub data_scale: Option<i32>,
    pub comment: Option<String>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FieldRootResponse {
    pub root_item: FieldRootInfo,
}

#[derive(Debug, Serialize)]
pub struct FieldRootsScanResponse {
    pub inserted: i64,
    pub total_roots: i64,
}

#[derive(Debug, Deserialize)]
pub struct FieldRootSyncRequest {
    pub sync_comment: Option<bool>,
    pub sync_type: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct FieldRootSyncResult {
    pub table_name: String,
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct FieldRootSyncResponse {
    pub results: Vec<FieldRootSyncResult>,
}

// ===================== 服务状态 =====================

#[derive(Debug, Serialize)]
pub struct AdminServiceInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub status: String,
    pub db_connected: bool,
    pub uptime_seconds: i64,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct AdminServicesResponse {
    pub service: AdminServiceInfo,
}
