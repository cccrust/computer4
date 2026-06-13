use crate::model::supplier;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    name: String,
    contact_person: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: Option<String>,
    contact_person: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    address: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<supplier::Supplier>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let suppliers = supplier::list_suppliers(&conn, params.page, params.page_size)?;
    Ok(Json(suppliers))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<supplier::Supplier>, AppError> {
    let conn = state.conn.lock().unwrap();
    match supplier::get_supplier(&conn, id)? {
        Some(s) => Ok(Json(s)),
        None => Err(AppError::NotFound(format!("供應商 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<supplier::Supplier>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = supplier::create_supplier(
        &conn,
        &payload.name,
        payload.contact_person.as_deref(),
        payload.email.as_deref(),
        payload.phone.as_deref(),
        payload.address.as_deref(),
    )?;
    let s = supplier::get_supplier(&conn, id)?
        .ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(s))
}

pub async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<supplier::Supplier>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = supplier::update_supplier(
        &conn,
        id,
        payload.name.as_deref(),
        payload.contact_person.as_deref(),
        payload.email.as_deref(),
        payload.phone.as_deref(),
        payload.address.as_deref(),
    )?;
    if !updated {
        return Err(AppError::NotFound(format!("供應商 #{} 不存在", id)));
    }
    let s = supplier::get_supplier(&conn, id)?
        .ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(s))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if supplier::delete_supplier(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("供應商 #{} 不存在", id)))
    }
}
