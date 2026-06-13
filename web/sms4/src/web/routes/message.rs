use crate::model::message;
use crate::model::user;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct SendPayload {
    sender_id: i64,
    receiver_id: i64,
    content: String,
}

pub async fn send(
    State(state): State<AppState>,
    Json(payload): Json<SendPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    user::get_user(&conn, payload.sender_id)?
        .ok_or_else(|| AppError::NotFound("使用者不存在".into()))?;
    user::get_user(&conn, payload.receiver_id)?
        .ok_or_else(|| AppError::NotFound("接收者不存在".into()))?;
    let id = message::send_message(&conn, payload.sender_id, payload.receiver_id, &payload.content)?;
    Ok(Json(json!({ "id": id, "message": "訊息已傳送" })))
}

pub async fn conversations(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<message::Conversation>>, AppError> {
    let conn = state.conn.lock().unwrap();
    user::get_user(&conn, user_id)?
        .ok_or_else(|| AppError::NotFound("使用者不存在".into()))?;
    let convs = message::list_conversations(&conn, user_id)?;
    Ok(Json(convs))
}

pub async fn messages(
    State(state): State<AppState>,
    Path((user_id, other_id)): Path<(i64, i64)>,
) -> Result<Json<Vec<message::MessageWithUser>>, AppError> {
    let conn = state.conn.lock().unwrap();
    user::get_user(&conn, user_id)?
        .ok_or_else(|| AppError::NotFound("使用者不存在".into()))?;
    user::get_user(&conn, other_id)?
        .ok_or_else(|| AppError::NotFound("對方使用者不存在".into()))?;
    message::mark_read(&conn, user_id, other_id)?;
    let msgs = message::list_messages(&conn, user_id, other_id)?;
    Ok(Json(msgs))
}

pub async fn unread(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let count = message::get_unread_count(&conn, user_id)?;
    Ok(Json(json!({ "unread": count })))
}
