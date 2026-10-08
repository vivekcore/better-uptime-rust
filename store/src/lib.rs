use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::models::user_res::CreateUserRes;

pub struct Store {}
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
        pool: &PgPool,
        username: &str,
        password: &str
    ) -> Result<CreateUserRes,sqlx::Error> {

        let user = sqlx::query_as::<_,CreateUserRes>(
            r#"
                INSER INTO users (username, password)
                VALUES ($1,$2)
                RETURNING id, username, created_at
            "#
        )
        .bind(username)
        .bind(password)
        .fetch_one(pool)
        .await?;
        
        Ok(user)

    }
    pub fn create_website(&self) -> String {
        String::from("1")
    }
    pub fn get_website(&self) {
        print!("get website fn called")
    }
}
