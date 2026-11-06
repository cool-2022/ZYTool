use once_cell::sync::Lazy;
use serde_json::Value;
use std::time::Instant;

use chrono::Utc;

use crate::core::config::SETTINGS;
use crate::core::db::{self, DbPool};
use crate::core::error::{bad_request, internal_error, AppResult};
use crate::models::admin::{
    AdminColumnInfo, AdminServiceInfo, AdminTableInfo, FieldRootInfo, FieldRootRef, FieldRootRow,
    FieldRootSyncResult, FieldRootUpsertRequest,
};

static START_TIME: Lazy<Instant> = Lazy::new(Instant::now);

pub struct AdminRowsData {
    pub columns: Vec<String>,
    pub rows: Vec<Value>,
    pub total: i64,
}

/// 表名只允许小写字母/下划线/数字，防止标识符注入
fn validate_table_name(table: &str) -> AppResult<()> {
    let valid = !table.is_empty()
        && table.len() <= 63
        && table
            .chars()
            .enumerate()
            .all(|(i, c)| c.is_ascii_lowercase() || c == '_' || (i > 0 && c.is_ascii_digit()));
    if valid {
        Ok(())
    } else {
        Err(bad_request("无效的表名"))
    }
}

/// 校验表存在于 public 模式，后续 SQL 才能安全拼接该标识符
async fn ensure_table_exists(pool: &DbPool, table: &str) -> AppResult<()> {
    validate_table_name(table)?;
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1 FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name = $1
        )",
    )
    .bind(table)
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("校验数据表失败: {}", e)))?;

    if exists {
        Ok(())
    } else {
        Err(bad_request(format!("数据表不存在: {}", table)))
    }
}

fn escape_like(keyword: &str) -> String {
    keyword
        .replace('\\', "\\\\")
        .replace('%', "\\%")
        .replace('_', "\\_")
}

pub async fn list_tables(pool: &DbPool) -> AppResult<Vec<AdminTableInfo>> {
    let rows = sqlx::query_as::<_, (String, Option<String>, i64)>(
        "SELECT c.relname, obj_description(c.oid, 'pg_class'), c.reltuples::bigint
         FROM pg_class c
         JOIN pg_namespace n ON n.oid = c.relnamespace
         WHERE c.relkind = 'r' AND n.nspname = 'public'
         ORDER BY c.relname",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询表列表失败: {}", e)))?;

    Ok(rows
        .into_iter()
        .map(|(name, comment, row_estimate)| AdminTableInfo {
            name,
            comment,
            row_estimate,
        })
        .collect())
}

pub async fn list_columns(pool: &DbPool, table: &str) -> AppResult<Vec<AdminColumnInfo>> {
    ensure_table_exists(pool, table).await?;

    let rows = sqlx::query_as::<_, (String, String, bool, bool, Option<String>, Option<String>)>(
        "SELECT c.column_name,
                c.data_type,
                c.is_nullable = 'YES',
                EXISTS (
                    SELECT 1
                    FROM pg_index i
                    JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY (i.indkey)
                    WHERE i.indrelid = format('%I.%I', c.table_schema, c.table_name)::regclass
                      AND i.indisprimary
                      AND a.attname = c.column_name
                ),
                c.column_default,
                pg_catalog.col_description(
                    format('%I.%I', c.table_schema, c.table_name)::regclass,
                    c.ordinal_position
                )
         FROM information_schema.columns c
         WHERE c.table_schema = 'public' AND c.table_name = $1
         ORDER BY c.ordinal_position",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询表结构失败: {}", e)))?;

    Ok(rows
        .into_iter()
        .map(
            |(name, data_type, nullable, is_primary, column_default, comment)| AdminColumnInfo {
                name,
                data_type,
                nullable,
                is_primary,
                column_default,
                comment,
            },
        )
        .collect())
}

