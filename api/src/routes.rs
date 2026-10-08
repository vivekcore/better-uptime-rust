
use axum::{routing::post,routing::get,Router};
use store::Store;


use crate::{services::user::create_user};
use crate::{services::test::health_check};
use crate::{services::website::{
        create_website,
        fetch_websites
}};

pub fn routes() -> Router<Store> {
 Router::new()
        .route("/", get(health_check))
        .route("/create-user", post(create_user))
        .route("/get-site", get(fetch_websites))
        .route("/add-site", post(create_website))
}






