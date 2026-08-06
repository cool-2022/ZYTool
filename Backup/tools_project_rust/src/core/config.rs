use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Settings {
    #[serde(default = "default_app_name")]
    pub app_name: String,
    #[serde(default = "default_app_version")]
    pub app_version: String,
    #[serde(default = "default_app_description")]
    pub app_description: String,
    #[serde(default)]
    pub debug: bool,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_cors_origins")]
    pub cors_origins: Vec<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_file")]
    pub log_file: Option<String>,
    #[serde(default = "default_log_dir")]
    pub log_dir: String,
    #[serde(default = "default_max_request_size")]
    pub max_request_size: usize,
    #[serde(default)]
    pub rate_limit_enabled: bool,
    #[serde(default = "default_rate_limit_requests")]
    pub rate_limit_requests: u32,
    #[serde(default = "default_rate_limit_period")]
    pub rate_limit_period: u64,
    #[serde(default = "default_secret_key")]
    pub secret_key: String,
    #[serde(default = "default_algorithm")]
    pub algorithm: String,
    #[serde(default = "default_access_token_expire_minutes")]
    pub access_token_expire_minutes: i64,
    #[serde(default)]
    pub qweather_key: String,
    #[serde(default = "default_qweather_host")]
    pub qweather_host: String,

    // PostgreSQL 数据库配置
    #[serde(default = "default_database_url")]
    pub database_url: String,
    #[serde(default = "default_db_host")]
    pub db_host: String,
    #[serde(default = "default_db_port")]
    pub db_port: u16,
    #[serde(default = "default_db_user")]
    pub db_user: String,
    #[serde(default = "default_db_password")]
    pub db_password: String,
    #[serde(default = "default_db_name")]
    pub db_name: String,
    #[serde(default = "default_db_pool_size")]
    pub db_pool_size: u32,

    // QQ 互联 OAuth 配置
    #[serde(default)]
    pub qq_app_id: String,
    #[serde(default)]
    pub qq_app_key: String,
    #[serde(default = "default_qq_redirect_uri")]
    pub qq_redirect_uri: String,
}

fn default_app_name() -> String { "ZYTool Backend API".to_string() }
fn default_app_version() -> String { "1.0.0".to_string() }
fn default_app_description() -> String { "后端工具集API，提供文本处理、正则表达式、密码生成等功能以及Agent工具".to_string() }
fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 8000 }
fn default_cors_origins() -> Vec<String> {
    vec![
        "http://localhost:5000".to_string(),
        "http://localhost:5050".to_string(),
        "http://localhost:5173".to_string(),
    ]
}
fn default_log_level() -> String { "INFO".to_string() }
fn default_log_file() -> Option<String> { Some("app.log".to_string()) }
fn default_log_dir() -> String { "logs".to_string() }
fn default_max_request_size() -> usize { 10 * 1024 * 1024 }
fn default_rate_limit_requests() -> u32 { 100 }
fn default_rate_limit_period() -> u64 { 60 }
fn default_secret_key() -> String { "your-secret-key-change-this-in-production-09d25e094faa6ca2556c818166b7a9563b93f7099f6f0f4caa6cf63b88e8d3e7".to_string() }
fn default_algorithm() -> String { "HS256".to_string() }
fn default_access_token_expire_minutes() -> i64 { 30 }
fn default_qweather_host() -> String { "https://devapi.qweather.com".to_string() }
fn default_database_url() -> String { "".to_string() }
fn default_db_host() -> String { "localhost".to_string() }
fn default_db_port() -> u16 { 5432 }
fn default_db_user() -> String { "postgres".to_string() }
fn default_db_password() -> String { "".to_string() }
fn default_db_name() -> String { "zytool".to_string() }
fn default_db_pool_size() -> u32 { 10 }
fn default_qq_redirect_uri() -> String { "http://localhost:5173/login".to_string() }

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    dotenvy::dotenv().ok();
    envy::from_env::<Settings>().unwrap_or_else(|e| {
        eprintln!("配置加载失败: {}, 使用默认配置", e);
        Settings {
            app_name: default_app_name(),
            app_version: default_app_version(),
            app_description: default_app_description(),
            debug: false,
            host: default_host(),
            port: default_port(),
            cors_origins: default_cors_origins(),
            log_level: default_log_level(),
            log_file: default_log_file(),
            log_dir: default_log_dir(),
            max_request_size: default_max_request_size(),
            rate_limit_enabled: true,
            rate_limit_requests: default_rate_limit_requests(),
            rate_limit_period: default_rate_limit_period(),
            secret_key: default_secret_key(),
            algorithm: default_algorithm(),
            access_token_expire_minutes: default_access_token_expire_minutes(),
            qweather_key: String::new(),
            qweather_host: default_qweather_host(),
            database_url: default_database_url(),
            db_host: default_db_host(),
            db_port: default_db_port(),
            db_user: default_db_user(),
            db_password: default_db_password(),
            db_name: default_db_name(),
            db_pool_size: default_db_pool_size(),
            qq_app_id: String::new(),
            qq_app_key: String::new(),
            qq_redirect_uri: default_qq_redirect_uri(),
        }
    })
});
