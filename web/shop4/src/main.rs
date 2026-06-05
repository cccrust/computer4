mod auth;
mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::db::init_db;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "shop4=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_path = "/Users/Shared/ccc/project/computer4/web/shop4/shop4.db";
    if !std::path::Path::new(db_path).exists() {
        std::fs::File::create(db_path).expect("Failed to create database file");
    }
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| format!("sqlite:{}", db_path));

    let pool = init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    let pool = Arc::new(pool);

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/categories", get(handlers::get_categories))
        .route("/api/products", get(handlers::get_products))
        .route("/api/products", post(handlers::create_product))
        .route("/api/products/:id", get(handlers::get_product))
        .route("/api/products/:id", put(handlers::update_product))
        .route("/api/products/:id", delete(handlers::delete_product))
        .route("/api/products/:id/reviews", get(handlers::get_product_reviews))
        .route("/api/products/:id/reviews", post(handlers::create_review))
        .route("/api/cart", get(handlers::get_cart))
        .route("/api/cart", post(handlers::add_to_cart))
        .route("/api/cart/:id", put(handlers::update_cart_item))
        .route("/api/cart/:id", delete(handlers::remove_from_cart))
        .route("/api/orders", get(handlers::get_orders))
        .route("/api/orders", post(handlers::create_order))
        .route("/api/orders/:id", get(handlers::get_order))
        .route("/api/user/profile", get(handlers::get_user_profile))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind to port 8080");

    tracing::info!("Shop4 server running on http://0.0.0.0:8080");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}