from fastapi import APIRouter

from app.core.db import get_pool


router = APIRouter(tags=["misc"])


async def _load_categories_from_db():
    """工具分类优先读库（tool_categories/tools 表），失败返回 None 由调用方回退"""
    pool = await get_pool()
    if pool is None:
        return None
    rows = await pool.fetch(
        "SELECT c.id, c.name, c.description,"
        "       t.id, t.name, t.icon, t.description, t.type, t.route"
        " FROM tool_categories c"
        " LEFT JOIN tools t ON t.category_id = c.id AND t.is_enabled"
        " WHERE c.is_enabled"
        " ORDER BY c.sort_order, c.id, t.sort_order, t.id"
    )
    categories = []
    for r in rows:
        if not categories or categories[-1]["id"] != r[0]:
            categories.append({"id": r[0], "name": r[1], "description": r[2], "tools": []})
        if r[3] is not None:
            categories[-1]["tools"].append({
                "id": r[3], "name": r[4], "icon": r[5] or "",
                "description": r[6] or "", "type": r[7] or "", "route": r[8] or "",
            })
    return categories


def _fallback_categories():
    return [
        {
            "id": 1,
            "name": '前端工具',
            "description": '浏览器直接处理，无需后端',
            "tools": [
                    {"id": 1, "name": 'JSON格式化', "icon": '{}',
                        "description": 'JSON数据格式化美化', "type": 'frontend', "route": '/tools/json'},
                    {"id": 2, "name": 'Base64编码', "icon": '64',
                        "description": 'Base64编码解码', "type": 'frontend', "route": '/tools/base64'},
                    {"id": 3, "name": 'URL编码', "icon": '%',
                        "description": 'URL编码解码', "type": 'frontend', "route": '/tools/url'},
                    {"id": 10, "name": '颜色选择器', "icon": '🎨',
                        "description": '选择颜色代码', "type": 'frontend', "route": '/tools/color'},
                    {"id": 11, "name": '时间戳转换', "icon": '⏰',
                        "description": '时间戳转换工具', "type": 'frontend', "route": '/tools/timestamp'}
            ]
        },
        {
            "id": 2,
            "name": '后端工具',
            "description": '需要服务器处理的复杂功能',
            "tools": [
                    {"id": 4, "name": '文本对比', "icon": '≈',
                        "description": '对比两个文本的差异', "type": 'backend', "route": '/tools/diff'},
                    {"id": 9, "name": '正则测试', "icon": '.*',
                        "description": '测试正则表达式', "type": 'backend', "route": ''},
                    {"id": 12, "name": '密码生成器', "icon": '🔐',
                        "description": '生成安全密码', "type": 'backend', "route": ''},
                    {"id": 13, "name": '地图导航', "icon": '🗺',
                        "description": '显示当前位置地图', "type": 'backend', "route": '/tools/map'},
                    {"id": 14, "name": 'Sql合理性检查', "icon": '🔍',
                     "description": '比对输入的语句是否合理', "type": 'backend', "route": '/tools/sql'}
            ]
        },
        {
            "id": 3,
            "name": '图片工具',
            "description": '图片处理和转换工具（待开发）',
            "tools": [
                    {"id": 5, "name": '图片压缩', "icon": '📷',
                        "description": '压缩图片文件大小', "type": 'frontend', "route": ''},
                    {"id": 6, "name": '格式转换', "icon": '🔄',
                        "description": '转换图片格式', "type": 'frontend', "route": ''},
                    {"id": 7, "name": '二维码生成', "icon": '📱',
                        "description": '生成二维码', "type": 'frontend', "route": ''},
                    {"id": 8, "name": '图片水印', "icon": '💧',
                        "description": '添加图片水印', "type": 'backend', "route": ''}
            ]
        }
    ]


@router.get("/categories", summary="获取工具分类列表")
async def get_categories():
    """获取工具分类列表（优先读库，无 DB 或失败时回退硬编码）"""
    try:
        categories = await _load_categories_from_db()
    except Exception:
        categories = None
    if categories is None:
        categories = _fallback_categories()
    return {"categories": categories}


@router.get("/system-info", summary="系统信息")
async def system_info():
    """获取系统信息"""
    import platform
    import psutil
    import os
    from app.core.config import settings

    return {
        "system": {
            "platform": platform.system(),
            "platform_version": platform.version(),
            "architecture": platform.architecture()[0],
            "processor": platform.processor(),
            "machine": platform.machine(),
        },
        "runtime": {
            "python_version": platform.python_version(),
            "app_name": settings.app_name,
            "app_version": settings.app_version,
        },
        "resources": {
            "cpu_percent": psutil.cpu_percent(interval=1),
            "memory_percent": psutil.virtual_memory().percent,
            "disk_usage": psutil.disk_usage('/').percent,
        }
    }
