use crate::db::models::Player;
use sqlx::{Error, MySql, MySqlPool, QueryBuilder, mysql::MySqlQueryResult};
use std::collections::HashMap;

pub async fn get_all_players(pool: &MySqlPool) -> Result<Vec<Player>, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, `rank` FROM Players"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_remaining_players(pool: &MySqlPool) -> Result<Vec<Player>, Error> {
    // get players with eliminated field false
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, `rank` FROM Players WHERE eliminated = 0"
    )
    .fetch_all(pool)
    .await
}

pub async fn eliminate_player(pool: &MySqlPool, player_id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("UPDATE Players SET eliminated = 1 WHERE id = ?", player_id)
        .execute(pool)
        .await
}

// just in case we made a mistake
pub async fn uneliminate_player(pool: &MySqlPool, player_id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("UPDATE Players SET eliminated = 0 WHERE id = ?", player_id)
        .execute(pool)
        .await
}

pub async fn get_player_by_id(pool: &MySqlPool, id: i32) -> Result<Player, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, `rank` FROM Players WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_player(pool: &MySqlPool, player: Player) -> Result<MySqlQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO Players (id, username, avatar_url, country, `rank`) VALUES (?, ?, ?, ?, ?)",
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
        QueryBuilder::new("INSERT INTO Players(id, username, avatar_url, country, `rank`) ");
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

pub async fn get_player_round_score(pool: &MySqlPool, player_id: i32, round: String) -> Result<i32, Error> {
    let player_score = sqlx::query!(
        "SELECT score FROM PlayerScores WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_one(pool)
    .await?;

    Ok(player_score.score)
}

pub async fn update_player_round_score(
    pool: &MySqlPool,
    player_id: i32,
    round: String,
    score: i32,
) -> Result<MySqlQueryResult, Error> {
    let player_score = sqlx::query!(
        "SELECT * FROM PlayerScores WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_one(pool)
    .await;

    if player_score.is_ok() {
        sqlx::query!(
            "UPDATE PlayerScores SET score = ? WHERE player_id = ? AND round = ?",
            score,
            player_id,
            round
        )
        .execute(pool)
        .await
    } else {
        sqlx::query!(
            "INSERT INTO PlayerScores (player_id, score, round) VALUES (?, ?, ?)",
            player_id,
            score,
            round
        )
        .execute(pool)
        .await
    }
}

/// Create a team for a user from a list of player IDs with validation.
///
/// Validation rules:
/// - Exactly 8 players
/// - At most 2 players per country
/// - Total price for the given round must be <= 100_000_000
/// - Captain (if provided) must be one of the 8 players
pub async fn create_team_from_players(
    pool: &MySqlPool,
    user_id: i32,
    player_ids: Vec<i32>,
    round: String,
    captain_id: Option<i32>,
) -> Result<MySqlQueryResult, Error> {
    // Validate count
    if player_ids.len() != 8 {
        return Err(Error::Protocol(format!(
            "validation failed: team must contain exactly 8 players (got {})",
            player_ids.len()
        )));
    }

    // Validate country limits and total price
    let mut country_counts: HashMap<String, i32> = HashMap::new();
    let mut total_price: i64 = 0;
    const MAX_BUDGET: i64 = 100_000_000;

    for pid in &player_ids {
        let rec = sqlx::query!(
            "SELECT country FROM Players WHERE id = ?",
            pid
        )
        .fetch_one(pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => Error::Protocol(format!(
                "validation failed: player with ID {} does not exist",
                pid
            )),
            _ => e,
        })?;

        let country = rec.country;
        let count = country_counts.entry(country.clone()).or_insert(0);
        *count += 1;
        if *count > 2 {
            return Err(Error::Protocol(format!(
                "validation failed: more than 2 players from country {}",
                country
            )));
        }

        let price_rec = sqlx::query!(
            "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
            pid,
            &round
        )
        .fetch_one(pool)
        .await
        .map_err(|e| e)?;

        total_price += price_rec.price as i64;
    }

    if total_price > MAX_BUDGET {
        return Err(Error::Protocol(format!(
            "validation failed: total team price {} exceeds max budget {}",
            total_price, MAX_BUDGET
        )));
    }

    // create or update team
    let mut tx = pool.begin().await?;

    // Check if a team already exists for this user and round
    let existing = sqlx::query!(
        "SELECT id FROM Teams WHERE user_id = ? AND round = ?",
        user_id,
        &round
    )
    .fetch_optional(&mut *tx)
    .await?;

    let team_id: i32;
    let mut final_res: Option<MySqlQueryResult> = None;

    if let Some(rec) = existing {
        team_id = rec.id;

        // Update existing team with captain
        let update_res = sqlx::query!(
            "UPDATE Teams SET captain_id = ? WHERE id = ?",
            captain_id,
            team_id
        )
        .execute(&mut *tx)
        .await?;

        final_res = Some(update_res);

        // remove existing players for this team
        let del_res = sqlx::query!(
            "DELETE FROM TeamPlayers WHERE team_id = ?",
            team_id
        )
        .execute(&mut *tx)
        .await?;

        final_res = Some(del_res);
    } else {
        // create new team
        let team_res = sqlx::query!(
            "INSERT INTO Teams (user_id, round, captain_id) VALUES (?, ?, ?)",
            user_id,
            &round,
            captain_id
        )
        .execute(&mut *tx)
        .await?;

        team_id = team_res.last_insert_id() as i32;
        final_res = Some(team_res);
    }

    // insert players
    for pid in &player_ids {
        let res = sqlx::query!(
            "INSERT INTO TeamPlayers (team_id, player_id) VALUES (?, ?)",
            team_id,
            pid
        )
        .execute(&mut *tx)
        .await?;

        final_res = Some(res);
    }

    // Commit the transaction
    tx.commit().await?;

    final_res.ok_or_else(|| Error::Protocol("no DB operation performed".to_string()))
}