pub async fn list_rows(
    pool: &DbPool,
    table: &str,
    page: i64,
    page_size: i64,
    keyword: Option<&str>,
) -> AppResult<AdminRowsData> {
    ensure_table_exists(pool, table).await?;

    let keyword = keyword.map(escape_like).unwrap_or_default();
    // 表名已通过白名单校验与存在性确认；$1 为空串时不过滤
    let filter = "($1 = '' OR to_jsonb(t)::text ILIKE '%' || $1 || '%' ESCAPE '\\')";

    let total: i64 = sqlx::query_scalar(&format!(
        "SELECT count(*) FROM \"{}\" t WHERE {}",
        table, filter
    ))
    .bind(&keyword)
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("统计表数据失败: {}", e)))?;

    let columns: Vec<String> = sqlx::query_scalar(
        "SELECT column_name FROM information_schema.columns
         WHERE table_schema = 'public' AND table_name = $1
         ORDER BY ordinal_position",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询表字段失败: {}", e)))?;

    let rows: Vec<Value> = sqlx::query_scalar(&format!(
        "SELECT to_jsonb(t) FROM \"{}\" t WHERE {} LIMIT $2 OFFSET $3",
        table, filter
    ))
    .bind(&keyword)
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询表数据失败: {}", e)))?;

    Ok(AdminRowsData {
        columns,
        rows,
        total,
    })
}

// ===================== 字根管理（字段数据字典） =====================

const FIELD_ROOT_COLUMNS: &str =
    "id, field_name, data_type, data_length, data_precision, data_scale, \
     comment, remark, created_at, updated_at";

