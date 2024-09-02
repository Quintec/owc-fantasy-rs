use crate::db::models::User;
use sqlx::{mysql::MySqlQueryResult, Error, MySqlPool};

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

pub async fn create_user(pool: &MySqlPool, user: User) -> Result<MySqlQueryResult, Error> {
    sqlx::query_as!(
        User,
        "INSERT INTO Users (id, username, avatar_url) VALUES (?, ?, ?)",
        user.id,
        user.username,
        user.avatar_url
    )
    .execute(pool)
    .await
}
