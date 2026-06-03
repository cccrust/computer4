use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::SharedState;

fn safe_path(raw: &str) -> Option<PathBuf> {
    let p = PathBuf::from(raw);
    // Only allow absolute paths or relative under home
    if p.to_string_lossy().contains("..") {
        return None;
    }
    Some(p)
}

#[derive(Deserialize)]
pub struct PathQuery {
    pub path: String,
}

#[derive(Serialize)]
pub struct FsEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<u64>,
    pub extension: Option<String>,
}

#[derive(Serialize)]
pub struct FsResponse<T> {
    pub ok: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> FsResponse<T> {
    pub fn ok(data: T) -> Json<Self> {
        Json(Self { ok: true, data: Some(data), error: None })
    }
    pub fn err(msg: &str) -> Json<FsResponse<()>> {
        Json(FsResponse { ok: false, data: None, error: Some(msg.to_string()) })
    }
}

pub async fn fs_list(
    Query(q): Query<PathQuery>,
    State(_state): State<SharedState>,
) -> impl axum::response::IntoResponse {
    let path = match safe_path(&q.path) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };

    match fs::read_dir(&path) {
        Ok(entries) => {
            let mut items: Vec<serde_json::Value> = entries
                .filter_map(|e| e.ok())
                .map(|e| {
                    let meta = e.metadata().ok();
                    let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
                    let size = meta.as_ref().map(|m| m.len()).unwrap_or(0);
                    let modified = meta.as_ref().and_then(|m| {
                        m.modified().ok().and_then(|t| {
                            t.duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs())
                        })
                    });
                    let name = e.file_name().to_string_lossy().to_string();
                    let full_path = e.path().to_string_lossy().to_string();
                    let ext = if !is_dir {
                        Path::new(&name).extension().map(|x| x.to_string_lossy().to_string())
                    } else { None };
                    serde_json::json!({
                        "name": name,
                        "path": full_path,
                        "is_dir": is_dir,
                        "size": size,
                        "modified": modified,
                        "extension": ext,
                    })
                })
                .collect();

            // Sort: dirs first, then by name
            items.sort_by(|a, b| {
                let a_dir = a["is_dir"].as_bool().unwrap_or(false);
                let b_dir = b["is_dir"].as_bool().unwrap_or(false);
                if a_dir != b_dir {
                    b_dir.cmp(&a_dir)
                } else {
                    a["name"].as_str().unwrap_or("").cmp(b["name"].as_str().unwrap_or(""))
                }
            });

            Json(serde_json::json!({ "ok": true, "data": items }))
        }
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

pub async fn fs_read(
    Query(q): Query<PathQuery>,
    State(_state): State<SharedState>,
) -> impl axum::response::IntoResponse {
    let path = match safe_path(&q.path) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };

    match fs::read(&path) {
        Ok(bytes) => {
            // Try UTF-8, else base64-like hex
            match String::from_utf8(bytes.clone()) {
                Ok(text) => Json(serde_json::json!({ "ok": true, "data": { "content": text, "binary": false } })),
                Err(_) => {
                    let encoded = bytes.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
                    Json(serde_json::json!({ "ok": true, "data": { "content": encoded, "binary": true } }))
                }
            }
        }
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct WriteBody {
    pub path: String,
    pub content: String,
}

pub async fn fs_write(
    State(_state): State<SharedState>,
    Json(body): Json<WriteBody>,
) -> impl axum::response::IntoResponse {
    let path = match safe_path(&body.path) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };
    match fs::write(&path, &body.content) {
        Ok(_) => Json(serde_json::json!({ "ok": true })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct MkdirBody { pub path: String }

pub async fn fs_mkdir(
    State(_state): State<SharedState>,
    Json(body): Json<MkdirBody>,
) -> impl axum::response::IntoResponse {
    let path = match safe_path(&body.path) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };
    match fs::create_dir_all(&path) {
        Ok(_) => Json(serde_json::json!({ "ok": true })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct DeleteBody { pub path: String }

pub async fn fs_delete(
    State(_state): State<SharedState>,
    Json(body): Json<DeleteBody>,
) -> impl axum::response::IntoResponse {
    let path = match safe_path(&body.path) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };
    let result = if path.is_dir() {
        fs::remove_dir_all(&path)
    } else {
        fs::remove_file(&path)
    };
    match result {
        Ok(_) => Json(serde_json::json!({ "ok": true })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct RenameBody { pub from: String, pub to: String }

pub async fn fs_rename(
    State(_state): State<SharedState>,
    Json(body): Json<RenameBody>,
) -> impl axum::response::IntoResponse {
    let from = match safe_path(&body.from) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };
    let to = match safe_path(&body.to) {
        Some(p) => p,
        None => return Json(serde_json::json!({ "ok": false, "error": "Invalid path" })),
    };
    match fs::rename(&from, &to) {
        Ok(_) => Json(serde_json::json!({ "ok": true })),
        Err(e) => Json(serde_json::json!({ "ok": false, "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct ProxyQuery {
    pub url: String,
}

pub async fn proxy_handler(Query(q): Query<ProxyQuery>) -> axum::response::Response {
    let client = reqwest::Client::new();
    match client.get(&q.url).header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36").send().await {
        Ok(res) => {
            let mut response_builder = axum::response::Response::builder().status(res.status().as_u16());
            let mut is_html = false;
            
            if let Some(headers) = response_builder.headers_mut() {
                for (name, value) in res.headers() {
                    let name_str = name.as_str().to_lowercase();
                    if name_str != "x-frame-options" 
                        && name_str != "content-security-policy"
                        && name_str != "strict-transport-security"
                        && name_str != "transfer-encoding"
                        && name_str != "content-encoding"
                    {
                        if name_str == "content-type" && value.to_str().unwrap_or("").contains("text/html") {
                            is_html = true;
                        }
                        headers.insert(name.clone(), value.clone());
                    }
                }
            }
            
            if let Ok(bytes) = res.bytes().await {
                let mut body_bytes = bytes.to_vec();
                if is_html {
                    if let Ok(html_str) = String::from_utf8(body_bytes.clone()) {
                        let base_url = format!("<base href=\"{}\">", q.url);
                        let injected = if html_str.contains("<head>") {
                            html_str.replacen("<head>", &format!("<head>\n{}", base_url), 1)
                        } else {
                            format!("{}\n{}", base_url, html_str)
                        };
                        body_bytes = injected.into_bytes();
                    }
                }
                response_builder.body(axum::body::Body::from(body_bytes)).unwrap_or_else(|_| {
                    axum::response::Response::builder().status(500).body(axum::body::Body::from("Error")).unwrap()
                })
            } else {
                axum::response::Response::builder().status(500).body(axum::body::Body::from("Failed to read body")).unwrap()
            }
        },
        Err(e) => {
            axum::response::Response::builder().status(502).body(axum::body::Body::from(format!("Proxy error: {}", e))).unwrap()
        }
    }
}