// typmod 中字符长度带 4 字节偏移，numeric 小数位为有符号 11 位。
const COLUMN_META_SQL: &str = "SELECT a.attname::text AS field_name, c.relname::text AS table_name,
        et.typname::text || CASE WHEN t.typcategory = 'A' THEN '[]' ELSE '' END AS data_type,
        CASE WHEN et.typname IN ('varchar', 'bpchar') AND a.atttypmod > 0
             THEN a.atttypmod - 4 END AS data_length,
        CASE WHEN et.typname = 'numeric' AND a.atttypmod > 0
             THEN ((a.atttypmod - 4) >> 16) & 65535
             WHEN et.typname IN ('time', 'timetz', 'timestamp', 'timestamptz', 'interval')
              AND a.atttypmod >= 0 AND (a.atttypmod & 65535) <> 65535
             THEN a.atttypmod & 65535 END AS data_precision,
        CASE WHEN et.typname = 'numeric' AND a.atttypmod > 0
             THEN (((a.atttypmod - 4) & 2047) # 1024) - 1024 END AS data_scale,
        COALESCE(pg_catalog.col_description(c.oid, a.attnum), '') AS comment,
        a.attinhcount > 0 AS inherited
     FROM pg_attribute a
     JOIN pg_class c ON c.oid = a.attrelid
     JOIN pg_namespace n ON n.oid = c.relnamespace
     JOIN pg_type t ON t.oid = a.atttypid
     JOIN pg_type et ON et.oid = CASE WHEN t.typcategory = 'A' THEN t.typelem ELSE t.oid END
     WHERE n.nspname = 'public' AND c.relkind IN ('r', 'p') AND a.attnum > 0
       AND NOT a.attisdropped AND c.relname <> 'field_roots'";

#[derive(sqlx::FromRow)]
struct ColumnMeta {
    field_name: String,
    table_name: String,
    data_type: String,
    data_length: Option<i32>,
    data_precision: Option<i32>,
    data_scale: Option<i32>,
    comment: String,
    inherited: bool,
}

/// 通用技术字段不入库、不展示，减少噪音（同步仍按引用实况处理）
const TECH_FIELDS: &str = "'id', 'created_at', 'updated_at'";

/// 实时字段清单（排除通用技术字段）
fn live_columns_sql() -> String {
    format!(
        "SELECT * FROM ({}) m WHERE m.field_name NOT IN ({})",
        COLUMN_META_SQL, TECH_FIELDS
    )
}

#[derive(Clone, Copy)]
enum TypeKind {
    Char,
    Numeric,
    Time,
    Plain,
}

/// 类型白名单：同步时类型名会直接拼进 ALTER 语句，只允许这里列出的名称（可带一层 [] 数组后缀）
const TYPE_KINDS: &[(&str, TypeKind)] = &[
    ("varchar", TypeKind::Char),
    ("bpchar", TypeKind::Char),
    ("numeric", TypeKind::Numeric),
    ("time", TypeKind::Time),
    ("timetz", TypeKind::Time),
    ("timestamp", TypeKind::Time),
    ("timestamptz", TypeKind::Time),
    ("interval", TypeKind::Time),
    ("text", TypeKind::Plain),
    ("int2", TypeKind::Plain),
    ("int4", TypeKind::Plain),
    ("int8", TypeKind::Plain),
    ("float4", TypeKind::Plain),
    ("float8", TypeKind::Plain),
    ("bool", TypeKind::Plain),
    ("date", TypeKind::Plain),
    ("uuid", TypeKind::Plain),
    ("json", TypeKind::Plain),
    ("jsonb", TypeKind::Plain),
    ("inet", TypeKind::Plain),
    ("bytea", TypeKind::Plain),
];

fn type_kind(data_type: &str) -> Option<TypeKind> {
    let base = data_type.strip_suffix("[]").unwrap_or(data_type);
    TYPE_KINDS
        .iter()
        .find(|(name, _)| *name == base)
        .map(|(_, kind)| *kind)
}

/// 字段名与表名同一套白名单规则，后续 SQL 才能安全拼接该标识符
fn validate_field_name(name: &str) -> AppResult<()> {
    validate_table_name(name).map_err(|_| bad_request("无效的字段名"))
}

fn validate_type_spec(
    data_type: &str,
    length: Option<i32>,
    precision: Option<i32>,
    scale: Option<i32>,
) -> AppResult<()> {
    if data_type.is_empty() {
        return if length.is_none() && precision.is_none() && scale.is_none() {
            Ok(())
        } else {
            Err(bad_request("未指定类型时不能填写长度/精度/小数位"))
        };
    }
    let kind =
        type_kind(data_type).ok_or_else(|| bad_request(format!("不支持的类型: {}", data_type)))?;
    let (valid, hint) = match kind {
        TypeKind::Char => (
            precision.is_none()
                && scale.is_none()
                && length.map_or(true, |l| (1..=10_485_760).contains(&l)),
            "字符类型只能填写长度（1–10485760）",
        ),
        TypeKind::Numeric => (
            length.is_none()
                && match precision {
                    Some(p) => {
                        (1..=1000).contains(&p) && scale.map_or(true, |s| (0..=p).contains(&s))
                    }
                    None => scale.is_none(),
                },
            "numeric 只能填写精度（1–1000）和小数位（0–精度），填写小数位须先填精度",
        ),
        TypeKind::Time => (
            length.is_none() && scale.is_none() && precision.map_or(true, |p| (0..=6).contains(&p)),
            "时间类型只能填写精度（秒的小数位 0–6）",
        ),
        TypeKind::Plain => (
            length.is_none() && precision.is_none() && scale.is_none(),
            "该类型不能填写长度/精度/小数位",
        ),
    };
    if valid {
        Ok(())
    } else {
        Err(bad_request(hint))
    }
}

/// 拼出可直接用于 DDL 的完整类型，如 varchar(64)、numeric(12,2)、timestamptz(3)、varchar(20)[]
fn type_expr(
    data_type: &str,
    length: Option<i32>,
    precision: Option<i32>,
    scale: Option<i32>,
) -> String {
    let (base, array) = match data_type.strip_suffix("[]") {
        Some(base) => (base, "[]"),
        None => (data_type, ""),
    };
    let modifier = match (length, precision, scale) {
        (Some(l), _, _) => format!("({})", l),
        (None, Some(p), Some(s)) => format!("({},{})", p, s),
        (None, Some(p), None) => format!("({})", p),
        _ => String::new(),
    };
    format!("{}{}{}", base, modifier, array)
}

fn normalized_type(req: &FieldRootUpsertRequest) -> String {
    req.data_type.as_deref().unwrap_or("").trim().to_lowercase()
}

fn normalized_scale(req: &FieldRootUpsertRequest) -> Option<i32> {
    // PostgreSQL 的 numeric(p) 会将省略的小数位存为 0。
    if matches!(type_kind(&normalized_type(req)), Some(TypeKind::Numeric))
        && req.data_precision.is_some()
    {
        Some(req.data_scale.unwrap_or(0))
    } else {
        req.data_scale
    }
}

fn validate_root_payload(req: &FieldRootUpsertRequest) -> AppResult<()> {
    validate_field_name(req.field_name.trim())?;
    validate_type_spec(
        &normalized_type(req),
        req.data_length,
        req.data_precision,
        req.data_scale,
    )
}

fn quote_ident(ident: &str) -> String {
    format!("\"{}\"", ident.replace('"', "\"\""))
}

fn quote_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// 按字段名实时关联 public 下所有引用列
async fn list_refs(pool: &DbPool, field_names: &[String]) -> AppResult<Vec<ColumnMeta>> {
    sqlx::query_as::<_, ColumnMeta>(&format!(
        "WITH RECURSIVE table_depths AS (
            SELECT c.oid AS relid, 0 AS depth FROM pg_class c
            WHERE c.relkind IN ('r', 'p')
              AND NOT EXISTS (SELECT 1 FROM pg_inherits i WHERE i.inhrelid = c.oid)
            UNION ALL
            SELECT i.inhrelid, d.depth + 1 FROM pg_inherits i
            JOIN table_depths d ON d.relid = i.inhparent
         ), depths AS (SELECT relid, max(depth) AS depth FROM table_depths GROUP BY relid)
         SELECT m.* FROM ({}) m
         JOIN pg_class c ON c.relname::text = m.table_name AND c.relnamespace = 'public'::regnamespace
         LEFT JOIN depths d ON d.relid = c.oid
         WHERE m.field_name = ANY($1)
         ORDER BY m.field_name, COALESCE(d.depth, 0), m.table_name",
        COLUMN_META_SQL
    ))
    .bind(field_names)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询字根引用失败: {}", e)))
}

