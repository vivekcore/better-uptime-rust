pub mod services;
pub mod routes;


use store::{Store, create_pool};
use store::models::user::CreateUser;
use store::models::user_res::CreateUserRes;



#[tokio::main]
async fn main() {
    let pool = create_pool().await; 
    print!("Databse conneccted!"); 
    let store = Store { pool };
   
    let app = axum::Router::new().merge(routes::routes()).with_state(store);
    
    
    let listener = tokio::net::TcpListener::bind("localhost:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
