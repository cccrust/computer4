use crate::error::AppError;
use crate::models::{ApiResponse, User, UserResponse};
use axum::{
    extract::State,
    http::HeaderMap,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

const JWT_SECRET: &[u8] = b"shop4_secret_key_change_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

pub async fn register(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user_exists: Option<(i64,)> =
        sqlx::query_as("SELECT COUNT(*) FROM users WHERE email = ? OR username = ?")
            .bind(&payload.email)
            .bind(&payload.username)
            .fetch_optional(&*pool)
            .await?;

    if let Some((count,)) = user_exists {
        if count > 0 {
            return Err(AppError::BadRequest(
                "Email or username already exists".to_string(),
            ));
        }
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| AppError::InternalError(e.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) VALUES (?, ?, ?, ?, 'user', ?, ?)",
    )
    .bind(&id)
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&now)
    .bind(&now)
    .execute(&*pool)
    .await?;

    let user = User {
        id: id.clone(),
        username: payload.username,
        email: payload.email,
        password_hash,
        role: "user".to_string(),
        created_at: now.clone(),
        updated_at: now,
    };

    let token = create_token(&user)?;

    Ok(Json(ApiResponse::success(AuthResponse { token, user: user.into() })))
}

pub async fn login(
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user: Option<User> =
        sqlx::query_as("SELECT * FROM users WHERE email = ?").bind(&payload.email).fetch_optional(&*pool).await?;

    let user = user.ok_or_else(|| AppError::AuthError("Invalid credentials".to_string()))?;

    let valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::AuthError("Invalid credentials".to_string()))?;

    if !valid {
        return Err(AppError::AuthError("Invalid credentials".to_string()));
    }

    let token = create_token(&user)?;

    Ok(Json(ApiResponse::success(AuthResponse {
        token,
        user: user.into(),
    })))
}

pub fn create_token(user: &User) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        role: user.role.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| AppError::InternalError(e.to_string()))
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::AuthError(e.to_string()))
}

pub fn extract_token(headers: &HeaderMap) -> Option<String> {
    headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|s| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let user = User {
            id: "test-id".to_string(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            role: "user".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
            updated_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let token = create_token(&user).unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, "test-id");
        assert_eq!(claims.role, "user");
    }
}