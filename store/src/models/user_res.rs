use chrono::{DateTime, Utc};
use sqlx::FromRow;
use serde::Serialize;
#[derive(Debug, FromRow, Serialize)]
pub struct CreateUserRes {
    pub id: String,
    pub username: String,
    pub created_at: DateTime<Utc>,
}