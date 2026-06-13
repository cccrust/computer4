use crate::model::interest;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct InterestPayload {
    pub user_id: i64,
    pub tag: String,
}

#[derive(Deserialize)]
pub struct InterestRemove {
    pub user_id: i64,
    pub tag: String,
}

pub async fn add(
    State(state): State<AppState>,
    Json(payload): Json<InterestPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = interest::add_interest(&conn, payload.user_id, &payload.tag)?;
    Ok(Json(json!({ "id": id, "message": "已新增興趣標籤" })))
}

pub async fn remove(
    State(state): State<AppState>,
    Json(payload): Json<InterestRemove>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if interest::remove_interest(&conn, payload.user_id, &payload.tag)? {
        Ok(Json(json!({ "message": "已移除興趣標籤" })))
    } else {
        Err(AppError::NotFound("興趣標籤不存在".into()))
    }
}

pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let list = interest::list_interests(&conn, user_id)?;
    Ok(Json(json!({ "interests": list })))
}
