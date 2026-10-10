use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::models::{
    user_res::{CreateUserRes, UserAuth},
    website_res::AddWebsiteRes,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
}
pub mod models;

pub async fn create_pool() -> PgPool {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database")
}

impl Store {
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
    ) -> Result<CreateUserRes, sqlx::Error> {
        let user = sqlx::query_as::<_, CreateUserRes>(
            r#"
                INSERT INTO users (username, password)
                VALUES ($1,$2)
                RETURNING id, username, created_at
            "#,
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserAuth>, sqlx::Error> {
        sqlx::query_as::<_, UserAuth>(
            "SELECT id, username, password AS password_hash, created_at FROM users WHERE username = $1",
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn add_website(
        &self,
        url: &str,
        user_id: Uuid,
    ) -> Result<AddWebsiteRes, sqlx::Error> {
        let response = sqlx::query_as::<_, AddWebsiteRes>(
            r#"
                INSERT INTO websites (url, user_id)
                VALUES ($1, $2)
                RETURNING id, url, time_added
            "#,
        )
        .bind(url)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(response)
    }
    pub async fn get_website(&self, user_id: Uuid) -> Result<Vec<AddWebsiteRes>, sqlx::Error> {
        let response = sqlx::query_as::<_, AddWebsiteRes>(
            r#"
                SELECT id, url, time_added
                FROM websites
                WHERE user_id = $1
                ORDER BY id DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(response)
    }
}
