use crate::model::like;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct LikePayload {
    user_id: i64,
    post_id: i64,
}

pub async fn like(
    State(state): State<AppState>,
    Json(payload): Json<LikePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if like::like_post(&conn, payload.user_id, payload.post_id)? {
        Ok(Json(json!({ "message": "已按讚" })))
    } else {
        Ok(Json(json!({ "message": "已經按過讚了" })))
    }
}

pub async fn unlike(
    State(state): State<AppState>,
    Json(payload): Json<LikePayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if like::unlike_post(&conn, payload.user_id, payload.post_id)? {
        Ok(Json(json!({ "message": "已取消讚" })))
    } else {
        Err(AppError::NotFound("尚未按讚".into()))
    }
}
