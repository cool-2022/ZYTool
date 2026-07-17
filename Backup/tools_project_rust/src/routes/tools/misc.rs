use axum::{routing::get, Json, Router};
use sysinfo::{Disks, System};

use crate::core::config::SETTINGS;
use crate::models::{
    CategoriesResponse, Category, ResourceInfo, RuntimeInfo, SystemInfo, SystemInfoResponse,
    ToolItem,
};

pub fn router() -> Router {
    Router::new()
        .route("/categories", get(get_categories))
        .route("/system-info", get(system_info))
}

async fn get_categories() -> Json<CategoriesResponse> {
    Json(CategoriesResponse {
        categories: vec![
            Category {
                id: 1,
                name: "前端工具".to_string(),
                description: "浏览器直接处理，无需后端".to_string(),
                tools: vec![
                    ToolItem { id: 1, name: "JSON格式化".to_string(), icon: "{}".to_string(), description: "JSON数据格式化美化".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 2, name: "Base64编码".to_string(), icon: "64".to_string(), description: "Base64编码解码".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 3, name: "URL编码".to_string(), icon: "%".to_string(), description: "URL编码解码".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 10, name: "颜色选择器".to_string(), icon: "🎨".to_string(), description: "选择颜色代码".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 11, name: "时间戳转换".to_string(), icon: "⏰".to_string(), description: "时间戳转换工具".to_string(), tool_type: "frontend".to_string() },
                ],
            },
            Category {
                id: 2,
                name: "后端工具".to_string(),
                description: "需要服务器处理的复杂功能".to_string(),
                tools: vec![
                    ToolItem { id: 4, name: "文本对比".to_string(), icon: "≈".to_string(), description: "对比两个文本的差异".to_string(), tool_type: "backend".to_string() },
                    ToolItem { id: 9, name: "正则测试".to_string(), icon: ".*".to_string(), description: "测试正则表达式".to_string(), tool_type: "backend".to_string() },
                    ToolItem { id: 12, name: "密码生成器".to_string(), icon: "🔐".to_string(), description: "生成安全密码".to_string(), tool_type: "backend".to_string() },
                    ToolItem { id: 13, name: "地图导航".to_string(), icon: "🗺".to_string(), description: "显示当前位置地图".to_string(), tool_type: "backend".to_string() },
                    ToolItem { id: 15, name: "地图导航".to_string(), icon: "🦌".to_string(), description: "显示路径".to_string(), tool_type: "backend".to_string() },
                    ToolItem { id: 14, name: "Sql合理性检查".to_string(), icon: "🔍".to_string(), description: "比对输入的语句是否合理".to_string(), tool_type: "backend".to_string() },
                ],
            },
            Category {
                id: 3,
                name: "图片工具".to_string(),
                description: "图片处理和转换工具（待开发）".to_string(),
                tools: vec![
                    ToolItem { id: 5, name: "图片压缩".to_string(), icon: "📷".to_string(), description: "压缩图片文件大小".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 6, name: "格式转换".to_string(), icon: "🔄".to_string(), description: "转换图片格式".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 7, name: "二维码生成".to_string(), icon: "📱".to_string(), description: "生成二维码".to_string(), tool_type: "frontend".to_string() },
                    ToolItem { id: 8, name: "图片水印".to_string(), icon: "💧".to_string(), description: "添加图片水印".to_string(), tool_type: "backend".to_string() },
                ],
            },
        ],
    })
}

async fn system_info() -> Json<SystemInfoResponse> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new_with_refreshed_list();
    let disk_usage = disks
        .iter()
        .next()
        .map(|d| {
            let total = d.total_space() as f64;
            let used = (d.total_space() - d.available_space()) as f64;
            if total > 0.0 { used / total * 100.0 } else { 0.0 }
        })
        .unwrap_or(0.0);

    Json(SystemInfoResponse {
        system: SystemInfo {
            platform: std::env::consts::OS.to_string(),
            platform_version: "unknown".to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            processor: "unknown".to_string(),
            machine: std::env::consts::ARCH.to_string(),
        },
        runtime: RuntimeInfo {
            rust_version: format!("rustc {}", option_env!("CARGO_PKG_RUST_VERSION").unwrap_or("unknown")),
            app_name: SETTINGS.app_name.clone(),
            app_version: SETTINGS.app_version.clone(),
        },
        resources: ResourceInfo {
            cpu_percent: sys.global_cpu_usage(),
            memory_percent: sys.used_memory() as f32 / sys.total_memory() as f32 * 100.0,
            disk_usage,
        },
    })
}
