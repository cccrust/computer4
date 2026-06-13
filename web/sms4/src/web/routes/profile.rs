use crate::model::profile::{self, Profile, ProfileSearch};
use crate::model::interest;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct ProfileUpdate {
    birthday: Option<String>,
    gender: Option<String>,
    city: Option<String>,
    occupation: Option<String>,
    education: Option<String>,
    height: Option<i64>,
    looking_for: Option<String>,
    about_me: Option<String>,
}

pub async fn get(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = profile::get_profile(&conn, user_id)?
        .ok_or_else(|| AppError::NotFound("該使用者尚無交友資料".into()))?;
    let tags = interest::list_interests(&conn, user_id)?;
    let tag_list: Vec<String> = tags.iter().map(|t| t.tag.clone()).collect();
    Ok(Json(json!({ "profile": p, "tags": tag_list })))
}

pub async fn update(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Json(payload): Json<ProfileUpdate>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let p = Profile {
        user_id,
        birthday: payload.birthday,
        gender: payload.gender,
        city: payload.city,
        occupation: payload.occupation,
        education: payload.education,
        height: payload.height,
        looking_for: payload.looking_for,
        about_me: payload.about_me,
        updated_at: String::new(),
    };
    profile::upsert_profile(&conn, user_id, &p)?;
    Ok(Json(json!({ "message": "交友資料已更新" })))
}

pub async fn search(
    State(state): State<AppState>,
    Query(params): Query<ProfileSearch>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let results = profile::search_profiles(&conn, &params)?;
    Ok(Json(json!({ "results": results, "count": results.len() })))
}
