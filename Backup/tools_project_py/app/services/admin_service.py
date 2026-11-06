"""
后台管理服务（字段数据字典/表浏览/服务状态）
逻辑与 Rust 端 services/admin.rs 对齐，响应结构保持一致
"""
from __future__ import annotations

import time
from typing import Any, Dict, List, Optional, Tuple

import asyncpg

from app.core.config import settings
from app.core.db import get_pool

START_TIME = time.monotonic()

TECH_FIELDS = ("id", "created_at", "updated_at")

DEFAULT_PAGE_SIZE = 20
MAX_PAGE_SIZE = 200


class AdminError(Exception):
    def __init__(self, message: str, status_code: int = 500):
        super().__init__(message)
        self.message = message
        self.status_code = status_code


def bad_request(message: str) -> AdminError:
    return AdminError(message, 400)


def internal_error(message: str) -> AdminError:
    return AdminError(message, 500)


async def require_pool() -> asyncpg.Pool:
    pool = await get_pool()
    if pool is None:
        raise internal_error("数据库连接池未初始化")
    return pool


def validate_table_name(table: str) -> None:
    valid = (
        table
        and len(table) <= 63
        and all(
            c.islower() or c == "_" or (i > 0 and c.isdigit())
            for i, c in enumerate(table)
        )
    )
    if not valid:
        raise bad_request("无效的表名")


def validate_field_name(name: str) -> None:
    try:
        validate_table_name(name)
    except AdminError:
        raise bad_request("无效的字段名")


def escape_like(keyword: str) -> str:
    return keyword.replace("\\", "\\\\").replace("%", "\\%").replace("_", "\\_")


def quote_ident(ident: str) -> str:
    return '"{}"'.format(ident.replace('"', '""'))


def quote_literal(value: str) -> str:
    return "'{}'".format(value.replace("'", "''"))


async def ensure_table_exists(pool: asyncpg.Pool, table: str) -> None:
    validate_table_name(table)
    exists = await pool.fetchval(
        "SELECT EXISTS ("
        " SELECT 1 FROM information_schema.tables"
        " WHERE table_schema = 'public' AND table_name = $1"
        ")",
        table,
    )
    if not exists:
        raise bad_request(f"数据表不存在: {table}")


# ===================== 数据库表浏览 =====================

async def list_tables(pool: asyncpg.Pool) -> List[Dict[str, Any]]:
    rows = await pool.fetch(
        "SELECT c.relname, obj_description(c.oid, 'pg_class'), c.reltuples::bigint"
        " FROM pg_class c"
        " JOIN pg_namespace n ON n.oid = c.relnamespace"
        " WHERE c.relkind = 'r' AND n.nspname = 'public'"
        " ORDER BY c.relname"
    )
    return [
        {"name": r[0], "comment": r[1], "row_estimate": r[2]}
        for r in rows
    ]


async def list_columns(pool: asyncpg.Pool, table: str) -> List[Dict[str, Any]]:
    await ensure_table_exists(pool, table)
    rows = await pool.fetch(
        "SELECT c.column_name,"
        "       c.data_type,"
        "       c.is_nullable = 'YES',"
        "       EXISTS ("
        "         SELECT 1"
        "         FROM pg_index i"
        "         JOIN pg_attribute a ON a.attrelid = i.indrelid AND a.attnum = ANY (i.indkey)"
        "         WHERE i.indrelid = format('%I.%I', c.table_schema, c.table_name)::regclass"
        "           AND i.indisprimary"
        "           AND a.attname = c.column_name"
        "       ),"
        "       c.column_default,"
        "       pg_catalog.col_description("
        "         format('%I.%I', c.table_schema, c.table_name)::regclass,"
        "         c.ordinal_position"
        "       )"
        " FROM information_schema.columns c"
        " WHERE c.table_schema = 'public' AND c.table_name = $1"
        " ORDER BY c.ordinal_position",
        table,
    )
    return [
        {
            "name": r[0],
            "data_type": r[1],
            "nullable": r[2],
            "is_primary": r[3],
            "column_default": r[4],
            "comment": r[5],
        }
        for r in rows
    ]


