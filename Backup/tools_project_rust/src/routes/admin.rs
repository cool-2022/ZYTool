use axum::{
    extract::{Path, Query},
    http::StatusCode,
    routing::{get, post, put},
    Extension, Json, Router,
};

use crate::core::auth::CurrentUser;
use crate::core::db::{get_pool, DbPool};
use crate::core::error::{internal_error, AppError, AppResult};
use crate::models::admin::{
    AdminColumnsResponse, AdminRowsQuery, AdminRowsResponse, AdminServicesResponse,
    AdminTablesResponse, FieldRootResponse, FieldRootSyncRequest, FieldRootSyncResponse,
    FieldRootUpsertRequest, FieldRootsQuery, FieldRootsResponse, FieldRootsScanResponse,
};
use crate::models::{BaseInfo, BaseResponse};
use crate::services::admin as admin_service;

const DEFAULT_PAGE_SIZE: i64 = 20;
const MAX_PAGE_SIZE: i64 = 200;

pub fn router() -> Router {
    Router::new()
        .route("/tables", get(list_tables))
        .route("/tables/{table}/columns", get(list_columns))
        .route("/tables/{table}/rows", get(list_rows))
        .route("/roots", get(list_roots).post(create_root))
        .route("/roots/scan", post(scan_roots))
        .route("/roots/{id}", put(update_root).delete(delete_root))
        .route("/roots/{id}/sync", post(sync_root))
        .route("/services", get(service_status))
}

fn require_admin(current_user: &CurrentUser) -> AppResult<()> {
    if current_user.roles.contains(&"admin".to_string()) {
        Ok(())
    } else {
        Err(AppError::new(
            "权限不足，仅管理员可访问",
            StatusCode::FORBIDDEN,
        ))
    }
}

fn require_pool() -> AppResult<&'static DbPool> {
    get_pool().ok_or_else(|| internal_error("数据库连接池未初始化"))
}

async fn list_tables(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<AdminTablesResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let tables = admin_service::list_tables(pool).await?;

    Ok(Json(BaseResponse::ok(AdminTablesResponse { tables }, base)))
}

async fn list_columns(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(table): Path<String>,
) -> AppResult<Json<BaseResponse<AdminColumnsResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let columns = admin_service::list_columns(pool, &table).await?;

    Ok(Json(BaseResponse::ok(
        AdminColumnsResponse { table, columns },
        base,
    )))
}

async fn list_rows(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(table): Path<String>,
    Query(query): Query<AdminRowsQuery>,
) -> AppResult<Json<BaseResponse<AdminRowsResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;

    let page = query.page.unwrap_or(1).max(1);
    let page_size = query
        .page_size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE);

    let data = admin_service::list_rows(pool, &table, page, page_size, query.keyword.as_deref())
        .await?;

    Ok(Json(BaseResponse::ok(
        AdminRowsResponse {
            table,
            columns: data.columns,
            rows: data.rows,
            total: data.total,
            page,
            page_size,
        },
        base,
    )))
}

// ===================== 字根管理（字段数据字典） =====================

fn normalize_page(page: Option<i64>, page_size: Option<i64>) -> (i64, i64) {
    (
        page.unwrap_or(1).max(1),
        page_size.unwrap_or(DEFAULT_PAGE_SIZE).clamp(1, MAX_PAGE_SIZE),
    )
}

async fn list_roots(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Query(query): Query<FieldRootsQuery>,
) -> AppResult<Json<BaseResponse<FieldRootsResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let (page, page_size) = normalize_page(query.page, query.page_size);
    let (rows, total) =
        admin_service::list_roots(pool, page, page_size, query.keyword.as_deref()).await?;

    Ok(Json(BaseResponse::ok(
        FieldRootsResponse {
            rows,
            total,
            page,
            page_size,
        },
        base,
    )))
}

async fn scan_roots(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<FieldRootsScanResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let (inserted, total_roots) = admin_service::scan_roots(pool).await?;

    Ok(Json(BaseResponse::ok(
        FieldRootsScanResponse {
            inserted,
            total_roots,
        },
        base,
    )))
}

async fn create_root(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Json(payload): Json<FieldRootUpsertRequest>,
) -> AppResult<Json<BaseResponse<FieldRootResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let root_item = admin_service::create_root(pool, &payload).await?;

    Ok(Json(BaseResponse::ok(
        FieldRootResponse { root_item },
        base,
    )))
}

async fn update_root(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(id): Path<i64>,
    Json(payload): Json<FieldRootUpsertRequest>,
) -> AppResult<Json<BaseResponse<FieldRootResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let root_item = admin_service::update_root(pool, id, &payload).await?;

    Ok(Json(BaseResponse::ok(
        FieldRootResponse { root_item },
        base,
    )))
}

async fn delete_root(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(id): Path<i64>,
) -> AppResult<Json<BaseResponse<()>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    admin_service::delete_root(pool, id).await?;

    Ok(Json(BaseResponse::ok((), base)))
}

async fn sync_root(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
    Path(id): Path<i64>,
    Json(payload): Json<FieldRootSyncRequest>,
) -> AppResult<Json<BaseResponse<FieldRootSyncResponse>>> {
    require_admin(&current_user)?;
    let pool = require_pool()?;
    let results = admin_service::sync_root(
        pool,
        id,
        payload.sync_comment.unwrap_or(false),
        payload.sync_type.unwrap_or(false),
    )
    .await?;

    Ok(Json(BaseResponse::ok(FieldRootSyncResponse { results }, base)))
}

// ===================== 服务状态 =====================

async fn service_status(
    current_user: CurrentUser,
    Extension(base): Extension<Option<BaseInfo>>,
) -> AppResult<Json<BaseResponse<AdminServicesResponse>>> {
    require_admin(&current_user)?;
    let service = admin_service::service_status(get_pool()).await;

    Ok(Json(BaseResponse::ok(AdminServicesResponse { service }, base)))
}