fn fmt_opt(value: Option<i32>) -> String {
    value.map_or_else(|| "未指定".to_string(), |v| v.to_string())
}

/// 注释差异展示具体值，过长截断，空值显示（空）
fn fmt_comment(value: &str) -> String {
    if value.is_empty() {
        return "（空）".to_string();
    }
    let truncated: String = value.chars().take(20).collect();
    if value.chars().count() > 20 {
        format!("「{}…」", truncated)
    } else {
        format!("「{}」", truncated)
    }
}

fn type_diffs(root: &FieldRootRow, col: &ColumnMeta) -> Vec<String> {
    let mut diffs = Vec::new();
    if col.data_type != root.data_type {
        diffs.push(format!("类型 {} ≠ {}", col.data_type, root.data_type));
    }
    for (label, actual, expected) in [
        ("长度", col.data_length, root.data_length),
        ("精度", col.data_precision, root.data_precision),
        ("小数位", col.data_scale, root.data_scale),
    ] {
        if actual != expected {
            diffs.push(format!(
                "{} {} ≠ {}",
                label,
                fmt_opt(actual),
                fmt_opt(expected)
            ));
        }
    }
    diffs
}

fn build_refs(root: &FieldRootRow, cols: &[ColumnMeta]) -> Vec<FieldRootRef> {
    cols.iter()
        .filter(|c| c.field_name == root.field_name)
        .map(|c| {
            let mut diffs = type_diffs(root, c);
            let type_synced = diffs.is_empty();
            let comment_synced = c.comment == root.comment;
            if !comment_synced {
                diffs.push(format!(
                    "注释 {} ≠ {}",
                    fmt_comment(&c.comment),
                    fmt_comment(&root.comment)
                ));
            }
            FieldRootRef {
                table_name: c.table_name.clone(),
                data_type: c.data_type.clone(),
                data_length: c.data_length,
                data_precision: c.data_precision,
                data_scale: c.data_scale,
                type_expr: type_expr(&c.data_type, c.data_length, c.data_precision, c.data_scale),
                comment: c.comment.clone(),
                type_synced,
                comment_synced,
                synced: type_synced && comment_synced,
                diffs,
            }
        })
        .collect()
}

