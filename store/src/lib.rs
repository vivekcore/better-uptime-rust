use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::models::{user_res::CreateUserRes, website_res::AddWebsiteRes};

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
        password: &str
    ) -> Result<CreateUserRes,sqlx::Error> {

        let user = sqlx::query_as::<_,CreateUserRes>(
            r#"
                INSERT INTO users (username, password)
                VALUES ($1,$2)
                RETURNING id, username, created_at
            "#
        )
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(user)

    }

    pub async fn add_website(
        &self,
        url: &str
    ) -> Result<AddWebsiteRes, sqlx::Error> {
        
        let response = sqlx::query_as::<_,AddWebsiteRes>(
            r#"
                INSER INTO websites (url)
                VALUES ($ 1)
                RETURNING id, url, created_at
            "#
        )
        .bind(url)
        .fetch_one(&self.pool)
        .await?;

    Ok(response)
    }
    pub async  fn get_website(
        &self,
        user_id: String
    ) -> Result<Vec<AddWebsiteRes>, sqlx::Error>{

        let response = sqlx::query_as::<_,AddWebsiteRes>(
            r#"
                SELECT id, url, created_at
                FROM websites
                WHERE user_id = $1
                ORDER BY id DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

    Ok(response)
    }
}
