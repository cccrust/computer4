use crate::model::customer;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListParams {
    search: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    name: String,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<customer::Customer>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let customers = customer::list_customers(
        &conn,
        params.search.as_deref(),
        params.page,
        params.page_size,
    )?;
    Ok(Json(customers))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<customer::Customer>, AppError> {
    let conn = state.conn.lock().unwrap();
    match customer::get_customer(&conn, id)? {
        Some(c) => Ok(Json(c)),
        None => Err(AppError::NotFound(format!("客戶 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<customer::Customer>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = customer::create_customer(
        &conn,
        &payload.name,
        payload.email.as_deref(),
        payload.phone.as_deref(),
        payload.address.as_deref(),
    )?;
    let c = customer::get_customer(&conn, id)?
        .ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(c))
}

pub async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<customer::Customer>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = customer::update_customer(
        &conn,
        id,
        payload.name.as_deref(),
        payload.email.as_deref(),
        payload.phone.as_deref(),
        payload.address.as_deref(),
    )?;
    if !updated {
        return Err(AppError::NotFound(format!("客戶 #{} 不存在", id)));
    }
    let c = customer::get_customer(&conn, id)?
        .ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(c))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if customer::delete_customer(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("客戶 #{} 不存在", id)))
    }
}
