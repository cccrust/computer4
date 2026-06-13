use crate::model::invoice;
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ListParams {
    status: Option<String>,
    customer_id: Option<i64>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreatePayload {
    invoice_number: Option<String>,
    order_id: Option<i64>,
    customer_id: i64,
    due_date: String,
    amount: f64,
    notes: Option<String>,
}

#[derive(Deserialize)]
pub struct StatusPayload {
    status: String,
}

pub async fn list(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<invoice::Invoice>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let invoices = invoice::list_invoices(
        &conn,
        params.status.as_deref(),
        params.customer_id,
        params.page,
        params.page_size,
    )?;
    Ok(Json(invoices))
}

pub async fn get(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<invoice::Invoice>, AppError> {
    let conn = state.conn.lock().unwrap();
    match invoice::get_invoice(&conn, id)? {
        Some(inv) => Ok(Json(inv)),
        None => Err(AppError::NotFound(format!("發票 #{} 不存在", id))),
    }
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthUser,
    Json(payload): Json<CreatePayload>,
) -> Result<Json<invoice::Invoice>, AppError> {
    let conn = state.conn.lock().unwrap();
    let id = invoice::create_invoice(
        &conn,
        payload.invoice_number.as_deref(),
        payload.order_id,
        payload.customer_id,
        &payload.due_date,
        payload.amount,
        payload.notes.as_deref(),
    )?;
    let inv = invoice::get_invoice(&conn, id)?
        .ok_or_else(|| AppError::Internal("建立後查詢失敗".into()))?;
    Ok(Json(inv))
}

pub async fn update_status(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
    Json(payload): Json<StatusPayload>,
) -> Result<Json<invoice::Invoice>, AppError> {
    let conn = state.conn.lock().unwrap();
    let updated = invoice::update_invoice_status(&conn, id, &payload.status)?;
    if !updated {
        return Err(AppError::NotFound(format!("發票 #{} 不存在", id)));
    }
    let inv = invoice::get_invoice(&conn, id)?
        .ok_or_else(|| AppError::Internal("更新後查詢失敗".into()))?;
    Ok(Json(inv))
}

pub async fn delete(
    State(state): State<AppState>,
    _auth: AuthUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();
    if invoice::delete_invoice(&conn, id)? {
        Ok(Json(serde_json::json!({ "deleted": true })))
    } else {
        Err(AppError::NotFound(format!("發票 #{} 不存在", id)))
    }
}
