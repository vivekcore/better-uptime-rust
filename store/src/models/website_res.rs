use chrono::{DateTime, Utc};

use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(serde::Serialize,FromRow,Debug)]
pub struct AddWebsiteRes {
   pub id: Uuid,
   pub url: String,
   created_at: DateTime<Utc>
}