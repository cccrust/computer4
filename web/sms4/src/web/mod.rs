pub mod error;
pub mod routes;

use std::sync::{Arc, Mutex};

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
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
        let dist = resolve_dist_dir();
        let dist_path = dist.clone();
        app = app
            .nest_service("/assets", ServeDir::new(dist.join("assets")))
            .fallback(get(move || {
                let dist = dist_path.clone();
                async move {
                    match tokio::fs::read_to_string(dist.join("index.html")).await {
                        Ok(html) => Html(html).into_response(),
                        Err(_) => (StatusCode::NOT_FOUND, "前端尚未建置，請執行 cd web && npm run build").into_response(),
                    }
                }
            }));
    }

    app
}

fn resolve_dist_dir() -> std::path::PathBuf {
    let candidates = [
        std::path::PathBuf::from("web/dist"),
        std::path::PathBuf::from("../web/dist"),
    ];
    for c in &candidates {
        if c.exists() {
            return c.clone();
        }
    }
    std::path::PathBuf::from("web/dist")
}

pub async fn start(conn: Connection, host: &str, port: u16, dev_mode: bool) {
    let app = build_app(conn, dev_mode);
    let addr: std::net::SocketAddr = format!("{}:{}", host, port).parse().expect("無效的位址");
    println!("Web 伺服器啟動於 http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
