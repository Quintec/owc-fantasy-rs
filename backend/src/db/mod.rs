use sqlx::mysql::MySqlPoolOptions;

pub async fn create_pool() -> sqlx::MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap()
}
