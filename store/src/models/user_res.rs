use chrono::{DateTime, Utc,};
use uuid::Uuid;
use sqlx::FromRow;
use serde::Serialize;
#[derive(Debug, FromRow, Serialize)]
pub struct CreateUserRes {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
}