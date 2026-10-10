pub mod middleware;
pub mod routes;
pub mod services;

use axum::Router;
use store::models::user::CreateUser;
use store::{Store, create_pool};
#[derive(Clone)]
pub struct AppState {
    pub store: Store,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    println!("Database connected!");

    let state = AppState {
        store: Store { pool },
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET not set"),
    };

    let app = Router::new()
        .merge(routes::pub_routes())
        .merge(routes::protected_routes(state.clone()))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
