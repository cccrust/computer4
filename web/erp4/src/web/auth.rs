use crate::web::error::AppError;
use crate::web::AppState;
use anyhow::Result;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
}

fn jwt_secret() -> String {
    std::env::var("ERP4_JWT_SECRET")
        .unwrap_or_else(|_| "erp4-dev-secret-key-change-in-production".into())
}

pub fn create_token(
    user_id: i64,
    username: &str,
    display_name: &str,
    role: &str,
) -> Result<String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        display_name: display_name.to_string(),
        role: role.to_string(),
        exp: now + 86400,
    };
    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )?)
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let conn = state.conn.lock().unwrap();
    let u = user::authenticate_user(&conn, &req.username, &req.password)?
        .ok_or_else(|| AppError::Unauthorized("帳號或密碼錯誤".into()))?;
    let display_name = u.display_name.clone();
    let token = create_token(u.id, &u.username, &display_name, &u.role)?;
    Ok(Json(LoginResponse {
        token,
        user: UserInfo {
            id: u.id,
            username: u.username,
            display_name,
            role: u.role,
        },
    }))
}

pub async fn me(auth: AuthUser) -> Json<UserInfo> {
    Json(UserInfo {
        id: auth.id,
        username: auth.username,
        display_name: auth.display_name,
        role: auth.role,
    })
}

fn extract_bearer_token(parts: &Parts) -> Option<&str> {
    let header = parts.headers.get("authorization")?;
    let value = header.to_str().ok()?;
    value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))
}

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = extract_bearer_token(parts).ok_or(AuthRejection)?;
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret().as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AuthRejection)?;
        Ok(AuthUser {
            id: data.claims.sub,
            username: data.claims.username,
            display_name: data.claims.display_name,
            role: data.claims.role,
        })
    }
}

#[derive(Debug)]
pub struct AuthRejection;

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, Json(json!({ "error": "未授權" }))).into_response()
    }
}

use crate::model::user;
