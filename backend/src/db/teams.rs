use crate::db::models::{PlayerWithPrice, Team};

use sqlx::{Error, MySqlPool};

pub async fn get_teams_by_user_id(pool: &MySqlPool, user_id: i32) -> Result<Vec<Team>, Error> {
    sqlx::query_as!(
        Team,
        "SELECT id, user_id, round, captain_id FROM Teams WHERE user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_round_team_by_user_id(
    pool: &MySqlPool,
    user_id: i32,
    round: String,
) -> Result<Team, Error> {
    sqlx::query_as!(
        Team,
        "SELECT id, user_id, round, captain_id FROM Teams WHERE user_id = ? AND round = ?",
        user_id,
        round
    )
    .fetch_one(pool)
    .await
}

pub async fn get_players_by_team_id(pool: &MySqlPool, team_id: i32) -> Result<Vec<PlayerWithPrice>, Error> {
    // First, get the round for this team
    let team = sqlx::query!("SELECT round FROM Teams WHERE id = ?", team_id)
        .fetch_one(pool)
        .await?;
    
    let round = team.round;

    // Determine previous round for fallback pricing
    let previous_round = get_previous_round(&round);

    // Get players with current and previous round prices
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
        FROM TeamPlayers tp
        JOIN Players p ON tp.player_id = p.id
        LEFT JOIN PlayerPrices pp_current ON p.id = pp_current.player_id AND pp_current.round = ?
        LEFT JOIN PlayerPrices pp_prev ON p.id = pp_prev.player_id AND pp_prev.round = ?
        WHERE tp.team_id = ?
        "#,
        round,
        previous_round,
        team_id
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

/// Rank-based price calculation for fallback (matches the one in players.rs)
fn rank_to_price_fallback(rank: i32) -> i32 {
    const MIN_PRICE: i32 = 6_000_000; // 6M floor (raised to match pScore pricing)
    const MAX_PRICE: i32 = 15_000_000; // 15M ceiling for rank-based
    
    if rank <= 0 {
        return MIN_PRICE;
    }
    
    // Logarithmic curve: price = max - k * log(rank)
    const K: f64 = 2_500_000.0; // scaling factor
    let log_price = MAX_PRICE as f64 - K * (rank as f64).ln();
    
    // Clamp between min and max, round to nearest thousand
    let price = log_price.max(MIN_PRICE as f64).min(MAX_PRICE as f64);
    ((price / 1000.0).round() * 1000.0) as i32
}

/// Helper function to determine the previous round
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
