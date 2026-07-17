use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolItem {
    pub id: i64,
    pub name: String,
    pub icon: String,
    pub description: String,
    #[serde(rename = "type")]
    pub tool_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub tools: Vec<ToolItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub processor: String,
    pub machine: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub rust_version: String,
    pub app_name: String,
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourceInfo {
    pub cpu_percent: f32,
    pub memory_percent: f32,
    pub disk_usage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfoResponse {
    pub system: SystemInfo,
    pub runtime: RuntimeInfo,
    pub resources: ResourceInfo,
}
