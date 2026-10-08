
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}