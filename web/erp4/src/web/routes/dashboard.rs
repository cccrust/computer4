use crate::model::{customer, invoice, order, product, purchase_order};
use crate::web::auth::AuthUser;
use crate::web::error::AppError;
use crate::web::AppState;
use axum::extract::State;
use axum::Json;

pub async fn dashboard(
    State(state): State<AppState>,
    _auth: AuthUser,
) -> Result<Json<serde_json::Value>, AppError> {
    let conn = state.conn.lock().unwrap();

    let customer_count = customer::list_customers(&conn, None, None, None)?.len();
    let product_count = product::list_products(&conn, None, None, None, None)?.len();
    let low_stock_count = product::list_products(&conn, None, Some(10), None, None)?.len();
    let pending_orders = order::list_orders(&conn, Some("pending"), None, None, None)?.len();
    let pending_pos =
        purchase_order::list_purchase_orders(&conn, Some("pending"), None, None)?.len();

    let overdue_total: f64 = invoice::aging_report(&conn)?
        .iter()
        .map(|inv| inv.amount)
        .sum();

    let recent_orders = order::list_orders(&conn, None, None, Some(1), Some(5))?;

    Ok(Json(serde_json::json!({
        "customer_count": customer_count,
        "product_count": product_count,
        "low_stock_count": low_stock_count,
        "pending_orders": pending_orders,
        "pending_pos": pending_pos,
        "overdue_invoices_total": overdue_total,
        "recent_orders": recent_orders,
    })))
}