fn into_root_info(root: FieldRootRow, refs: Vec<FieldRootRef>) -> FieldRootInfo {
    FieldRootInfo {
        type_expr: type_expr(
            &root.data_type,
            root.data_length,
            root.data_precision,
            root.data_scale,
        ),
        id: Some(root.id),
        registered: true,
        field_name: root.field_name,
        data_type: root.data_type,
        data_length: root.data_length,
        data_precision: root.data_precision,
        data_scale: root.data_scale,
        comment: root.comment,
        remark: root.remark,
        updated_at: Some(root.updated_at),
        refs,
    }
}

/// 列表数据源为现有表全部字段（实时关联），field_roots 记录仅作为入库标注
pub async fn list_roots(
    pool: &DbPool,
    page: i64,
    page_size: i64,
    keyword: Option<&str>,
) -> AppResult<(Vec<FieldRootInfo>, i64)> {
    let keyword = keyword.map(escape_like).unwrap_or_default();
    let grouped = format!(
        "SELECT m.field_name, fr.id, fr.data_type, fr.data_length, fr.data_precision,
                fr.data_scale, fr.comment, fr.remark, fr.updated_at
         FROM (SELECT DISTINCT field_name FROM ({}) live) m
         LEFT JOIN field_roots fr ON fr.field_name = m.field_name
         WHERE ($1 = '' OR m.field_name ILIKE '%' || $1 || '%' ESCAPE '\\'
                OR COALESCE(fr.comment, '') ILIKE '%' || $1 || '%' ESCAPE '\\'
                OR COALESCE(fr.remark, '') ILIKE '%' || $1 || '%' ESCAPE '\\')",
        live_columns_sql()
    );

    let total: i64 = sqlx::query_scalar(&format!("SELECT count(*) FROM ({}) t", grouped))
        .bind(&keyword)
        .fetch_one(pool)
        .await
        .map_err(|e| internal_error(format!("统计字段失败: {}", e)))?;

    type LiveRow = (
        String,
        Option<i64>,
        Option<String>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<String>,
        Option<String>,
        Option<chrono::DateTime<Utc>>,
    );
    let rows = sqlx::query_as::<_, LiveRow>(&format!(
        "{} ORDER BY m.field_name LIMIT $2 OFFSET $3",
        grouped
    ))
    .bind(&keyword)
    .bind(page_size)
    .bind((page - 1) * page_size)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询字段失败: {}", e)))?;

    let names: Vec<String> = rows.iter().map(|r| r.0.clone()).collect();
    let cols = list_refs(pool, &names).await?;

    let items = rows
        .into_iter()
        .map(
            |(
                field_name,
                id,
                data_type,
                data_length,
                data_precision,
                data_scale,
                comment,
                remark,
                updated_at,
            )| {
                let registered = id.is_some();
                // 未入库时以首个引用列为展示准绳，差异反映各表之间的分歧
                let (data_type, data_length, data_precision, data_scale, comment) = if registered {
                    (
                        data_type.unwrap_or_default(),
                        data_length,
                        data_precision,
                        data_scale,
                        comment.unwrap_or_default(),
                    )
                } else {
                    cols.iter()
                        .find(|c| c.field_name == field_name)
                        .map(|c| {
                            (
                                c.data_type.clone(),
                                c.data_length,
                                c.data_precision,
                                c.data_scale,
                                c.comment.clone(),
                            )
                        })
                        .unwrap_or_default()
                };
                let pseudo = FieldRootRow {
                    id: id.unwrap_or(0),
                    field_name: field_name.clone(),
                    data_type,
                    data_length,
                    data_precision,
                    data_scale,
                    comment,
                    remark: remark.unwrap_or_default(),
                    created_at: Utc::now(),
                    updated_at: updated_at.unwrap_or_else(Utc::now),
                };
                let refs = build_refs(&pseudo, &cols);
                FieldRootInfo {
                    type_expr: type_expr(
                        &pseudo.data_type,
                        pseudo.data_length,
                        pseudo.data_precision,
                        pseudo.data_scale,
                    ),
                    id,
                    registered,
                    field_name,
                    data_type: pseudo.data_type,
                    data_length: pseudo.data_length,
                    data_precision: pseudo.data_precision,
                    data_scale: pseudo.data_scale,
                    comment: pseudo.comment,
                    remark: pseudo.remark,
                    updated_at,
                    refs,
                }
            },
        )
        .collect();

    Ok((items, total))
}

