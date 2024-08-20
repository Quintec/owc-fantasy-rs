use crate::db::models::User;
use sqlx::{Error, MySqlPool};

pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, Error> {
    sqlx::query_as!(User, "SELECT id, username, avatar_url FROM Users")
        .fetch_all(pool)
        .await
}

pub async fn get_user_by_id(pool: &MySqlPool, id: i32) -> Result<User, Error> {
    sqlx::query_as!(
        User,
        "SELECT id, username, avatar_url FROM Users WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}
