use crate::model::purchase_order;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListParams {
    status: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    supplier_id: i64,
    notes: Option<String>,
}

#[derive(Deserialize)]
pub struct StatusPayload {
    status: String,
}

#[derive(Deserialize)]
pub struct AddItemPayload {
    product_id: i64,
    quantity: i64,
    unit_price: Option<f64>,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<purchase_order::PurchaseOrder>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = purchase_order::list_purchase_orders(
        &conn,
        params.status.as_deref(),
        params.page,
        params.page_size,
    )?;
    Ok(Json(items))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<purchase_order::PurchaseOrder>, AppError> {
    let conn = state.conn.lock().unwrap();
    match purchase_order::get_purchase_order(&conn, id)? {
        Some(po) => Ok(Json(po)),
        None => Err(AppError::NotFound(format!("採購單 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<purchase_order::PurchaseOrder>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = purchase_order::create_purchase_order(
        &conn,
        payload.supplier_id,
        payload.notes.as_deref(),
    )?;
    let po = purchase_order::get_purchase_order(&conn, id)?
        .ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(po))
}

pub async fn update_status(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<StatusPayload>,
) -> Result<Json<purchase_order::PurchaseOrder>, AppError> {
    let conn = state.conn.lock().unwrap();
    if !purchase_order::update_purchase_order_status(&conn, id, &payload.status)? {
        return Err(AppError::NotFound(format!("採購單 #{} 不存在", id)));
    }
    let po = purchase_order::get_purchase_order(&conn, id)?
        .ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(po))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if purchase_order::delete_purchase_order(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("採購單 #{} 不存在", id)))
    }
}

pub async fn list_items(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<purchase_order::PurchaseOrderItem>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = purchase_order::list_purchase_order_items(&conn, id)?;
    Ok(Json(items))
}

pub async fn add_item(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<AddItemPayload>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    let item_id = purchase_order::add_purchase_order_item(
        &conn,
        id,
        payload.product_id,
        payload.quantity,
        payload.unit_price.unwrap_or(0.0),
    )?;
    Ok(Json(serde_json::json!({ "id": item_id })))
}
