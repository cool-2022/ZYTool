use futures::stream::{self, Stream, StreamExt};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

use crate::core::config::SETTINGS;
use crate::core::db::DbPool;
use crate::core::error::{internal_error, AppError};

const SYSTEM_PROMPT: &str =
    "你是 ZYTool 在线工具箱内置的 AI 助手，请用简洁、准确的中文回答用户的问题。\
     涉及天气等实时信息时，优先调用提供的工具获取最新数据，不要凭记忆回答。";

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

// ===================== OpenAI 兼容协议（流式增量解析） =====================

#[derive(Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Deserialize)]
struct StreamChoice {
    delta: Option<StreamDelta>,
}

#[derive(Deserialize)]
struct StreamDelta {
    content: Option<String>,
    tool_calls: Option<Vec<ToolCallDelta>>,
}

#[derive(Deserialize)]
struct ToolCallDelta {
    id: Option<String>,
    function: Option<ToolCallFunction>,
}

#[derive(Deserialize)]
struct ToolCallFunction {
    name: Option<String>,
    arguments: Option<String>,
}

/// 流式过程中累积的工具调用（参数为分片传输，需拼接）
#[derive(Debug, Default, Clone)]
struct ToolCallAcc {
    seen: bool,
    id: String,
    name: String,
    arguments: String,
}

fn endpoint(cfg: &ModelConfig) -> String {
    format!(
        "{}/chat/completions",
        cfg.api_base_url.trim_end_matches('/')
    )
}

/// 历史消息转 OpenAI messages 格式（system 提示词置顶）
fn build_messages(history: &[(String, String)]) -> Vec<Value> {
    let mut messages = vec![json!({"role": "system", "content": SYSTEM_PROMPT})];
    messages.extend(
        history
            .iter()
            .map(|(role, content)| json!({"role": role, "content": content})),
    );
    messages
}

/// 工具结果回灌：assistant 的 tool_calls + tool 角色结果两条消息
fn append_tool_messages(
    messages: &mut Vec<Value>,
    call: &ToolCallAcc,
    result: &str,
) {
    messages.push(json!({
        "role": "assistant",
        "tool_calls": [{
            "id": call.id,
            "type": "function",
            "function": {"name": call.name, "arguments": call.arguments}
        }]
    }));
    messages.push(json!({"role": "tool", "tool_call_id": call.id, "content": result}));
}

async fn send_request(
    messages: Vec<Value>,
    tools: Option<Value>,
    stream: bool,
    cfg: &ModelConfig,
) -> Result<reqwest::Response, AppError> {
    let mut body = json!({
        "model": cfg.model_name,
        "messages": messages,
        "stream": stream,
    });
    if let Some(tools) = tools {
        body["tools"] = tools;
    }

    let resp = reqwest::Client::new()
        .post(endpoint(cfg))
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

/// SSE 增量解析流：透传 content 增量；acc 为 Some 时同步累积 tool_calls 分片
fn content_stream(
    resp: reqwest::Response,
    acc: Option<Arc<Mutex<ToolCallAcc>>>,
) -> impl Stream<Item = Result<String, AppError>> + Send {
    let byte_stream = resp.bytes_stream();

    stream::unfold(
        (byte_stream, String::new(), acc),
        |(mut bytes, mut buf, acc)| async move {
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
                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(delta) =
                                chunk.choices.into_iter().next().and_then(|c| c.delta)
                            {
                                if let (Some(acc), Some(calls)) = (&acc, delta.tool_calls) {
                                    let mut guard = acc.lock().await;
                                    for call in calls {
                                        if let Some(id) = call.id {
                                            guard.id = id;
                                        }
                                        if let Some(f) = call.function {
                                            if let Some(name) = f.name {
                                                guard.name.push_str(&name);
                                            }
                                            if let Some(args) = f.arguments {
                                                guard.arguments.push_str(&args);
                                            }
                                        }
                                        guard.seen = true;
                                    }
                                }
                                if let Some(text) = delta.content {
                                    if !text.is_empty() {
                                        return Some((Ok(text), (bytes, buf, acc)));
                                    }
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
                            (bytes, buf, acc),
                        ))
                    }
                    None => return None,
                }
            }
        },
    )
}

/// 非流式调用（带工具循环：命中 tool_calls 则执行工具后二次请求），返回 (完整回复, 消耗 tokens)
pub async fn chat_complete(
    history: Vec<(String, String)>,
    cfg: ModelConfig,
) -> Result<(String, i32), AppError> {
    let messages = build_messages(&history);
    let resp = send_request(
        messages.clone(),
        Some(crate::services::agents::tools::schemas()),
        false,
        &cfg,
    )
    .await?;
    let parsed: Value = resp
        .json()
        .await
        .map_err(|e| internal_error(format!("解析 Kimi 响应失败: {}", e)))?;
    let tokens = parsed["usage"]["total_tokens"].as_i64().unwrap_or(0) as i32;
    let choice = &parsed["choices"][0];

    if choice["finish_reason"].as_str() == Some("tool_calls") {
        if let Some(tc) = choice["message"]["tool_calls"].as_array().and_then(|a| a.first()) {
            let call = ToolCallAcc {
                seen: true,
                id: tc["id"].as_str().unwrap_or_default().to_string(),
                name: tc["function"]["name"].as_str().unwrap_or_default().to_string(),
                arguments: tc["function"]["arguments"]
                    .as_str()
                    .unwrap_or("{}")
                    .to_string(),
            };
            let result = crate::services::agents::tools::execute(&call.name, &call.arguments).await;

            let mut messages = messages;
            append_tool_messages(&mut messages, &call, &result);
            let resp2 = send_request(messages, None, false, &cfg).await?;
            let parsed2: Value = resp2
                .json()
                .await
                .map_err(|e| internal_error(format!("解析 Kimi 响应失败: {}", e)))?;
            let tokens2 = parsed2["usage"]["total_tokens"].as_i64().unwrap_or(0) as i32;
            let content = parsed2["choices"][0]["message"]["content"]
                .as_str()
                .unwrap_or_default()
                .to_string();
            return Ok((content, tokens + tokens2));
        }
    }

    let content = choice["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    Ok((content, tokens))
}

/// 流式调用（带工具）：第一轮流式透传内容并累积 tool_calls；
/// 若模型要求调工具，执行后把结果回灌，发起第二轮流式并衔接在同一流上
pub async fn chat_stream_with_tools(
    history: Vec<(String, String)>,
    cfg: ModelConfig,
) -> Result<impl Stream<Item = Result<String, AppError>>, AppError> {
    let resp = send_request(
        build_messages(&history),
        Some(crate::services::agents::tools::schemas()),
        true,
        &cfg,
    )
    .await?;

    let acc = Arc::new(Mutex::new(ToolCallAcc::default()));
    let first = content_stream(resp, Some(acc.clone()));

    let second = stream::once(async move {
        let call = acc.lock().await.clone();
        if !call.seen {
            let empty: futures::stream::BoxStream<'static, Result<String, AppError>> =
                stream::empty().boxed();
            return empty;
        }
        tracing::info!("模型请求调用工具: {}({})", call.name, call.arguments);
        let result = crate::services::agents::tools::execute(&call.name, &call.arguments).await;

        let mut messages = build_messages(&history);
        append_tool_messages(&mut messages, &call, &result);
        match send_request(messages, None, true, &cfg).await {
            Ok(resp) => content_stream(resp, None).boxed(),
            Err(e) => stream::once(async move { Err::<String, AppError>(e) }).boxed(),
        }
    })
    .flatten();

    Ok(first.chain(second))
}
