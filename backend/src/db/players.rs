use crate::db::models::Player;
use sqlx::MySqlPool;

pub async fn get_all_players(pool: &MySqlPool) -> Vec<Player> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players"
    )
    .fetch_all(pool)
    .await
    .expect("Error fetching players")
}

pub async fn get_player_by_id(pool: &MySqlPool, id: i32) -> Player {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
    .expect("Error fetching player")
}

pub async fn create_player(pool: &MySqlPool, player: Player) {
    sqlx::query!(
        "INSERT INTO Players (id, username, avatar_url, country, rank) VALUES (?, ?, ?, ?, ?)",
        player.id,
        player.username,
        player.avatar_url,
        player.country,
        player.rank
    )
    .execute(pool)
    .await
    .expect("Error creating player");
}

pub async fn update_player_price(pool: &MySqlPool, player_id: i32, price: dou)
