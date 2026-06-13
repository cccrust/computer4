use crate::model::follow;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct FollowPayload {
    follower_id: i64,
    followee_id: i64,
}

pub async fn follow(
    State(state): State<AppState>,
    Json(payload): Json<FollowPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    match follow::follow_user(&conn, payload.follower_id, payload.followee_id) {
        Ok(()) => Ok(Json(json!({ "message": "已追蹤" }))),
        Err(e) => Err(AppError::BadRequest(e.to_string())),
    }
}

pub async fn unfollow(
    State(state): State<AppState>,
    Json(payload): Json<FollowPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if follow::unfollow_user(&conn, payload.follower_id, payload.followee_id)? {
        Ok(Json(json!({ "message": "已取消追蹤" })))
    } else {
        Err(AppError::NotFound("尚無此追蹤關係".into()))
    }
}

pub async fn followers(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<follow::UserBrief>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let users = follow::list_followers(&conn, user_id)?;
    Ok(Json(users))
}

pub async fn following(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<Vec<follow::UserBrief>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let users = follow::list_following(&conn, user_id)?;
    Ok(Json(users))
}