pub async fn scan_roots(pool: &DbPool) -> AppResult<(i64, i64)> {
    let result = sqlx::query(&format!(
        "INSERT INTO field_roots (field_name, data_type, data_length, data_precision, data_scale, comment)
         SELECT DISTINCT ON (m.field_name)
                m.field_name, m.data_type, m.data_length, m.data_precision, m.data_scale, m.comment
         FROM ({}) m
         WHERE (m.data_length IS NULL OR m.data_length > 0)
           AND (m.data_precision IS NULL OR m.data_precision BETWEEN 0 AND 1000)
           AND (m.data_scale IS NULL OR (m.data_precision IS NOT NULL AND m.data_scale BETWEEN 0 AND m.data_precision))
         ORDER BY m.field_name, (m.comment = ''), m.table_name
         ON CONFLICT (field_name) DO NOTHING",
        live_columns_sql()
    ))
    .execute(pool)
    .await
    .map_err(|e| internal_error(format!("扫描字段入库失败: {}", e)))?;

    let total: i64 = sqlx::query_scalar("SELECT count(*) FROM field_roots")
        .fetch_one(pool)
        .await
        .map_err(|e| internal_error(format!("统计字根失败: {}", e)))?;

    Ok((result.rows_affected() as i64, total))
}

async fn get_root_row(pool: &DbPool, id: i64) -> AppResult<FieldRootRow> {
    sqlx::query_as::<_, FieldRootRow>(&format!(
        "SELECT {} FROM field_roots WHERE id = $1",
        FIELD_ROOT_COLUMNS
    ))
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("查询字根失败: {}", e)))?
    .ok_or_else(|| bad_request("字根不存在"))
}

async fn root_info_with_refs(pool: &DbPool, root: FieldRootRow) -> AppResult<FieldRootInfo> {
    let refs = list_refs(pool, &[root.field_name.clone()]).await?;
    let item_refs = build_refs(&root, &refs);
    Ok(into_root_info(root, item_refs))
}

pub async fn create_root(pool: &DbPool, req: &FieldRootUpsertRequest) -> AppResult<FieldRootInfo> {
    validate_root_payload(req)?;
    let row = sqlx::query_as::<_, FieldRootRow>(&format!(
        "INSERT INTO field_roots (field_name, data_type, data_length, data_precision, data_scale, comment, remark)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING {}",
        FIELD_ROOT_COLUMNS
    ))
    .bind(req.field_name.trim())
    .bind(normalized_type(req))
    .bind(req.data_length)
    .bind(req.data_precision)
    .bind(normalized_scale(req))
    .bind(req.comment.as_deref().unwrap_or("").trim())
    .bind(req.remark.as_deref().unwrap_or("").trim())
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("新增字根失败: {}", e)))?;

    root_info_with_refs(pool, row).await
}

/// 只更新字典记录本身，引用表字段由 sync_root 手动同步
pub async fn update_root(
    pool: &DbPool,
    id: i64,
    req: &FieldRootUpsertRequest,
) -> AppResult<FieldRootInfo> {
    validate_root_payload(req)?;
    let row = sqlx::query_as::<_, FieldRootRow>(&format!(
        "UPDATE field_roots
         SET field_name = $2, data_type = $3, data_length = $4, data_precision = $5,
             data_scale = $6, comment = $7, remark = $8, updated_at = now()
         WHERE id = $1
         RETURNING {}",
        FIELD_ROOT_COLUMNS
    ))
    .bind(id)
    .bind(req.field_name.trim())
    .bind(normalized_type(req))
    .bind(req.data_length)
    .bind(req.data_precision)
    .bind(normalized_scale(req))
    .bind(req.comment.as_deref().unwrap_or("").trim())
    .bind(req.remark.as_deref().unwrap_or("").trim())
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("更新字根失败: {}", e)))?
    .ok_or_else(|| bad_request("字根不存在"))?;

    root_info_with_refs(pool, row).await
}

