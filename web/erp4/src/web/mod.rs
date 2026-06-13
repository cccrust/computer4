pub mod auth;
pub mod error;
pub mod routes;

use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use axum::Router;
use rusqlite::Connection;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<Mutex<Connection>>,
}

pub fn build_app(conn: Connection, dev_mode: bool) -> Router {
    let state = AppState {
        conn: Arc::new(Mutex::new(conn)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = routes::build_routes();

    let mut app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        .with_state(state);

    if !dev_mode {
        #[cfg(debug_assertions)]
        let dist = "../web/dist";
        #[cfg(not(debug_assertions))]
        let dist = "web/dist";
        app = app.fallback_service(ServeDir::new(dist));
    }

    app
}

pub async fn start(conn: Connection, host: &str, port: u16, dev_mode: bool) {
    let app = build_app(conn, dev_mode);
    let addr: SocketAddr = format!("{}:{}", host, port).parse().expect("無效的位址");
    println!("Web 伺服器啟動於 http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
