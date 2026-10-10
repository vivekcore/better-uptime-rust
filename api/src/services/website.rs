use crate::AppState;
use crate::middleware::auth::Claims;
use axum::Extension;
use axum::{Json, extract::State, http::StatusCode};
use store::models::{website::AddWebsite, website_res::AddWebsiteRes};
use uuid::Uuid;

pub async fn create_website(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AddWebsite>,
) -> Result<Json<AddWebsiteRes>, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid user ID in token".to_string(),
        )
    })?;

    let response = state
        .store
        .add_website(&payload.url, user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}

pub async fn fetch_websites(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<AddWebsiteRes>>, (StatusCode, String)> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            "Invalid user ID in token".to_string(),
        )
    })?;
    let response = state
        .store
        .get_website(user_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(response))
}
