use crate::model::order;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ListParams {
    status: Option<String>,
    customer_id: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    customer_id: i64,
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
) -> Result<Json<Vec<order::Order>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let orders = order::list_orders(
        &conn,
        params.status.as_deref(),
        params.customer_id,
        params.page,
        params.page_size,
    )?;
    Ok(Json(orders))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<order::Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    match order::get_order(&conn, id)? {
        Some(o) => Ok(Json(o)),
        None => Err(AppError::NotFound(format!("訂單 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<order::Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = order::create_order(&conn, payload.customer_id, payload.notes.as_deref())?;
    let o =
        order::get_order(&conn, id)?.ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(o))
}

pub async fn update_status(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<StatusPayload>,
) -> Result<Json<order::Order>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = order::update_order_status(&conn, id, &payload.status)?;
    if !updated {
        return Err(AppError::NotFound(format!("訂單 #{} 不存在", id)));
    }
    let o =
        order::get_order(&conn, id)?.ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(o))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if order::delete_order(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("訂單 #{} 不存在", id)))
    }
}

pub async fn list_items(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<Vec<order::OrderItem>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let items = order::list_order_items(&conn, id)?;
    Ok(Json(items))
}

pub async fn add_item(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<AddItemPayload>,
) -> Result<Json<order::OrderItem>, AppError> {
    let conn = state.conn.lock().unwrap();
    let unit_price = payload.unit_price.unwrap_or(0.0);
    let item_id =
        order::add_order_item(&conn, id, payload.product_id, payload.quantity, unit_price)?;
    let items = order::list_order_items(&conn, id)?;
    let item = items
        .into_iter()
        .find(|i| i.id == item_id)
        .ok_or_else(|| AppError::Internal("新增後查詢失敗".into()))?;
    Ok(Json(item))
}
