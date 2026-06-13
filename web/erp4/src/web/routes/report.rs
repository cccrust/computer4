use crate::model::report::SalesRow;
use crate::model::{invoice, product, report};
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SalesParams {
    from: Option<String>,
    to: Option<String>,
}

pub async fn sales(
    State(state): State<AppState>,
    _auth: AuthUser,
    Query(params): Query<SalesParams>,
) -> Result<Json<Vec<SalesRow>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let rows = report::sales_report(
        &conn,
        params.from.as_deref(),
        params.to.as_deref(),
        None,
        None,
    )?;
    Ok(Json(rows))
}

pub async fn inventory(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<Vec<product::Product>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let products = product::list_products(&conn, None, None, None, None)?;
    Ok(Json(products))
}

pub async fn aging(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<Vec<invoice::Invoice>>, AppError> {
    let conn = state.conn.lock().unwrap();
    let invoices = invoice::aging_report(&conn)?;
    Ok(Json(invoices))
}
