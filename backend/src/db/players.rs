use crate::db::models::{Player, PlayerWithPrice, PlayerWithScore};
use crate::util::round::Round;
use sqlx::{Error, MySql, MySqlPool, QueryBuilder, mysql::MySqlQueryResult};
use std::collections::HashMap;

pub async fn get_all_players(pool: &MySqlPool) -> Result<Vec<PlayerWithScore>, Error> {
    let current_round = crate::util::round::compute_round();
    let previous_round = get_previous_round(current_round.as_str());
    
    let round_str = previous_round.as_deref().unwrap_or(current_round.as_str());
    
    sqlx::query_as!(
        PlayerWithScore,
        "SELECT p.id, p.username, p.avatar_url, p.country, p.`rank`, p.eliminated, CAST(COALESCE(ps.score, 0) AS SIGNED) as `score: i32`
         FROM Players p
         LEFT JOIN PlayerScores ps ON p.id = ps.player_id AND ps.round = ?
         ORDER BY p.id",
        round_str
    )
    .fetch_all(pool)
    .await
}

pub async fn get_all_players_by_round(pool: &MySqlPool, round: String) -> Result<Vec<PlayerWithScore>, Error> {
    sqlx::query_as!(
        PlayerWithScore,
        "SELECT p.id, p.username, p.avatar_url, p.country, p.`rank`, p.eliminated, CAST(COALESCE(ps.score, 0) AS SIGNED) as `score: i32`
         FROM Players p
         LEFT JOIN PlayerScores ps ON p.id = ps.player_id AND ps.round = ?
         ORDER BY p.id",
        round
    )
    .fetch_all(pool)
    .await
}

pub async fn get_active_players(pool: &MySqlPool) -> Result<Vec<Player>, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, `rank`, eliminated FROM Players WHERE eliminated = 0"
    )
    .fetch_all(pool)
    .await
}

/// Get remaining (non-eliminated) players with prices for a specific round
/// If a player doesn't have a price set, falls back to: previous round -> rank-based calculation
pub async fn get_remaining_players_with_prices(
    pool: &MySqlPool,
    round: String,
) -> Result<Vec<PlayerWithPrice>, Error> {
    // Determine previous round for fallback pricing
    let previous_round = get_previous_round(&round);
    
    // Fetch players with current and previous round prices
    let query_result = sqlx::query!(
        r#"
        SELECT 
            p.id,
            p.username,
            p.avatar_url,
            p.country,
            p.`rank`,
            p.eliminated,
            pp_current.price as current_price,
            pp_prev.price as prev_price
        FROM Players p
        LEFT JOIN PlayerPrices pp_current ON p.id = pp_current.player_id AND pp_current.round = ?
        LEFT JOIN PlayerPrices pp_prev ON p.id = pp_prev.player_id AND pp_prev.round = ?
        WHERE p.eliminated = 0
        "#,
        round,
        previous_round
    )
    .fetch_all(pool)
    .await?;

    // Calculate final price with Rust logic for fallback
    let players = query_result
        .into_iter()
        .map(|row| {
            // Fallback order: current round price -> previous round price -> rank-based default
            let price = row.current_price
                .or(row.prev_price)
                .unwrap_or_else(|| rank_to_price_fallback(row.rank));

            PlayerWithPrice {
                id: row.id,
                username: row.username,
                avatar_url: row.avatar_url,
                country: row.country,
                rank: row.rank,
                eliminated: row.eliminated,
                price,
            }
        })
        .collect();

    Ok(players)
}

/// Rank-based pricing fallback: 6M-15M logarithmic curve
/// Used when no pScore available (higher rank = higher price)
fn rank_to_price_fallback(rank: i32) -> i32 {
    const MIN_PRICE: i32 = 6_000_000;
    const MAX_PRICE: i32 = 15_000_000;
    
    if rank <= 0 {
        return MIN_PRICE;
    }
    
    const K: f64 = 2_500_000.0;
    let log_price = MAX_PRICE as f64 - K * (rank as f64).ln();
    let price = log_price.max(MIN_PRICE as f64).min(MAX_PRICE as f64);
    ((price / 1000.0).round() * 1000.0) as i32
}

