use crate::db::models::User;
use sqlx::MySqlPool;

pub async fn get_all_users(pool: &MySqlPool) -> Vec<User> {
    sqlx::query_as!(User, "SELECT id, username, avatar_url FROM Users")
        .fetch_all(pool)
        .await
        .expect("Error fetching users")
}

pub async fn get_user_by_id(pool: &MySqlPool, id: i32) -> User {
    sqlx::query_as!(
        User,
        "SELECT id, username, avatar_url FROM Users WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
    .expect("Error fetching user")
}
