from fastapi import APIRouter


router = APIRouter(tags=["misc"])


@router.get("/api/categories")
async def get_categories():
    """获取工具分类列表"""
    categories = [
            {
                "id": 1,
                "name": '前端工具',
                "description": '浏览器直接处理，无需后端',
                "tools": [
                    { "id": 1, "name": 'JSON格式化', "icon": '{}', "description": 'JSON数据格式化美化', "type": 'frontend' },
                    { "id": 2, "name": 'Base64编码', "icon": '64', "description": 'Base64编码解码', "type": 'frontend' },
                    { "id": 3, "name": 'URL编码', "icon": '%', "description": 'URL编码解码', "type": 'frontend' },
                    { "id": 10, "name": '颜色选择器', "icon": '🎨', "description": '选择颜色代码', "type": 'frontend' },
                    { "id": 11, "name": '时间戳转换', "icon": '⏰', "description": '时间戳转换工具', "type": 'frontend' }
                ]
            },
            {
                "id": 2,
                "name": '后端工具',
                "description": '需要服务器处理的复杂功能',
                "tools": [
                    { "id": 4, "name": '文本对比', "icon": '≈', "description": '对比两个文本的差异', "type": 'backend' },
                    { "id": 9, "name": '正则测试', "icon": '.*', "description": '测试正则表达式', "type": 'backend' },
                    { "id": 12, "name": '密码生成器', "icon": '🔐', "description": '生成安全密码', "type": 'backend' },
                    { "id": 13, "name": '地图导航', "icon": '🗺', "description": '显示当前位置地图', "type": 'backend' },
                    { "id": 14, "name": 'Sql合理性检查', "icon": '🔍', "description": '比对输入的语句是否合理', "type": 'backend' }
                ]
            },
            {
                "id": 3,
                "name": '图片工具',
                "description": '图片处理和转换工具（待开发）',
                "tools": [
                    { "id": 5, "name": '图片压缩', "icon": '📷', "description": '压缩图片文件大小', "type": 'frontend' },
                    { "id": 6, "name": '格式转换', "icon": '🔄', "description": '转换图片格式', "type": 'frontend' },
                    { "id": 7, "name": '二维码生成', "icon": '📱', "description": '生成二维码', "type": 'frontend' },
                    { "id": 8, "name": '图片水印', "icon": '💧', "description": '添加图片水印', "type": 'backend' }
                ]
            }
        ]
    return {"categories": categories}


@router.get("/api/health")
async def health_check():
    """健康检查"""
    return {"status": "healthy", "message": "ZYTool API is running"}