fn get_previous_round(round: &str) -> Option<String> {
    match round {
        "gf" => Some("f".to_string()),
        "f" => Some("sf".to_string()),
        "sf" => Some("qf".to_string()),
        "qf" => Some("ro16".to_string()),
        "ro16" => Some("ro32".to_string()),
        "ro32" => Some("ro64".to_string()),
        "ro64" => None, // First round, no previous
        _ => None,
    }
}

pub async fn eliminate_player(pool: &MySqlPool, player_id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("UPDATE Players SET eliminated = 1 WHERE id = ?", player_id)
        .execute(pool)
        .await
}

pub async fn uneliminate_player(pool: &MySqlPool, player_id: i32) -> Result<MySqlQueryResult, Error> {
    sqlx::query!("UPDATE Players SET eliminated = 0 WHERE id = ?", player_id)
        .execute(pool)
        .await
}

pub async fn get_player_by_id(pool: &MySqlPool, id: i32) -> Result<Player, Error> {
    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, `rank`, eliminated FROM Players WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn create_player(pool: &MySqlPool, player: Player) -> Result<MySqlQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO Players (id, username, avatar_url, country, `rank`, eliminated) VALUES (?, ?, ?, ?, ?, ?)",
        player.id,
        player.username,
        player.avatar_url,
        player.country,
        player.rank,
        player.eliminated
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
        QueryBuilder::new("INSERT INTO Players(id, username, avatar_url, country, `rank`, eliminated) ");
    query_builder.push_values(players.into_iter().take(BIND_LIMIT / 6), |mut b, user| {
        b.push_bind(user.id)
            .push_bind(user.username)
            .push_bind(user.avatar_url)
            .push_bind(user.country)
            .push_bind(user.rank)
            .push_bind(user.eliminated);
    });
    query_builder.push(" AS new ON DUPLICATE KEY UPDATE username = new.username, avatar_url = new.avatar_url, country = new.country, `rank` = new.`rank`");
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
    // Try to get price from PlayerPrices table
    let price_result = sqlx::query!(
        "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
        player_id,
        round
    )
    .fetch_optional(pool)
    .await?;

    match price_result {
        Some(rec) => Ok(rec.price),
        None => {
            // No price set, calculate default from player rank
            let player = sqlx::query!(
                "SELECT `rank` FROM Players WHERE id = ?",
                player_id
            )
            .fetch_one(pool)
            .await?;

            // Calculate default price based on rank
            // Formula: Better rank (lower number) = higher price
            // Price = max(1_000_000, 50_000_000 - (rank * 10_000))
            let base_price = 50_000_000i64 - (player.rank as i64 * 10_000);
            let default_price = base_price.max(1_000_000) as i32;
            
            Ok(default_price)
        }
    }
}

