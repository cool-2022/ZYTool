use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::core::db::DbPool;
use crate::core::error::{internal_error, not_found, AppError};

#[derive(Debug, FromRow)]
struct SessionRow {
    session_uuid: Uuid,
    title: String,
    message_count: i32,
    total_tokens: i32,
    model_id: Option<i64>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct MessageRow {
    message_uuid: Uuid,
    role: String,
    content: String,
    content_type: String,
    tokens_used: i32,
    model_id: Option<i64>,
    created_at: DateTime<Utc>,
}

pub async fn list_sessions(
    pool: &DbPool,
    user_id: i64,
) -> Result<Vec<crate::models::SessionResponse>, AppError> {
    let rows = sqlx::query_as::<_, SessionRow>(
        r#"
        SELECT id, session_uuid, title, message_count, total_tokens, model_id, updated_at
        FROM ai_sessions
        WHERE user_id = $1 AND status = 1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询会话列表失败: {}", e)))?;

    Ok(rows.into_iter().map(into_session_response).collect())
}

pub async fn create_session(
    pool: &DbPool,
    user_id: i64,
    title: String,
    model_id: Option<i64>,
) -> Result<crate::models::SessionResponse, AppError> {
    let row = sqlx::query_as::<_, SessionRow>(
        r#"
        INSERT INTO ai_sessions (user_id, title, model_id)
        VALUES ($1, $2, $3)
        RETURNING id, session_uuid, title, message_count, total_tokens, model_id, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&title)
    .bind(model_id)
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("创建会话失败: {}", e)))?;

    Ok(into_session_response(row))
}

pub async fn delete_session(
    pool: &DbPool,
    user_id: i64,
    session_uuid: &str,
) -> Result<(), AppError> {
    let uuid = parse_uuid(session_uuid)?;

    let result = sqlx::query(
        r#"
        UPDATE ai_sessions
        SET status = 3, updated_at = NOW()
        WHERE session_uuid = $1 AND user_id = $2 AND status = 1
        "#,
    )
    .bind(uuid)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| internal_error(format!("删除会话失败: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(not_found("会话不存在或无权访问"));
    }

    Ok(())
}

pub async fn update_session_title(
    pool: &DbPool,
    user_id: i64,
    session_uuid: &str,
    title: String,
) -> Result<(), AppError> {
    let uuid = parse_uuid(session_uuid)?;

    let result = sqlx::query(
        r#"
        UPDATE ai_sessions
        SET title = $1, updated_at = NOW()
        WHERE session_uuid = $2 AND user_id = $3 AND status = 1
        "#,
    )
    .bind(&title)
    .bind(uuid)
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| internal_error(format!("更新会话标题失败: {}", e)))?;

    if result.rows_affected() == 0 {
        return Err(not_found("会话不存在或无权访问"));
    }

    Ok(())
}

pub async fn get_session_id_by_uuid(
    pool: &DbPool,
    user_id: i64,
    session_uuid: &str,
) -> Result<i64, AppError> {
    let uuid = parse_uuid(session_uuid)?;

    let row = sqlx::query_as::<_, (i64,)>(
        r#"
        SELECT id FROM ai_sessions
        WHERE session_uuid = $1 AND user_id = $2 AND status = 1
        "#,
    )
    .bind(uuid)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| internal_error(format!("查询会话失败: {}", e)))?;

    row.map(|r| r.0)
        .ok_or_else(|| not_found("会话不存在或无权访问"))
}

pub async fn list_messages(
    pool: &DbPool,
    user_id: i64,
    session_uuid: &str,
) -> Result<Vec<crate::models::MessageResponse>, AppError> {
    let uuid = parse_uuid(session_uuid)?;

    let rows = sqlx::query_as::<_, MessageRow>(
        r#"
        SELECT m.message_uuid, m.role, m.content, m.content_type, m.tokens_used, m.model_id, m.created_at
        FROM ai_messages m
        JOIN ai_sessions s ON s.id = m.session_id
        WHERE s.session_uuid = $1 AND s.user_id = $2 AND m.status = 1
        ORDER BY m.created_at ASC
        "#,
    )
    .bind(uuid)
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| internal_error(format!("查询消息失败: {}", e)))?;

    Ok(rows.into_iter().map(into_message_response).collect())
}

pub async fn save_user_message(
    pool: &DbPool,
    session_id: i64,
    content: &str,
    model_id: Option<i64>,
) -> Result<Uuid, AppError> {
    let uuid = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO ai_messages (session_id, role, content, content_type, model_id)
        VALUES ($1, 'user', $2, 'text', $3)
        RETURNING message_uuid
        "#,
    )
    .bind(session_id)
    .bind(content)
    .bind(model_id)
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("保存用户消息失败: {}", e)))?;

    Ok(uuid)
}

pub async fn save_assistant_message(
    pool: &DbPool,
    session_id: i64,
    content: &str,
    model_id: Option<i64>,
    tokens_used: i32,
) -> Result<Uuid, AppError> {
    let uuid = sqlx::query_scalar::<_, Uuid>(
        r#"
        INSERT INTO ai_messages (session_id, role, content, content_type, model_id, tokens_used)
        VALUES ($1, 'assistant', $2, 'text', $3, $4)
        RETURNING message_uuid
        "#,
    )
    .bind(session_id)
    .bind(content)
    .bind(model_id)
    .bind(tokens_used)
    .fetch_one(pool)
    .await
    .map_err(|e| internal_error(format!("保存助手消息失败: {}", e)))?;

    Ok(uuid)
}

pub async fn increment_session_stats(
    pool: &DbPool,
    session_id: i64,
    tokens_used: i32,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE ai_sessions
        SET message_count = message_count + 2,
            total_tokens = total_tokens + $1,
            updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(tokens_used)
    .bind(session_id)
    .execute(pool)
    .await
    .map_err(|e| internal_error(format!("更新会话统计失败: {}", e)))?;

    Ok(())
}

fn parse_uuid(s: &str) -> Result<Uuid, AppError> {
    Uuid::parse_str(s).map_err(|_| bad_request("无效的会话 ID 格式"))
}

fn into_session_response(row: SessionRow) -> crate::models::SessionResponse {
    crate::models::SessionResponse {
        id: row.session_uuid.to_string(),
        title: row.title,
        date: row.updated_at.format("%Y-%m").to_string(),
        message_count: row.message_count,
        total_tokens: row.total_tokens,
        model_id: row.model_id,
        updated_at: row.updated_at,
    }
}

fn into_message_response(row: MessageRow) -> crate::models::MessageResponse {
    crate::models::MessageResponse {
        id: row.message_uuid.to_string(),
        role: row.role,
        content: row.content,
        content_type: row.content_type,
        tokens_used: row.tokens_used,
        model_id: row.model_id,
        created_at: row.created_at,
    }
}

fn bad_request(msg: impl Into<String>) -> AppError {
    use crate::core::error::bad_request;
    bad_request(msg)
}
