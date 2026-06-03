mod api;
mod terminal;
mod fs_ops;

use axum::{
    extract::{State, WebSocketUpgrade, ws::{WebSocket, Message}},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub terminals: DashMap<String, terminal::TerminalSession>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = Arc::new(AppState {
        terminals: DashMap::new(),
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/fs/list", get(api::fs_list))
        .route("/api/fs/read", get(api::fs_read))
        .route("/api/fs/write", post(api::fs_write))
        .route("/api/fs/mkdir", post(api::fs_mkdir))
        .route("/api/fs/delete", post(api::fs_delete))
        .route("/api/fs/rename", post(api::fs_rename))
        .route("/api/proxy", get(api::proxy_handler))
        .route("/ws/terminal", get(terminal_ws_handler))
        .fallback_service(tower_http::services::ServeDir::new("static"))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("os4web listening on {}", addr);
    println!("🚀 os4web server running at http://localhost:3000");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



async fn terminal_ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| terminal::handle_terminal_ws(socket, state))
}
