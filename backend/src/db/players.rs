use crate::db::models::Player;
use sqlx::{mysql::MySqlQueryResult, Error, MySql, MySqlPool, QueryBuilder};

pub async fn get_all_players(pool: &MySqlPool) -> Result<Vec<Player>, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_remaining_players(pool: &MySqlPool) -> Result<Vec<Player>, Error> {
    // get players with eliminated field false
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players WHERE eliminated = 0"
    )
    .fetch_all(pool)
    .await
}

pub async fn eliminate_player(pool: &MySqlPool, player_id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("UPDATE Players SET eliminated = 1 WHERE id = ?", player_id)
        .execute(pool)
        .await
}

pub async fn get_player_by_id(pool: &MySqlPool, id: i32) -> Result<Player, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_player(pool: &MySqlPool, player: Player) -> Result<MySqlQueryResult, Error> {
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
}

pub async fn bulk_create_players(
    pool: &MySqlPool,
    players: Vec<Player>,
) -> Result<MySqlQueryResult, Error> {
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
    let query = query_builder.build();
    query.execute(pool).await
}

pub async fn delete_player(pool: &MySqlPool, id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("DELETE FROM Players WHERE id = ?", id)
        .execute(pool)
        .await
}

pub async fn get_player_price(
    pool: &MySqlPool,
    player_id: i32,
    round: String,
) -> Result<i32, Error> {
    let player_price = sqlx::query!(
        "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_one(pool)
    .await?;

    Ok(player_price.price)
}

pub async fn update_player_price(
    pool: &MySqlPool,
    player_id: i32,
    round: String,
    price: i32,
) -> Result<MySqlQueryResult, Error> {
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
    } else {
        sqlx::query!(
            "INSERT INTO PlayerPrices (player_id, price, round) VALUES (?, ?, ?)",
            player_id,
            price,
            round
        )
        .execute(pool)
        .await
    }
}
