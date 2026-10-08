use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct CreateUserRes {
    pub id: i32,
    pub username: String,
    pub created_at: DateTime<Utc>,
}