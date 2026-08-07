use futures::stream::{self, Stream, StreamExt};
use serde::{Deserialize, Serialize};

use crate::core::config::SETTINGS;
use crate::core::error::{internal_error, AppError};

const SYSTEM_PROMPT: &str =
    "你是 ZYTool 在线工具箱内置的 AI 助手，请用简洁、准确的中文回答用户的问题。";

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

/// 是否已配置 Kimi API（未配置时路由层走占位回复）
pub fn is_configured() -> bool {
    !SETTINGS.kimi_api_key.is_empty()
}

fn endpoint() -> String {
    format!(
        "{}/chat/completions",
        SETTINGS.kimi_base_url.trim_end_matches('/')
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

async fn send_request(stream: bool, history: Vec<(String, String)>) -> Result<reqwest::Response, AppError> {
    let body = ChatCompletionsRequest {
        model: SETTINGS.kimi_model.clone(),
        messages: build_messages(history),
        stream,
    };

    let resp = reqwest::Client::new()
        .post(endpoint())
        .bearer_auth(&SETTINGS.kimi_api_key)
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
pub async fn chat_complete(history: Vec<(String, String)>) -> Result<(String, i32), AppError> {
    let resp = send_request(false, history).await?;
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
) -> Result<impl Stream<Item = Result<String, AppError>>, AppError> {
    let resp = send_request(true, history).await?;
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