async def list_rows(
    pool: asyncpg.Pool, table: str, page: int, page_size: int, keyword: Optional[str]
) -> Dict[str, Any]:
    await ensure_table_exists(pool, table)
    kw = escape_like(keyword) if keyword else ""
    # 表名已通过白名单校验与存在性确认；$1 为空串时不过滤
    condition = "($1 = '' OR to_jsonb(t)::text ILIKE '%' || $1 || '%' ESCAPE '\\')"

    total = await pool.fetchval(
        f'SELECT count(*) FROM "{table}" t WHERE {condition}', kw
    )
    columns_rows = await pool.fetch(
        "SELECT column_name FROM information_schema.columns"
        " WHERE table_schema = 'public' AND table_name = $1"
        " ORDER BY ordinal_position",
        table,
    )
    rows = await pool.fetch(
        f'SELECT to_jsonb(t) FROM "{table}" t WHERE {condition} LIMIT $2 OFFSET $3',
        kw,
        page_size,
        (page - 1) * page_size,
    )
    return {
        "columns": [r[0] for r in columns_rows],
        "rows": [r[0] for r in rows],
        "total": total,
    }


# ===================== 字根管理（字段数据字典） =====================

# typmod 中字符长度带 4 字节偏移，numeric 小数位为有符号 11 位。
COLUMN_META_SQL = """
SELECT a.attname::text AS field_name, c.relname::text AS table_name,
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
      AND NOT a.attisdropped AND c.relname <> 'field_roots'
"""

FIELD_ROOT_COLUMNS = (
    "id, field_name, data_type, data_length, data_precision, data_scale, "
    "comment, remark, created_at, updated_at"
)

# 类型白名单：同步时类型名会直接拼进 ALTER 语句，只允许这里列出的名称（可带一层 [] 数组后缀）
CHAR_TYPES = ("varchar", "bpchar")
NUMERIC_TYPES = ("numeric",)
TIME_TYPES = ("time", "timetz", "timestamp", "timestamptz", "interval")
PLAIN_TYPES = (
    "text", "int2", "int4", "int8", "float4", "float8", "bool", "date",
    "uuid", "json", "jsonb", "inet", "bytea",
)
ALL_TYPES = CHAR_TYPES + NUMERIC_TYPES + TIME_TYPES + PLAIN_TYPES


def _base_type(data_type: str) -> str:
    return data_type[:-2] if data_type.endswith("[]") else data_type


def validate_type_spec(
    data_type: str,
    length: Optional[int],
    precision: Optional[int],
    scale: Optional[int],
) -> None:
    if not data_type:
        if length is None and precision is None and scale is None:
            return
        raise bad_request("未指定类型时不能填写长度/精度/小数位")
    base = _base_type(data_type)
    if base not in ALL_TYPES:
        raise bad_request(f"不支持的类型: {data_type}")
    if base in CHAR_TYPES:
        valid = precision is None and scale is None and (
            length is None or 1 <= length <= 10485760
        )
        hint = "字符类型只能填写长度（1–10485760）"
    elif base in NUMERIC_TYPES:
        valid = length is None and (
            (precision is not None and 1 <= precision <= 1000 and (scale is None or 0 <= scale <= precision))
            or (precision is None and scale is None)
        )
        hint = "numeric 只能填写精度（1–1000）和小数位（0–精度），填写小数位须先填精度"
    elif base in TIME_TYPES:
        valid = length is None and scale is None and (
            precision is None or 0 <= precision <= 6
        )
        hint = "时间类型只能填写精度（秒的小数位 0–6）"
    else:
        valid = length is None and precision is None and scale is None
        hint = "该类型不能填写长度/精度/小数位"
    if not valid:
        raise bad_request(hint)


def type_expr(
    data_type: str,
    length: Optional[int],
    precision: Optional[int],
    scale: Optional[int],
) -> str:
    """拼出可直接用于 DDL 的完整类型，如 varchar(64)、numeric(12,2)、timestamptz(3)、varchar(20)[]"""
    if data_type.endswith("[]"):
        base, array = data_type[:-2], "[]"
    else:
        base, array = data_type, ""
    if length is not None:
        modifier = f"({length})"
    elif precision is not None and scale is not None:
        modifier = f"({precision},{scale})"
    elif precision is not None:
        modifier = f"({precision})"
    else:
        modifier = ""
    return f"{base}{modifier}{array}"


def normalized_scale(
    data_type: str, precision: Optional[int], scale: Optional[int]
) -> Optional[int]:
    # PostgreSQL 的 numeric(p) 会将省略的小数位存为 0。
    if _base_type(data_type) in NUMERIC_TYPES and precision is not None:
        return scale if scale is not None else 0
    return scale


