use serde::Deserialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize)]
pub struct AddWebsite {
    pub url: String,
}
