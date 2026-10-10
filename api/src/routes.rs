use crate::AppState;
use crate::middleware::auth::auth;
use crate::services::test::health_check;
use crate::services::user::{sign_in, signup};
use crate::services::website::{create_website, fetch_websites};
use axum::middleware;
use axum::{Router, routing::get, routing::post};

pub fn pub_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(health_check))
        .route("/signup", post(signup))
        .route("/signin", post(sign_in))
}

pub fn protected_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/get-sites", get(fetch_websites))
        .route("/add-site", post(create_website))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth))
}