def validate_root_payload(payload: Dict[str, Any]) -> Dict[str, Any]:
    field_name = (payload.get("field_name") or "").strip()
    validate_field_name(field_name)
    data_type = (payload.get("data_type") or "").strip().lower()
    length = payload.get("data_length")
    precision = payload.get("data_precision")
    scale = payload.get("data_scale")
    validate_type_spec(data_type, length, precision, scale)
    return {
        "field_name": field_name,
        "data_type": data_type,
        "data_length": length,
        "data_precision": precision,
        "data_scale": normalized_scale(data_type, precision, scale),
        "comment": (payload.get("comment") or "").strip(),
        "remark": (payload.get("remark") or "").strip(),
    }


async def list_refs(pool: asyncpg.Pool, field_names: List[str]) -> List[Dict[str, Any]]:
    """按字段名实时关联 public 下所有引用列（含继承深度排序）"""
    if not field_names:
        return []
    rows = await pool.fetch(
        f"""WITH RECURSIVE table_depths AS (
            SELECT c.oid AS relid, 0 AS depth FROM pg_class c
            WHERE c.relkind IN ('r', 'p')
              AND NOT EXISTS (SELECT 1 FROM pg_inherits i WHERE i.inhrelid = c.oid)
            UNION ALL
            SELECT i.inhrelid, d.depth + 1 FROM pg_inherits i
            JOIN table_depths d ON d.relid = i.inhparent
         ), depths AS (SELECT relid, max(depth) AS depth FROM table_depths GROUP BY relid)
         SELECT m.* FROM ({COLUMN_META_SQL}) m
         JOIN pg_class c ON c.relname::text = m.table_name AND c.relnamespace = 'public'::regnamespace
         LEFT JOIN depths d ON d.relid = c.oid
         WHERE m.field_name = ANY($1)
         ORDER BY m.field_name, COALESCE(d.depth, 0), m.table_name""",
        field_names,
    )
    return [dict(r) for r in rows]


def _fmt_opt(value: Optional[int]) -> str:
    return "未指定" if value is None else str(value)


def _fmt_comment(value: str) -> str:
    """注释差异展示具体值，过长截断，空值显示（空）"""
    if not value:
        return "（空）"
    truncated = value[:20]
    return f"「{truncated}…」" if len(value) > 20 else f"「{truncated}」"


def _type_diffs(root: Dict[str, Any], col: Dict[str, Any]) -> List[str]:
    diffs = []
    if col["data_type"] != root["data_type"]:
        diffs.append(f"类型 {col['data_type']} ≠ {root['data_type']}")
    for label, key in (("长度", "data_length"), ("精度", "data_precision"), ("小数位", "data_scale")):
        if col[key] != root[key]:
            diffs.append(f"{label} {_fmt_opt(col[key])} ≠ {_fmt_opt(root[key])}")
    return diffs


