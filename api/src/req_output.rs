use serde::{Serialize};

#[derive(Serialize)]
pub struct GetWebsiteResponse {
    pub message: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct AddWebsiteRespose {
    pub message: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub name: String,
    pub message: String
}