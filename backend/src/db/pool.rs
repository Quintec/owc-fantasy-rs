use sqlx::mysql::MySqlPoolOptions;
use std::time::Duration;

pub async fn create_pool() -> sqlx::MySqlPool {
    let database_url = std::env::var("DATABASE_URL").unwrap();
    println!("Attempting to connect to: {}", database_url);
    
    MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await
        .expect("Error creating db pool")
}
