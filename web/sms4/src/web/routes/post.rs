use crate::model::post;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ListParams {
    user_id: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    user_id: i64,
    content: String,
}

#[derive(Deserialize)]
pub struct ReplyPayload {
    user_id: i64,
    content: String,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<post::PostWithUser>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let posts = post::list_posts(&conn, params.user_id, params.limit.unwrap_or(50), params.offset.unwrap_or(0))?;
    Ok(Json(posts))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = post::get_post(&conn, id)?.ok_or_else(|| AppError::NotFound(format!("貼文 #{} 不存在", id)))?;
    let replies = post::list_replies(&conn, id)?;
    Ok(Json(json!({
        "post": p,
        "replies": replies
    })))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<post::PostWithUser>, AppError> {
    let conn = state.conn.lock().unwrap();
    let post_id = post::create_post(&conn, payload.user_id, &payload.content, None)?;
    let p = post::get_post(&conn, post_id)?.ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(p))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if post::delete_post(&conn, id)? {
        Ok(Json(json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("貼文 #{} 不存在", id)))
    }
}

pub async fn reply(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<ReplyPayload>,
) -> Result<Json<post::PostWithUser>, AppError> {
    let conn = state.conn.lock().unwrap();
    if post::get_post(&conn, id)?.is_none() {
        return Err(AppError::NotFound(format!("貼文 #{} 不存在", id)));
    }
    let reply_id = post::create_post(&conn, payload.user_id, &payload.content, Some(id))?;
    let r = post::get_post(&conn, reply_id)?.ok_or_else(|| AppError::Internal("建立回覆後查詢失敗".into()))?;
    Ok(Json(r))
}