pub async fn update_player_price(
    pool: &MySqlPool,
    player_id: i32,
    round: String,
    price: i32,
) -> Result<MySqlQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO PlayerPrices (player_id, round, price) VALUES (?, ?, ?) 
         ON DUPLICATE KEY UPDATE price = VALUES(price)",
        player_id,
        round,
        price
    )
    .execute(pool)
    .await
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
    round: &Round,
    total_score: i32,
    match_count: i32,
) -> Result<(), sqlx::Error> {
    let round_str = format!("{:?}", round);
    
    // Calculate average score (rounded to nearest integer)
    let avg_score = if match_count > 0 {
        ((total_score as f32 / match_count as f32).round()) as i32
    } else {
        0
    };

    sqlx::query!(
        "INSERT INTO PlayerScores (player_id, round, score, total_score, match_count) VALUES (?, ?, ?, ?, ?) 
         ON DUPLICATE KEY UPDATE score = ?, total_score = ?, match_count = ?",
        player_id,
        round_str,
        avg_score,
        total_score,
        match_count,
        avg_score,
        total_score,
        match_count
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a team for a user from a list of player IDs with validation.
///
/// Validation rules:
/// - Exactly 8 players
/// - All players must exist in the database
/// - No eliminated players allowed
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
    if player_ids.len() != 8 {
        return Err(Error::Protocol(format!(
            "validation failed: team must contain exactly 8 players (got {})",
            player_ids.len()
        )));
    }

    let unique_players: std::collections::HashSet<_> = player_ids.iter().collect();
    if unique_players.len() != player_ids.len() {
        return Err(Error::Protocol(
            "validation failed: team contains duplicate players".to_string()
        ));
    }

    let mut country_counts: HashMap<String, i32> = HashMap::new();
    let mut total_price: i64 = 0;
    const MAX_BUDGET: i64 = 100_000_000;

    let previous_round = get_previous_round(&round);

    for pid in &player_ids {
        let rec = sqlx::query!(
            "SELECT country, eliminated, `rank` FROM Players WHERE id = ?",
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

        // Check if player is eliminated
        if rec.eliminated != 0 {
            return Err(Error::Protocol(format!(
                "validation failed: player with ID {} has been eliminated from the tournament",
                pid
            )));
        }

        let country = rec.country;
        let rank = rec.rank;
        let count = country_counts.entry(country.clone()).or_insert(0);
        *count += 1;
        if *count > 2 {
            return Err(Error::Protocol(format!(
                "validation failed: more than 2 players from country {}",
                country
            )));
        }

        let price = if let Some(price_rec) = sqlx::query!(
            "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
            pid,
            &round
        )
        .fetch_optional(pool)
        .await?
        {
            price_rec.price as i64
        } else if let Some(ref prev_round) = previous_round {
            if let Some(prev_price_rec) = sqlx::query!(
                "SELECT price FROM PlayerPrices WHERE player_id = ? AND round = ?",
                pid,
                prev_round
            )
            .fetch_optional(pool)
            .await?
            {
                prev_price_rec.price as i64
            } else {
                rank_to_price_fallback(rank) as i64
            }
        } else {
            rank_to_price_fallback(rank) as i64
        };

        total_price += price;
    }

    if total_price > MAX_BUDGET {
        return Err(Error::Protocol(format!(
            "validation failed: total team price {} exceeds max budget {}",
            total_price, MAX_BUDGET
        )));
    }

    let mut tx = pool.begin().await?;

    let team_res = sqlx::query!(
        "INSERT INTO Teams (user_id, round, captain_id) VALUES (?, ?, ?) AS new
         ON DUPLICATE KEY UPDATE captain_id = new.captain_id",
        user_id,
        &round,
        captain_id
    )
    .execute(&mut *tx)
    .await?;

    let team_id: i32 = if team_res.last_insert_id() > 0 {
        team_res.last_insert_id() as i32
    } else {
        sqlx::query!("SELECT id FROM Teams WHERE user_id = ? AND round = ?", user_id, &round)
            .fetch_one(&mut *tx)
            .await?
            .id
    };

    sqlx::query!(
        "DELETE FROM TeamPlayers WHERE team_id = ?",
        team_id
    )
    .execute(&mut *tx)
    .await?;

    let mut final_res: Option<MySqlQueryResult> = Some(team_res);

    for pid in &player_ids {
        let res = sqlx::query!(
            "INSERT INTO TeamPlayers (team_id, player_id) VALUES (?, ?)",
            team_id,
            pid
        )
        .execute(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO PlayerScores (player_id, round, score, total_score, match_count) VALUES (?, ?, 0, 0, 0) ON DUPLICATE KEY UPDATE player_id = player_id",
            pid,
            &round
        )
        .execute(&mut *tx)
        .await?;

        final_res = Some(res);
    }

    tx.commit().await?;

    final_res.ok_or_else(|| Error::Protocol("no DB operation performed".to_string()))
}