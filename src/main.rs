use sqlx::mysql::MySqlPoolOptions;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Connected to the database!");

    // Example query
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM Users")
        .fetch_one(&pool)
        .await?;

    println!("Number of users: {}", row.0);

    Ok(())
}
