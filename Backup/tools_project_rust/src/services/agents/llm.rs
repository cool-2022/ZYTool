use futures::stream::{self, Stream, StreamExt};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::core::config::SETTINGS;
use crate::core::db::DbPool;
use crate::core::error::{internal_error, AppError};

const SYSTEM_PROMPT: &str =
    "你是 ZYTool 在线工具箱内置的 AI 助手，请用简洁、准确的中文回答用户的问题。";

/// 模型配置（来源：ai_models 表，回退 .env）
#[derive(Debug, Clone)]
pub struct ModelConfig {
    #[allow(dead_code)]
    pub provider: String,
    pub model_name: String,
    pub api_base_url: String,
    pub api_key: String,
    #[allow(dead_code)]
    pub max_tokens_default: i32,
}

static MODEL_CACHE: Lazy<RwLock<Option<(ModelConfig, Instant)>>> =
    Lazy::new(|| RwLock::new(None));
const CACHE_TTL: Duration = Duration::from_secs(60);

/// 解析当前生效的模型配置：
/// 1. 指定 model_id 时直接按 id 加载（用户显式选择，不看 is_enabled）
/// 2. 否则取 ai_models 表 is_enabled + priority 最高（缓存 60s）
/// 3. 都没有时回退 .env
pub async fn resolve_model(pool: Option<&DbPool>, model_id: Option<i64>) -> Option<ModelConfig> {
    if let (Some(pool), Some(id)) = (pool, model_id) {
        match load_model_by_id(pool, id).await {
            Ok(Some(cfg)) => {
                tracing::info!("使用指定模型配置: {}/{} (id={})", cfg.provider, cfg.model_name, id);
                return Some(cfg);
            }
            Ok(None) => tracing::warn!("模型 id={} 不存在，使用默认模型", id),
            Err(e) => tracing::warn!("查询模型 id={} 失败: {}，使用默认模型", id, e.message),
        }
    }

    {
        let guard = MODEL_CACHE.read().await;
        if let Some((cfg, ts)) = guard.as_ref() {
            if ts.elapsed() < CACHE_TTL {
                return Some(cfg.clone());
            }
        }
    }

    if let Some(pool) = pool {
        match load_model_from_db(pool).await {
            Ok(Some(cfg)) => {
                tracing::info!("使用数据库模型配置: {}/{}", cfg.provider, cfg.model_name);
                let mut guard = MODEL_CACHE.write().await;
                *guard = Some((cfg.clone(), Instant::now()));
                return Some(cfg);
            }
            Ok(None) => {}
            Err(e) => tracing::warn!("查询 ai_models 失败，回退环境变量: {}", e.message),
        }
    }

    if !SETTINGS.kimi_api_key.is_empty() {
        return Some(ModelConfig {
            provider: "kimi".to_string(),
            model_name: SETTINGS.kimi_model.clone(),
            api_base_url: SETTINGS.kimi_base_url.clone(),
            api_key: SETTINGS.kimi_api_key.clone(),
            max_tokens_default: 4096,
        });
    }

    None
}

async fn load_model_by_id(pool: &DbPool, id: i64) -> Result<Option<ModelConfig>, AppError> {
    let row = sqlx::query_as::<_, (String, String, String, String, i32)>(
        r#"
        SELECT provider, model_name, api_base_url, api_key_encrypted, max_tokens_default
        FROM ai_models
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("查询模型失败: {}", e)))?;

    Ok(row.map(into_model_config))
}

