use axum::{Json, Router, routing::get, routing::post};

mod req_input;
mod req_output;

use req_input::{AddWebsite, CreateUser};
use req_output::{AddWebsiteRespose, CreateUserResponse, GetWebsiteResponse};

use store::Store;

async fn hello() -> &'static str {
    "Hello, world!"
}

async fn get_website() -> Json<GetWebsiteResponse> {
    let store = Store {};

    let response = GetWebsiteResponse {
        message: "All websites ".to_string(),
        url: "all added urls".to_string(),
    };

    store.get_website();

    Json(response)
}

async fn create_website(Json(payload): Json<AddWebsite>) -> Json<AddWebsiteRespose> {
    let store = Store {};
    let response = AddWebsiteRespose {
        message: "Website added".to_string(),
        url: payload.url,
    };
    store.create_website();
    Json(response)
}

async fn create_user(Json(payload): Json<CreateUser>) -> Json<CreateUserResponse> {
    let store = Store {};

    let user = CreateUser {
        username: payload.username,
        password: payload.password,
    };

    store.create_user();

    let response = CreateUserResponse {
        name: user.username.clone(),
        message: "user created sucessfully".to_string(),
    };

    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/create-user", post(create_user))
        .route("/get-site", get(get_website))
        .route("/add-site", post(create_website));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
