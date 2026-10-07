use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AddWebsite {
    pub url: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String
}