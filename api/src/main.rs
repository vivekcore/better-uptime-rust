use axum::{
    Json, Router, routing::get, routing::post
};
use serde::{Deserialize, Serialize};

async fn hello() -> &'static str {
    "Hello, world!"
}

#[derive(Serialize)]
struct GetWebsiteResponse {
    message: String,
    url: String,
}

async fn get_website() -> Json<GetWebsiteResponse>{
    Json(GetWebsiteResponse{
        message: "Your Websites".to_string(),
        url: "https://all.website.com".to_string()
    })
}

#[derive(Deserialize)]
struct  AddWebsite {
    url: String,
}

#[derive(Serialize)]
struct AddWebsiteRespose {
    message: String,
    url: String,
}
async fn add_website(
    Json(payload): Json<AddWebsite>,
) -> Json<AddWebsiteRespose> {
    Json(AddWebsiteRespose{
        message: "Website added".to_string(),
        url: payload.url
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/get-site", get(get_website))
        .route("/add-site", post(add_website));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}