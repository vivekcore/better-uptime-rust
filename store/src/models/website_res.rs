use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;



#[derive(Serialize,FromRow,Debug)]
pub struct AddWebsiteRes {
   pub id: String,
   pub url: String,
   created_at: DateTime<Utc>
}