/// 只删除字典记录，不触碰任何引用表字段
pub async fn delete_root(pool: &DbPool, id: i64) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM field_roots WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| internal_error(format!("删除字根失败: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(bad_request("字根不存在"));
    }
    Ok(())
}

/// 把字根的注释/类型手动同步到所有引用表字段；逐表执行并收集结果，单表失败不影响其他表
pub async fn sync_root(
    pool: &DbPool,
    id: i64,
    sync_comment: bool,
    sync_type: bool,
) -> AppResult<Vec<FieldRootSyncResult>> {
    if !sync_comment && !sync_type {
        return Err(bad_request("请至少选择一项同步内容"));
    }
    let root = get_root_row(pool, id).await?;
    if sync_type {
        if root.data_type.is_empty() {
            return Err(bad_request("字根未指定标准类型，不能同步类型"));
        }
        validate_type_spec(
            &root.data_type,
            root.data_length,
            root.data_precision,
            root.data_scale,
        )?;
    }
    let target_type = type_expr(
        &root.data_type,
        root.data_length,
        root.data_precision,
        root.data_scale,
    );
    let refs = list_refs(pool, &[root.field_name.clone()]).await?;

    let mut results = Vec::new();
    for col in refs {
        // 父表 ALTER TYPE 会递归修改继承列，必须重读子表以免再次直接 ALTER。
        let col = if sync_type && col.inherited {
            sqlx::query_as::<_, ColumnMeta>(&format!(
                "SELECT * FROM ({}) m WHERE m.field_name = $1 AND m.table_name = $2",
                COLUMN_META_SQL
            ))
            .bind(&root.field_name)
            .bind(&col.table_name)
            .fetch_one(pool)
            .await
            .map_err(|e| internal_error(format!("重读继承字段失败: {}", e)))?
        } else {
            col
        };
        let mut statements = Vec::new();
        let mut changes = Vec::new();
        if sync_type {
            let diffs = type_diffs(&root, &col);
            if !diffs.is_empty() {
                statements.push(format!(
                    "ALTER TABLE public.{} ALTER COLUMN {} TYPE {}",
                    quote_ident(&col.table_name),
                    quote_ident(&root.field_name),
                    target_type
                ));
                changes.extend(diffs);
            }
        }
        if sync_comment && col.comment != root.comment {
            statements.push(format!(
                "COMMENT ON COLUMN public.{}.{} IS {}",
                quote_ident(&col.table_name),
                quote_ident(&root.field_name),
                quote_literal(&root.comment)
            ));
            changes.push("注释更新".to_string());
        }

        if statements.is_empty() {
            results.push(FieldRootSyncResult {
                table_name: col.table_name,
                ok: true,
                message: "所选属性已一致，无需同步".to_string(),
            });
            continue;
        }

        let mut tx = pool
            .begin()
            .await
            .map_err(|e| internal_error(format!("启动同步事务失败: {}", e)))?;
        let mut error = None;
        for stmt in &statements {
            if let Err(e) = sqlx::query(stmt).execute(&mut *tx).await {
                error = Some(e.to_string());
                break;
            }
        }
        let result = match error {
            Some(e) => {
                tx.rollback()
                    .await
                    .map_err(|e| internal_error(format!("回滚同步事务失败: {}", e)))?;
                Err(e)
            }
            None => tx.commit().await.map_err(|e| e.to_string()),
        };
        results.push(match result {
            Ok(()) => FieldRootSyncResult {
                table_name: col.table_name,
                ok: true,
                message: format!("同步成功：{}", changes.join("；")),
            },
            Err(e) => FieldRootSyncResult {
                table_name: col.table_name,
                ok: false,
                message: e,
            },
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_type_modifiers() {
        for (name, length, precision, scale) in [
            ("varchar", Some(64), None, None),
            ("bpchar[]", Some(20), None, None),
            ("numeric", None, Some(12), Some(2)),
            ("numeric[]", None, Some(12), None),
            ("timestamptz", None, Some(0), None),
            ("interval", None, Some(6), None),
            ("text[]", None, None, None),
            ("", None, None, None),
        ] {
            assert!(
                validate_type_spec(name, length, precision, scale).is_ok(),
                "{name}"
            );
        }
        for (name, length, precision, scale) in [
            ("varchar(64)", None, None, None),
            ("text; DROP TABLE users", None, None, None),
            ("text[][]", None, None, None),
            ("varchar", Some(0), None, None),
            ("int8", Some(64), None, None),
            ("numeric", None, None, Some(2)),
            ("numeric", None, Some(0), None),
            ("numeric", None, Some(12), Some(13)),
            ("timestamptz", None, Some(7), None),
            ("", Some(20), None, None),
        ] {
            assert!(
                validate_type_spec(name, length, precision, scale).is_err(),
                "{name}"
            );
        }
    }

    #[test]
    fn formats_type_expressions() {
        assert_eq!(type_expr("varchar", Some(64), None, None), "varchar(64)");
        assert_eq!(
            type_expr("numeric[]", None, Some(12), Some(2)),
            "numeric(12,2)[]"
        );
        assert_eq!(
            type_expr("timestamptz", None, Some(0), None),
            "timestamptz(0)"
        );
        assert_eq!(type_expr("text[]", None, None, None), "text[]");
    }

    #[test]
    fn normalizes_numeric_default_scale() {
        let req = FieldRootUpsertRequest {
            field_name: "amount".into(),
            data_type: Some(" NUMERIC[] ".into()),
            data_length: None,
            data_precision: Some(12),
            data_scale: None,
            comment: None,
            remark: None,
        };
        assert_eq!(normalized_type(&req), "numeric[]");
        assert_eq!(normalized_scale(&req), Some(0));
    }

    #[test]
    fn compares_attributes_and_comment_independently() {
        let root = FieldRootRow {
            id: 1,
            field_name: "amount".into(),
            data_type: "numeric".into(),
            data_length: None,
            data_precision: Some(12),
            data_scale: Some(2),
            comment: "金额".into(),
            remark: String::new(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let mut col = ColumnMeta {
            field_name: "amount".into(),
            table_name: "orders".into(),
            data_type: "numeric".into(),
            data_length: None,
            data_precision: Some(10),
            data_scale: Some(0),
            comment: "金额".into(),
            inherited: false,
        };
        let refs = build_refs(&root, std::slice::from_ref(&col));
        assert_eq!(refs[0].diffs, ["精度 10 ≠ 12", "小数位 0 ≠ 2"]);
        assert!(refs[0].comment_synced && !refs[0].type_synced);
        col.data_precision = Some(12);
        col.data_scale = Some(2);
        col.comment.clear();
        let refs = build_refs(&root, &[col]);
        assert_eq!(refs[0].diffs, ["注释 （空） ≠ 「金额」"]);
        assert!(refs[0].type_synced && !refs[0].comment_synced && !refs[0].synced);
    }
}

// ===================== 服务状态 =====================

pub async fn service_status(pool: Option<&DbPool>) -> AdminServiceInfo {
    let db_connected = match pool {
        Some(p) => db::ping(p).await.is_ok(),
        None => false,
    };

    AdminServiceInfo {
        name: SETTINGS.app_name.clone(),
        version: SETTINGS.app_version.clone(),
        description: SETTINGS.app_description.clone(),
        status: "running".to_string(),
        db_connected,
        uptime_seconds: START_TIME.elapsed().as_secs() as i64,
        timestamp: chrono::Local::now().to_rfc3339(),
    }
}
