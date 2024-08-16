use crate::db::models::Player;
use sqlx::{MySql, MySqlPool, QueryBuilder};

pub async fn get_all_players(pool: &MySqlPool) -> Vec<Player> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players"
    )
    .fetch_all(pool)
    .await
    .expect("Error fetching players")
}

pub async fn get_remaining_players(pool: &MySqlPool) -> Vec<Player> {
    // get players with eliminated field false
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players WHERE eliminated = 0"
    )
    .fetch_all(pool)
    .await
    .expect("Error fetching remaining players")
}

pub async fn eliminate_player(pool: &MySqlPool, player_id: i32) {
    sqlx::query!("UPDATE Players SET eliminated = 1 WHERE id = ?", player_id)
        .execute(pool)
        .await
        .expect("Error eliminating player");
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

pub async fn bulk_create_players(pool: &MySqlPool, players: Vec<Player>) {
    const BIND_LIMIT: usize = 65535;
    let mut query_builder: QueryBuilder<MySql> =
        QueryBuilder::new("INSERT INTO Players(id, username, avatar_url, country, rank) ");
    query_builder.push_values(players.into_iter().take(BIND_LIMIT / 5), |mut b, user| {
        b.push_bind(user.id)
            .push_bind(user.username)
            .push_bind(user.avatar_url)
            .push_bind(user.country)
            .push_bind(user.rank);
    });
    let mut query = query_builder.build();
    query.execute(pool).await.expect("Error creating players");
}

pub async fn delete_player(pool: &MySqlPool, id: i32) {
    sqlx::query!("DELETE FROM Players WHERE id = ?", id)
        .execute(pool)
        .await
        .expect("Error deleting player");
}

pub async fn get_player_price(pool: &MySqlPool, player_id: i32, round: String) -> i32 {
    let player_price = sqlx::query!(
        "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_one(pool)
    .await;

    if player_price.is_ok() {
        player_price.unwrap().price
    } else {
        0
    }
}

pub async fn update_player_price(pool: &MySqlPool, player_id: i32, round: String, price: f64) {
    // check if player price exists in PlayerPrices table for current round
    let player_price = sqlx::query!(
        "SELECT * FROM PlayerPrices WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_one(pool)
    .await;

    if player_price.is_ok() {
        sqlx::query!(
            "UPDATE PlayerPrices SET price = ?, round = ? WHERE player_id = ?",
            price,
            round,
            player_id
        )
        .execute(pool)
        .await
        .expect("Error updating player price");
    } else {
        sqlx::query!(
            "INSERT INTO PlayerPrices (player_id, price, round) VALUES (?, ?, ?)",
            player_id,
            price,
            round
        )
        .execute(pool)
        .await
        .expect("Error creating player price");
    }
}
