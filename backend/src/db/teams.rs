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

    // Get players with prices for this round
    sqlx::query_as!(
        PlayerWithPrice,
        r#"
        SELECT 
            p.id as `id!: i32`,
            p.username,
            p.avatar_url,
            p.country,
            p.`rank` as `rank!: i32`,
            p.eliminated as `eliminated!: i8`,
            COALESCE(pp.price, 0) as `price!: i32`
        FROM TeamPlayers tp
        JOIN Players p ON tp.player_id = p.id
        LEFT JOIN PlayerPrices pp ON p.id = pp.player_id AND pp.round = ?
        WHERE tp.team_id = ?
        "#,
        round,
        team_id
    )
    .fetch_all(pool)
    .await
}
