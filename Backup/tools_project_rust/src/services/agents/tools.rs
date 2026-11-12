// ============================================================================
// Agent 工具模块 —— Function Call 工具定义与执行
// 职责：向 LLM 提供 tools schema（OpenAI 兼容格式），并执行模型请求的工具调用。
// 约定：天气结果经 Redis 缓存（与 Python weather_tool 共用 key，双端互享）；
//       外部 API 失败/未配置时返回可读错误文本，由模型转述，不中断对话。
// ============================================================================
use once_cell::sync::Lazy;
use serde_json::{json, Value};

use crate::core::config::SETTINGS;
use crate::core::redis::{cache_get, cache_set};

// 天气结果缓存 10 分钟、城市 LocationID 缓存 24 小时（与 Python 侧一致）
const WEATHER_CACHE_TTL: u64 = 600;
const CITY_CACHE_TTL: u64 = 86400;

static HTTP: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("初始化工具 HTTP 客户端失败")
});

/// 提供给 LLM 的工具清单（OpenAI tools 格式）
pub fn schemas() -> Value {
    json!([{
        "type": "function",
        "function": {
            "name": "weather",
            "description": "天气查询，当用户询问天气情况时使用",
            "parameters": {
                "type": "object",
                "properties": {
                    "city": {
                        "type": "string",
                        "description": "城市名称，如：北京、上海、广州。如果用户未指定城市，传入空字符串"
                    }
                },
                "required": ["city"]
            }
        }
    }])
}

/// 执行工具调用，返回给模型的结果文本
pub async fn execute(name: &str, arguments: &str) -> String {
    match name {
        "weather" => {
            let city = serde_json::from_str::<Value>(arguments)
                .ok()
                .and_then(|v| v["city"].as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            weather(&city).await
        }
        _ => format!("未知工具：{}", name),
    }
}

/// 查询城市实时天气（成功结果按城市缓存 10 分钟）
async fn weather(city: &str) -> String {
    if SETTINGS.qweather_key.is_empty() {
        return "未配置和风天气 API Key，请在 .env 中设置 QWEATHER_API_KEY".to_string();
    }

    let target = if city.trim().is_empty() {
        "北京"
    } else {
        city.trim()
    };

    let cache_key = format!("weather:now:{}", target);
    if let Some(cached) = cache_get(&cache_key).await {
        return cached;
    }

    let Some(location_id) = get_location_id(target).await else {
        return format!("未找到城市：{}", target);
    };

    let url = format!(
        "{}/v7/weather/now",
        SETTINGS.qweather_host.trim_end_matches('/')
    );
    let data: Value = match HTTP
        .get(&url)
        .query(&[("location", location_id.as_str()), ("key", &SETTINGS.qweather_key)])
        .send()
        .await
    {
        Ok(resp) => match resp.json().await {
            Ok(d) => d,
            Err(e) => return format!("天气查询出错: {}", e),
        },
        Err(e) => return format!("天气查询出错: {}", e),
    };

    if data["code"].as_str() != Some("200") {
        return format!("天气查询失败，错误码：{}", data["code"]);
    }

    let now = &data["now"];
    let text = now["text"].as_str().unwrap_or_default();
    let temp = now["temp"].as_str().unwrap_or_default();
    let feels_like = now["feelsLike"].as_str().unwrap_or_default();
    let humidity = now["humidity"].as_str().unwrap_or_default();
    let wind_dir = now["windDir"].as_str().unwrap_or_default();
    let wind_scale = now["windScale"].as_str().unwrap_or_default();
    let vis = now["vis"].as_str().unwrap_or_default();
    let result = format!(
        "{}当前天气：{}，温度 {}°C，体感 {}°C，湿度 {}%，{} {}级，能见度 {}km",
        target, text, temp, feels_like, humidity, wind_dir, wind_scale, vis
    );
    cache_set(&cache_key, &result, WEATHER_CACHE_TTL).await;
    result
}

/// 通过城市名查询 LocationID（结果按城市缓存 24h）
/// GeoAPI 与天气共用 QWEATHER_HOST：旧版 geoapi.qweather.com/v2 路径已下线（404），
/// 现行路径为 {host}/geo/v2/city/lookup；新控制台账号必须用专属 API Host
async fn get_location_id(city: &str) -> Option<String> {
    let cache_key = format!("weather:city:{}", city);
    if let Some(cached) = cache_get(&cache_key).await {
        return Some(cached);
    }

    let url = format!(
        "{}/geo/v2/city/lookup",
        SETTINGS.qweather_host.trim_end_matches('/')
    );
    let resp = HTTP
        .get(&url)
        .query(&[("location", city), ("key", &SETTINGS.qweather_key)])
        .send()
        .await
        .ok()?;
    let data: Value = resp.json().await.ok()?;
    if data["code"].as_str() != Some("200") {
        return None;
    }
    let id = data["location"][0]["id"].as_str()?.to_string();
    cache_set(&cache_key, &id, CITY_CACHE_TTL).await;
    Some(id)
}
