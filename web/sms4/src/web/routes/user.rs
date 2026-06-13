use crate::model::user;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ListParams {
    search: Option<String>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    username: String,
    display_name: String,
    bio: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    display_name: Option<String>,
    bio: Option<String>,
}

#[derive(serde::Serialize)]
pub struct UserDetail {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub followers_count: i64,
    pub following_count: i64,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<user::User>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let users = user::list_users(&conn, params.search.as_deref())?;
    Ok(Json(users))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<UserDetail>, AppError> {
    let conn = state.conn.lock().unwrap();
    let u = user::get_user(&conn, id)?.ok_or_else(|| AppError::NotFound(format!("使用者 #{} 不存在", id)))?;
    let followers = user::get_followers_count(&conn, id)?;
    let following = user::get_following_count(&conn, id)?;
    Ok(Json(UserDetail {
        id: u.id,
        username: u.username,
        display_name: u.display_name,
        bio: u.bio,
        avatar: u.avatar,
        followers_count: followers,
        following_count: following,
        created_at: u.created_at,
        updated_at: u.updated_at,
    }))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<user::User>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = user::create_user(&conn, &payload.username, &payload.display_name, payload.bio.as_deref())?;
    let u = user::get_user(&conn, id)?.ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(u))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<user::User>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = user::update_user(&conn, id, payload.display_name.as_deref(), payload.bio.as_deref())?;
    if !updated {
        return Err(AppError::NotFound(format!("使用者 #{} 不存在", id)));
    }
    let u = user::get_user(&conn, id)?.ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(u))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if user::delete_user(&conn, id)? {
        Ok(Json(json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("使用者 #{} 不存在", id)))
    }
}

pub async fn timeline(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<crate::model::post::PostWithUser>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let posts = crate::model::post::get_timeline(&conn, id, 50, 0)?;
    Ok(Json(posts))
}
