use crate::{AppState, CreateUser};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use axum::{Json, extract::State, http::StatusCode};
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
}

#[derive(Serialize)]
pub struct AuthResponse {
    id: String,
    username: String,
    created_at: DateTime<Utc>,
    token: String,
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let hash_pass = hash_password(&payload.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to hash password".to_string(),
        )
    })?;

    let user = CreateUser {
        username: payload.username,
        password: hash_pass,
    };

    let response = state
        .store
        .create_user(&user.username, &user.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_id = response.id.to_string();
    let token = sign_token(&user_id, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        id: user_id,
        username: response.username,
        created_at: response.created_at,
        token,
    }))
}

pub async fn sign_in(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = state
        .store
        .find_user_by_username(&payload.username)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid username or password".to_string(),
            )
        })?;

    if !verify_password(&payload.password, &user.password_hash) {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".to_string(),
        ));
    }

    let user_id = user.id.to_string();
    let token = sign_token(&user_id, &state.jwt_secret)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AuthResponse {
        id: user_id,
        username: user.username,
        created_at: user.created_at,
        token,
    }))
}

fn hash_password(pw: &str) -> Result<String, argon2::password_hash::Error> {
    Ok(Argon2::default().hash_password(pw.as_bytes())?.to_string())
}

fn verify_password(pw: &str, hash: &str) -> bool {
    match PasswordHash::new(hash) {
        Ok(parsed) => Argon2::default()
            .verify_password(pw.as_bytes(), &parsed)
            .is_ok(),
        Err(_) => false,
    }
}

fn sign_token(user_id: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id.to_owned(),
        iat: now.timestamp() as usize,
        exp: (now + Duration::hours(24)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
