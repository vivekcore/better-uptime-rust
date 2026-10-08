
use sqlx::FromRow;
use serde::Deserialize;

#[derive(Debug, FromRow, Deserialize)]
pub struct AddWebsite {
    pub url: String
}