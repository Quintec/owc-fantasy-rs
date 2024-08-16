use crate::db::models::{Player, Team};

use sqlx::MySqlPool;

pub async fn get_teams_by_user_id(pool: &MySqlPool, user_id: i32) -> Vec<Team> {
    sqlx::query_as!(
        Team,
        "SELECT id, user_id, round FROM Teams WHERE user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await
    .expect("Error fetching teams")
}

pub async fn get_round_team_by_user_id(pool: &MySqlPool, user_id: i32, round: String) -> Team {
    sqlx::query_as!(
        Team,
        "SELECT id, user_id, round FROM Teams WHERE user_id = ? AND round = ?",
        user_id,
        round
    )
    .fetch_one(pool)
    .await
    .expect("Error fetching team")
}

pub async fn add_player_to_team(pool: &MySqlPool, team_id: i32, player_id: i32) {
    sqlx::query!(
        "INSERT INTO TeamPlayers (team_id, player_id) VALUES (?, ?)",
        team_id,
        player_id
    )
    .execute(pool)
    .await
    .expect("Error adding player to team");
}

pub async fn remove_player_from_team(pool: &MySqlPool, team_id: i32, player_id: i32) {
    sqlx::query!(
        "DELETE FROM TeamPlayers WHERE team_id = ? AND player_id = ?",
        team_id,
        player_id
    )
    .execute(pool)
    .await
    .expect("Error removing player from team");
}

pub async fn get_players_by_team_id(pool: &MySqlPool, team_id: i32) -> Vec<Player> {
    // get player ids from TeamPlayers
    let player_ids: Vec<i32> = sqlx::query!(
        "SELECT player_id FROM TeamPlayers WHERE team_id = ?",
        team_id
    )
    .map(|record| record.player_id)
    .fetch_all(pool)
    .await
    .expect("Error fetching player ids");

    let player_ids_str = player_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    sqlx::query_as!(
        Player,
        "SELECT id, username, avatar_url, country, rank FROM Players WHERE id IN (?)",
        player_ids_str
    )
    .fetch_all(pool)
    .await
    .expect("Error fetching players")
}
