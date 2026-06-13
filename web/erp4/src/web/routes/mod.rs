pub mod customer;
pub mod dashboard;
pub mod invoice;
pub mod order;
pub mod product;
pub mod purchase_order;
pub mod report;
pub mod supplier;

use axum::routing::{delete, get, post, put};
use axum::Router;

pub fn build_routes() -> Router<crate::web::AppState> {
    Router::new()
        .route("/login", post(super::auth::login))
        .route("/me", get(super::auth::me))
        .route("/dashboard", get(dashboard::dashboard))
        .route("/customers", get(customer::list).post(customer::create))
        .route(
            "/customers/{id}",
            get(customer::get)
                .put(customer::update)
                .delete(customer::delete),
        )
        .route("/products", get(product::list).post(product::create))
        .route(
            "/products/{id}",
            get(product::get)
                .put(product::update)
                .delete(product::delete),
        )
        .route("/suppliers", get(supplier::list).post(supplier::create))
        .route(
            "/suppliers/{id}",
            get(supplier::get)
                .put(supplier::update)
                .delete(supplier::delete),
        )
        .route("/orders", get(order::list).post(order::create))
        .route("/orders/{id}", get(order::get).delete(order::delete))
        .route("/orders/{id}/status", post(order::update_status))
        .route(
            "/orders/{id}/items",
            get(order::list_items).post(order::add_item),
        )
        .route(
            "/purchase-orders",
            get(purchase_order::list).post(purchase_order::create),
        )
        .route(
            "/purchase-orders/{id}",
            get(purchase_order::get).delete(purchase_order::delete),
        )
        .route(
            "/purchase-orders/{id}/status",
            post(purchase_order::update_status),
        )
        .route(
            "/purchase-orders/{id}/items",
            get(purchase_order::list_items).post(purchase_order::add_item),
        )
        .route("/invoices", get(invoice::list).post(invoice::create))
        .route("/invoices/{id}", get(invoice::get).delete(invoice::delete))
        .route("/invoices/{id}/status", post(invoice::update_status))
        .route("/reports/sales", get(report::sales))
        .route("/reports/inventory", get(report::inventory))
        .route("/reports/aging", get(report::aging))
}