async fn load_model_from_db(pool: &DbPool) -> Result<Option<ModelConfig>, AppError> {
    let row = sqlx::query_as::<_, (String, String, String, String, i32)>(
        r#"
        SELECT provider, model_name, api_base_url, api_key_encrypted, max_tokens_default
        FROM ai_models
        WHERE is_enabled = true
        ORDER BY priority DESC
        LIMIT 1
        "#,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("查询 ai_models 失败: {}", e)))?;

    Ok(row.map(into_model_config))
}

fn into_model_config(
    (provider, model_name, api_base_url, api_key_encrypted, max_tokens_default): (
        String,
        String,
        String,
        String,
        i32,
    ),
) -> ModelConfig {
    // 解密 api_key（历史明文数据原样兼容）
    let api_key = match crate::core::crypto::decrypt(&api_key_encrypted) {
        Ok(key) => key,
        Err(e) => {
            tracing::error!("解密 {} 的 API Key 失败: {}", model_name, e);
            String::new()
        }
    };
    ModelConfig {
        provider,
        model_name,
        api_base_url,
        api_key,
        max_tokens_default,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
struct ChatCompletionsRequest {
    model: String,
    messages: Vec<LlmMessage>,
    stream: bool,
}

#[derive(Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<CompletionChoice>,
    usage: Option<CompletionUsage>,
}

#[derive(Deserialize)]
struct CompletionChoice {
    message: Option<LlmMessage>,
    delta: Option<CompletionDelta>,
}

#[derive(Deserialize)]
struct CompletionDelta {
    content: Option<String>,
}

#[derive(Deserialize)]
struct CompletionUsage {
    total_tokens: i32,
}

fn endpoint(cfg: &ModelConfig) -> String {
    format!(
        "{}/chat/completions",
        cfg.api_base_url.trim_end_matches('/')
    )
}

fn build_messages(history: Vec<(String, String)>) -> Vec<LlmMessage> {
    let mut messages = vec![LlmMessage {
        role: "system".to_string(),
        content: SYSTEM_PROMPT.to_string(),
    }];
    messages.extend(history.into_iter().map(|(role, content)| LlmMessage {
        role,
        content,
    }));
    messages
}

async fn send_request(
    stream: bool,
    history: Vec<(String, String)>,
    cfg: ModelConfig,
) -> Result<reqwest::Response, AppError> {
    let body = ChatCompletionsRequest {
        model: cfg.model_name.clone(),
        messages: build_messages(history),
        stream,
    };

    let resp = reqwest::Client::new()
        .post(endpoint(&cfg))
        .bearer_auth(&cfg.api_key)
        .json(&body)
        .send()
        .await
        .map_err(|e| {
            tracing::error!("Kimi API 请求失败: {}", e);
            internal_error(format!("请求 Kimi API 失败: {}", e))
        })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        tracing::error!("Kimi API 返回错误 ({}): {}", status, text);
        return Err(internal_error(format!(
            "Kimi API 返回错误 ({}): {}",
            status, text
        )));
    }

    Ok(resp)
}

/// 非流式调用，返回 (完整回复, 消耗 tokens)
pub async fn chat_complete(
    history: Vec<(String, String)>,
    cfg: ModelConfig,
) -> Result<(String, i32), AppError> {
    let resp = send_request(false, history, cfg).await?;
    let parsed: ChatCompletionsResponse = resp
        .json()
        .await
        .map_err(|e| internal_error(format!("解析 Kimi 响应失败: {}", e)))?;

    let content = parsed
        .choices
        .into_iter()
        .next()
        .and_then(|c| c.message)
        .map(|m| m.content)
        .unwrap_or_default();
    let tokens = parsed.usage.map(|u| u.total_tokens).unwrap_or(0);

    Ok((content, tokens))
}

/// 流式调用，返回增量内容的流
pub async fn chat_stream(
    history: Vec<(String, String)>,
    cfg: ModelConfig,
) -> Result<impl Stream<Item = Result<String, AppError>>, AppError> {
    let resp = send_request(true, history, cfg).await?;
    let byte_stream = resp.bytes_stream();

    let stream = stream::unfold(
        (byte_stream, String::new()),
        |(mut bytes, mut buf)| async move {
            loop {
                // 优先处理缓冲区中已有的完整行
                if let Some(pos) = buf.find('\n') {
                    let line: String = buf.drain(..=pos).collect();
                    let line = line.trim();

                    if let Some(data) = line.strip_prefix("data:") {
                        let data = data.trim();
                        if data == "[DONE]" {
                            return None;
                        }
                        if let Ok(chunk) = serde_json::from_str::<ChatCompletionsResponse>(data) {
                            if let Some(text) = chunk
                                .choices
                                .into_iter()
                                .next()
                                .and_then(|c| c.delta)
                                .and_then(|d| d.content)
                            {
                                if !text.is_empty() {
                                    return Some((Ok(text), (bytes, buf)));
                                }
                            }
                        }
                    }
                    continue;
                }

                // 缓冲区没有完整行，读取下一段数据
                match bytes.next().await {
                    Some(Ok(chunk)) => buf.push_str(&String::from_utf8_lossy(&chunk)),
                    Some(Err(e)) => {
                        return Some((
                            Err(internal_error(format!("读取 Kimi 流式响应失败: {}", e))),
                            (bytes, buf),
                        ))
                    }
                    None => return None,
                }
            }
        },
    );

    Ok(stream)
}
