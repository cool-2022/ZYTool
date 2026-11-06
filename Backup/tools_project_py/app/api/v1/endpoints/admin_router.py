"""
后台管理路由
与 Rust 端 routes/admin.rs 路径与响应结构对齐；均要求 JWT 且 roles 含 admin
"""

from fastapi import APIRouter, Depends, Query
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from app.core.auth import get_current_user
from app.services import admin_service
from app.services.admin_service import AdminError

router = APIRouter(prefix="/admin", tags=["admin"])

DEFAULT_PAGE_SIZE = 20
MAX_PAGE_SIZE = 200


def ok(data, message: str = "操作成功"):
    return JSONResponse(jsonable_encoder({"success": True, "message": message, "data": data}))


def fail(status_code: int, message: str):
    return JSONResponse(
        status_code=status_code,
        content={"success": False, "message": message, "data": {"details": {}, "status_code": status_code}},
    )


def require_admin(current_user: dict):
    if "admin" not in current_user.get("roles", []):
        return fail(403, "权限不足，仅管理员可访问")
    return None


def normalize_page(page: int, page_size: int):
    return max(1, page), min(max(1, page_size), MAX_PAGE_SIZE)


# ===================== 数据库表浏览 =====================

@router.get("/tables")
async def list_tables(current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        tables = await admin_service.list_tables(pool)
        return ok({"tables": tables})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.get("/tables/{table}/columns")
async def list_columns(table: str, current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        columns = await admin_service.list_columns(pool, table)
        return ok({"table": table, "columns": columns})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.get("/tables/{table}/rows")
async def list_rows(
    table: str,
    page: int = Query(1),
    page_size: int = Query(DEFAULT_PAGE_SIZE),
    keyword: str = Query(None),
    current_user: dict = Depends(get_current_user),
):
    if (err := require_admin(current_user)) is not None:
        return err
    page, page_size = normalize_page(page, page_size)
    try:
        pool = await admin_service.require_pool()
        data = await admin_service.list_rows(pool, table, page, page_size, keyword)
        return ok({
            "table": table,
            "columns": data["columns"],
            "rows": data["rows"],
            "total": data["total"],
            "page": page,
            "page_size": page_size,
        })
    except AdminError as e:
        return fail(e.status_code, e.message)


# ===================== 字根管理（字段数据字典） =====================

@router.get("/roots")
async def list_roots(
    page: int = Query(1),
    page_size: int = Query(DEFAULT_PAGE_SIZE),
    keyword: str = Query(None),
    current_user: dict = Depends(get_current_user),
):
    if (err := require_admin(current_user)) is not None:
        return err
    page, page_size = normalize_page(page, page_size)
    try:
        pool = await admin_service.require_pool()
        rows, total = await admin_service.list_roots(pool, page, page_size, keyword)
        return ok({"rows": rows, "total": total, "page": page, "page_size": page_size})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.post("/roots/scan")
async def scan_roots(current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        inserted, total_roots = await admin_service.scan_roots(pool)
        return ok({"inserted": inserted, "total_roots": total_roots})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.post("/roots")
async def create_root(payload: dict, current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        root_item = await admin_service.create_root(pool, payload)
        return ok({"root_item": root_item})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.put("/roots/{root_id}")
async def update_root(root_id: int, payload: dict, current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        root_item = await admin_service.update_root(pool, root_id, payload)
        return ok({"root_item": root_item})
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.delete("/roots/{root_id}")
async def delete_root(root_id: int, current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        await admin_service.delete_root(pool, root_id)
        return ok(None)
    except AdminError as e:
        return fail(e.status_code, e.message)


@router.post("/roots/{root_id}/sync")
async def sync_root(root_id: int, payload: dict, current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    try:
        pool = await admin_service.require_pool()
        results = await admin_service.sync_root(
            pool,
            root_id,
            bool(payload.get("sync_comment", False)),
            bool(payload.get("sync_type", False)),
        )
        return ok({"results": results})
    except AdminError as e:
        return fail(e.status_code, e.message)


# ===================== 服务状态 =====================

@router.get("/services")
async def service_status(current_user: dict = Depends(get_current_user)):
    if (err := require_admin(current_user)) is not None:
        return err
    service = await admin_service.service_status()
    return ok({"service": service})
