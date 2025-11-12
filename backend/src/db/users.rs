use crate::db::models::{User, UserWithScore};
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

/// Get all users with their total scores across all rounds
/// Score = sum of all their team players' scores (with captain 2x bonus) across all rounds
pub async fn get_leaderboard(pool: &MySqlPool) -> Result<Vec<UserWithScore>, Error> {
    // Calculate scores in a subquery to avoid aggregation issues
    // Use COALESCE on ps.score to handle NULL scores (players without scores yet)
    let users = sqlx::query_as!(
        UserWithScore,
        r#"
        SELECT 
            u.id,
            u.username,
            u.avatar_url,
            CAST(COALESCE(scores.total_score, 0) AS SIGNED) as `score: i64`
        FROM Users u
        LEFT JOIN (
            SELECT 
                t.user_id,
                SUM(
                    CASE 
                        WHEN tp.player_id = t.captain_id THEN COALESCE(ps.score, 0) * 2
                        ELSE COALESCE(ps.score, 0)
                    END
                ) as total_score
            FROM Teams t
            INNER JOIN TeamPlayers tp ON t.id = tp.team_id
            LEFT JOIN PlayerScores ps ON tp.player_id = ps.player_id AND ps.round = t.round
            GROUP BY t.user_id
        ) AS scores ON u.id = scores.user_id
        ORDER BY `score: i64` DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    // DEBUG: Print what we got from the database
    for user in &users {
        eprintln!("DEBUG get_leaderboard: id={}, username={}, score={}", user.id, user.username, user.score);
    }

    Ok(users)
}

/// Debug: Get detailed breakdown of what's being counted for the leaderboard
pub async fn get_leaderboard_breakdown(pool: &MySqlPool) -> Result<Vec<serde_json::Value>, Error> {
    let breakdown = sqlx::query!(
        r#"
        SELECT 
            u.id as user_id,
            u.username,
            t.round,
            t.id as team_id,
            tp.player_id,
            p.username as player_username,
            ps.score,
            t.captain_id,
            CASE 
                WHEN tp.player_id = t.captain_id THEN ps.score * 2
                ELSE ps.score
            END as counted_score
        FROM Users u
        LEFT JOIN Teams t ON u.id = t.user_id
        LEFT JOIN TeamPlayers tp ON t.id = tp.team_id
        LEFT JOIN Players p ON tp.player_id = p.id
        LEFT JOIN PlayerScores ps ON tp.player_id = ps.player_id AND ps.round = t.round
        ORDER BY u.id, t.round, tp.player_id
        "#
    )
    .fetch_all(pool)
    .await?;

    let results: Vec<serde_json::Value> = breakdown.iter().map(|row| {
        serde_json::json!({
            "user_id": row.user_id,
            "username": &row.username,
            "round": &row.round,
            "team_id": row.team_id,
            "player_id": row.player_id,
            "player_username": &row.player_username,
            "score": row.score,
            "captain_id": row.captain_id,
            "counted_score": row.counted_score,
        })
    }).collect();

    Ok(results)
}

/// Debug: Get raw count of rows for each table for a specific user
pub async fn get_user_data_counts(pool: &MySqlPool, user_id: i32) -> Result<serde_json::Value, Error> {
    let teams_count = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM Teams WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool)
    .await?;

    let team_players_count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as count FROM TeamPlayers tp 
           JOIN Teams t ON tp.team_id = t.id 
           WHERE t.user_id = ?"#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    // Check how many user rows exist
    let user_rows = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM Users WHERE id = ?",
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    // Get all Users to see if there are duplicates
    let all_users_count = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM Users"
    )
    .fetch_one(pool)
    .await?;
    
    // Check total PlayerScores count (should not affect user with no teams)
    let total_player_scores = sqlx::query_scalar!(
        "SELECT COUNT(*) as count FROM PlayerScores"
    )
    .fetch_one(pool)
    .await?;
    
    // Get the EXACT result from the leaderboard query for this user
    let leaderboard_result = sqlx::query!(
        r#"
        SELECT 
            u.id,
            u.username,
            u.avatar_url,
            CAST(COALESCE(scores.total_score, 0) AS SIGNED) as `score: i64`
        FROM Users u
        LEFT JOIN (
            SELECT 
                t.user_id,
                SUM(
                    CASE 
                        WHEN tp.player_id = t.captain_id THEN COALESCE(ps.score, 0) * 2
                        ELSE COALESCE(ps.score, 0)
                    END
                ) as total_score
            FROM Teams t
            INNER JOIN TeamPlayers tp ON t.id = tp.team_id
            LEFT JOIN PlayerScores ps ON tp.player_id = ps.player_id AND ps.round = t.round
            GROUP BY t.user_id
        ) AS scores ON u.id = scores.user_id
        WHERE u.id = ?
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    // Check if user_id exists in Players table (shouldn't matter but let's check)
    let exists_as_player = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM Players WHERE id = ?",
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    // Try query WITHOUT GROUP BY to see what rows are being generated
    let raw_rows = sqlx::query!(
        r#"
        SELECT 
            u.id as user_id,
            t.id as team_id,
            tp.player_id,
            ps.score,
            t.captain_id
        FROM Users u
        LEFT JOIN Teams t ON u.id = t.user_id
        LEFT JOIN TeamPlayers tp ON t.id = tp.team_id
        LEFT JOIN PlayerScores ps ON tp.player_id = ps.player_id AND ps.round = t.round
        WHERE u.id = ?
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(serde_json::json!({
        "user_id": user_id,
        "user_rows_for_this_id": user_rows,
        "total_users_in_table": all_users_count,
        "teams_count": teams_count,
        "team_players_count": team_players_count,
        "total_player_scores_in_db": total_player_scores,
        "exists_as_player": exists_as_player,
        "raw_join_rows_count": raw_rows.len(),
        "raw_join_rows": raw_rows.iter().map(|r| serde_json::json!({
            "user_id": r.user_id,
            "team_id": r.team_id,
            "player_id": r.player_id,
            "score": r.score,
            "captain_id": r.captain_id,
        })).collect::<Vec<_>>(),
        "leaderboard_query_result": {
            "id": leaderboard_result.id,
            "username": leaderboard_result.username,
            "score": leaderboard_result.score,
        },
    }))
}