def build_refs(root: Dict[str, Any], cols: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    refs = []
    for c in cols:
        if c["field_name"] != root["field_name"]:
            continue
        diffs = _type_diffs(root, c)
        type_synced = not diffs
        comment_synced = c["comment"] == root["comment"]
        if not comment_synced:
            diffs.append(f"注释 {_fmt_comment(c['comment'])} ≠ {_fmt_comment(root['comment'])}")
        refs.append({
            "table_name": c["table_name"],
            "data_type": c["data_type"],
            "data_length": c["data_length"],
            "data_precision": c["data_precision"],
            "data_scale": c["data_scale"],
            "type_expr": type_expr(c["data_type"], c["data_length"], c["data_precision"], c["data_scale"]),
            "comment": c["comment"],
            "type_synced": type_synced,
            "comment_synced": comment_synced,
            "synced": type_synced and comment_synced,
            "diffs": diffs,
        })
    return refs


def _live_columns_sql() -> str:
    tech = ", ".join(f"'{f}'" for f in TECH_FIELDS)
    return f"SELECT * FROM ({COLUMN_META_SQL}) m WHERE m.field_name NOT IN ({tech})"


async def list_roots(
    pool: asyncpg.Pool, page: int, page_size: int, keyword: Optional[str]
) -> Tuple[List[Dict[str, Any]], int]:
    """列表数据源为现有表全部字段（实时关联），field_roots 记录仅作为入库标注"""
    kw = escape_like(keyword) if keyword else ""
    grouped = f"""
        SELECT m.field_name, fr.id, fr.data_type, fr.data_length, fr.data_precision,
               fr.data_scale, fr.comment, fr.remark, fr.updated_at
        FROM (SELECT DISTINCT field_name FROM ({_live_columns_sql()}) live) m
        LEFT JOIN field_roots fr ON fr.field_name = m.field_name
        WHERE ($1 = '' OR m.field_name ILIKE '%' || $1 || '%' ESCAPE '\\'
               OR COALESCE(fr.comment, '') ILIKE '%' || $1 || '%' ESCAPE '\\'
               OR COALESCE(fr.remark, '') ILIKE '%' || $1 || '%' ESCAPE '\\')
    """
    total = await pool.fetchval(f"SELECT count(*) FROM ({grouped}) t", kw)
    rows = await pool.fetch(
        f"{grouped} ORDER BY m.field_name LIMIT $2 OFFSET $3",
        kw,
        page_size,
        (page - 1) * page_size,
    )

    names = [r["field_name"] for r in rows]
    cols = await list_refs(pool, names)

    items = []
    for r in rows:
        registered = r["id"] is not None
        if registered:
            root = {
                "field_name": r["field_name"],
                "data_type": r["data_type"] or "",
                "data_length": r["data_length"],
                "data_precision": r["data_precision"],
                "data_scale": r["data_scale"],
                "comment": r["comment"] or "",
            }
        else:
            # 未入库时以首个引用列为展示准绳，差异反映各表之间的分歧
            first = next((c for c in cols if c["field_name"] == r["field_name"]), None)
            root = {
                "field_name": r["field_name"],
                "data_type": first["data_type"] if first else "",
                "data_length": first["data_length"] if first else None,
                "data_precision": first["data_precision"] if first else None,
                "data_scale": first["data_scale"] if first else None,
                "comment": first["comment"] if first else "",
            }
        refs = build_refs(root, cols)
        items.append({
            "id": r["id"],
            "registered": registered,
            "field_name": r["field_name"],
            "data_type": root["data_type"],
            "data_length": root["data_length"],
            "data_precision": root["data_precision"],
            "data_scale": root["data_scale"],
            "type_expr": type_expr(
                root["data_type"], root["data_length"], root["data_precision"], root["data_scale"]
            ),
            "comment": root["comment"],
            "remark": r["remark"] or "",
            "updated_at": r["updated_at"],
            "refs": refs,
        })
    return items, total


async def scan_roots(pool: asyncpg.Pool) -> Tuple[int, int]:
    """扫描 public 全部字段入库（排除通用技术字段与字典自身），已存在的不覆盖"""
    result = await pool.execute(
        f"""INSERT INTO field_roots (field_name, data_type, data_length, data_precision, data_scale, comment)
         SELECT DISTINCT ON (m.field_name)
                m.field_name, m.data_type, m.data_length, m.data_precision, m.data_scale, m.comment
         FROM ({_live_columns_sql()}) m
         WHERE (m.data_length IS NULL OR m.data_length > 0)
           AND (m.data_precision IS NULL OR m.data_precision BETWEEN 0 AND 1000)
           AND (m.data_scale IS NULL OR (m.data_precision IS NOT NULL AND m.data_scale BETWEEN 0 AND m.data_precision))
         ORDER BY m.field_name, (m.comment = ''), m.table_name
         ON CONFLICT (field_name) DO NOTHING"""
    )
    inserted = int(result.split()[-1]) if result else 0
    total = await pool.fetchval("SELECT count(*) FROM field_roots")
    return inserted, total


async def get_root_row(pool: asyncpg.Pool, root_id: int) -> Dict[str, Any]:
    row = await pool.fetchrow(
        f"SELECT {FIELD_ROOT_COLUMNS} FROM field_roots WHERE id = $1", root_id
    )
    if row is None:
        raise bad_request("字根不存在")
    return dict(row)


async def _root_info_with_refs(pool: asyncpg.Pool, root: Dict[str, Any]) -> Dict[str, Any]:
    cols = await list_refs(pool, [root["field_name"]])
    refs = build_refs(root, cols)
    return {
        "id": root["id"],
        "registered": True,
        "field_name": root["field_name"],
        "data_type": root["data_type"],
        "data_length": root["data_length"],
        "data_precision": root["data_precision"],
        "data_scale": root["data_scale"],
        "type_expr": type_expr(
            root["data_type"], root["data_length"], root["data_precision"], root["data_scale"]
        ),
        "comment": root["comment"],
        "remark": root["remark"],
        "updated_at": root["updated_at"],
        "refs": refs,
    }


async def create_root(pool: asyncpg.Pool, payload: Dict[str, Any]) -> Dict[str, Any]:
    data = validate_root_payload(payload)
    row = await pool.fetchrow(
        f"""INSERT INTO field_roots (field_name, data_type, data_length, data_precision, data_scale, comment, remark)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         RETURNING {FIELD_ROOT_COLUMNS}""",
        data["field_name"], data["data_type"], data["data_length"],
        data["data_precision"], data["data_scale"], data["comment"], data["remark"],
    )
    return await _root_info_with_refs(pool, dict(row))


async def update_root(pool: asyncpg.Pool, root_id: int, payload: Dict[str, Any]) -> Dict[str, Any]:
    """只更新字典记录本身，引用表字段由 sync_root 手动同步"""
    data = validate_root_payload(payload)
    row = await pool.fetchrow(
        f"""UPDATE field_roots
         SET field_name = $2, data_type = $3, data_length = $4, data_precision = $5,
             data_scale = $6, comment = $7, remark = $8, updated_at = now()
         WHERE id = $1
         RETURNING {FIELD_ROOT_COLUMNS}""",
        root_id, data["field_name"], data["data_type"], data["data_length"],
        data["data_precision"], data["data_scale"], data["comment"], data["remark"],
    )
    if row is None:
        raise bad_request("字根不存在")
    return await _root_info_with_refs(pool, dict(row))


async def delete_root(pool: asyncpg.Pool, root_id: int) -> None:
    """只删除字典记录，不触碰任何引用表字段"""
    result = await pool.execute("DELETE FROM field_roots WHERE id = $1", root_id)
    if result and result.split()[-1] == "0":
        raise bad_request("字根不存在")


async def sync_root(
    pool: asyncpg.Pool, root_id: int, sync_comment: bool, sync_type: bool
) -> List[Dict[str, Any]]:
    """把字根的注释/类型手动同步到所有引用表字段；逐表独立事务，单表失败不影响其他表"""
    if not sync_comment and not sync_type:
        raise bad_request("请至少选择一项同步内容")
    root = await get_root_row(pool, root_id)
    if sync_type:
        if not root["data_type"]:
            raise bad_request("字根未指定标准类型，不能同步类型")
        validate_type_spec(
            root["data_type"], root["data_length"], root["data_precision"], root["data_scale"]
        )
    target_type = type_expr(
        root["data_type"], root["data_length"], root["data_precision"], root["data_scale"]
    )
    cols = await list_refs(pool, [root["field_name"]])

    results = []
    async with pool.acquire() as conn:
        for col in cols:
            # 父表 ALTER TYPE 会递归修改继承列，必须重读子表以免再次直接 ALTER。
            if sync_type and col["inherited"]:
                refreshed = await conn.fetchrow(
                    f"SELECT * FROM ({COLUMN_META_SQL}) m WHERE m.field_name = $1 AND m.table_name = $2",
                    root["field_name"], col["table_name"],
                )
                if refreshed is not None:
                    col = dict(refreshed)

            statements = []
            changes = []
            if sync_type:
                diffs = _type_diffs(root, col)
                if diffs:
                    statements.append(
                        f"ALTER TABLE public.{quote_ident(col['table_name'])}"
                        f" ALTER COLUMN {quote_ident(root['field_name'])} TYPE {target_type}"
                    )
                    changes.extend(diffs)
            if sync_comment and col["comment"] != root["comment"]:
                statements.append(
                    f"COMMENT ON COLUMN public.{quote_ident(col['table_name'])}.{quote_ident(root['field_name'])}"
                    f" IS {quote_literal(root['comment'])}"
                )
                changes.append("注释更新")

            if not statements:
                results.append({
                    "table_name": col["table_name"],
                    "ok": True,
                    "message": "所选属性已一致，无需同步",
                })
                continue

            try:
                async with conn.transaction():
                    for stmt in statements:
                        await conn.execute(stmt)
                results.append({
                    "table_name": col["table_name"],
                    "ok": True,
                    "message": f"同步成功：{'；'.join(changes)}",
                })
            except Exception as e:
                results.append({
                    "table_name": col["table_name"],
                    "ok": False,
                    "message": str(e),
                })
    return results


# ===================== 服务状态 =====================

async def service_status() -> Dict[str, Any]:
    pool = await get_pool()
    db_connected = False
    if pool is not None:
        try:
            await pool.fetchval("SELECT 1")
            db_connected = True
        except Exception:
            db_connected = False

    from datetime import datetime, timezone
    return {
        "name": f"{settings.app_name} (Python)",
        "version": settings.app_version,
        "description": settings.app_description,
        "status": "running",
        "db_connected": db_connected,
        "uptime_seconds": int(time.monotonic() - START_TIME),
        "timestamp": datetime.now(timezone.utc).astimezone().isoformat(),
    }
