use axum::{Json, extract::State, http::StatusCode};

use crate::{CreateUser, CreateUserRes};
use store::Store;


pub async fn create_user(
    State(store): State<Store>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<CreateUserRes>, (StatusCode, String)> {
    let user = CreateUser {
        username: payload.username,
        password: payload.password,
    };

    let response = store
        .create_user(&user.username, &user.password)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}
