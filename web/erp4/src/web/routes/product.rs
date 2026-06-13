use crate::model::product;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    search: Option<String>,
    low_stock: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    name: String,
    sku: String,
    price: f64,
    stock: i64,
    description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdatePayload {
    name: Option<String>,
    sku: Option<String>,
    price: Option<f64>,
    stock: Option<i64>,
    description: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let products = product::list_products(
        &conn,
        params.search.as_deref(),
        params.low_stock,
        params.page,
        params.page_size,
    )?;
    Ok(Json(products))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    match product::get_product(&conn, id)? {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound(format!("產品 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = product::create_product(
        &conn,
        &payload.name,
        &payload.sku,
        payload.price,
        payload.stock,
        payload.description.as_deref(),
    )?;
    let p = product::get_product(&conn, id)?
        .ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(p))
}

pub async fn update(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePayload>,
) -> Result<Json<product::Product>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = product::update_product(
        &conn,
        id,
        payload.name.as_deref(),
        payload.sku.as_deref(),
        payload.price,
        payload.stock,
        payload.description.as_deref(),
    )?;
    if !updated {
        return Err(AppError::NotFound(format!("產品 #{} 不存在", id)));
    }
    let p = product::get_product(&conn, id)?
        .ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(p))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if product::delete_product(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("產品 #{} 不存在", id)))
    }
}
