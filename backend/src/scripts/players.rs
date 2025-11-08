use crate::db::{
    models::Player,
    players::bulk_create_players,
};
use crate::middleware::auth::admin_middleware;
use crate::state::AppState;
use actix_web::{middleware::from_fn, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ImportParticipantsRequest {
    pub participants_text: String,
}

#[derive(Debug, Serialize)]
pub struct ImportParticipantsResponse {
    pub players: Vec<Player>,
    pub count: usize,
    pub errors: Vec<String>,
}

// Extract player IDs from markdown text containing osu! profile links
pub fn extract_player_ids(text: &str) -> Vec<i32> {
    let re = regex::Regex::new(r"osu\.ppy\.sh/users/(\d+)").unwrap();
    let mut ids = std::collections::HashSet::new();
    
    for cap in re.captures_iter(text) {
        if let Some(id_str) = cap.get(1) {
            if let Ok(id) = id_str.as_str().parse::<i32>() {
                ids.insert(id);
            }
        }
    }
    
    ids.into_iter().collect()
}

// Import players from participants markdown text
// This endpoint extracts IDs, fetches data via OAuth2, and bulk inserts into DB
#[post("/import_from_participants", wrap = "from_fn(admin_middleware)")]
pub async fn players_import_from_participants(
    data: web::Data<AppState>,
    req: web::Json<ImportParticipantsRequest>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    
    // Get OAuth2 credentials from env
    let token_url = match env::var("OAUTH_TOKEN_URL") {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "OAUTH_TOKEN_URL not configured"
        })),
    };
    let client_id = match env::var("OAUTH_CLIENT_ID") {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "OAUTH_CLIENT_ID not configured"
        })),
    };
    let client_secret = match env::var("OAUTH_CLIENT_SECRET") {
        Ok(v) => v,
        Err(_) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "OAUTH_CLIENT_SECRET not configured"
        })),
    };
    let userinfo_url = match env::var("OAUTH_USERINFO_URL") {
        Ok(v) => v,
        Err(_) => {
            // Default to osu! API
            "https://osu.ppy.sh/api/v2/users/{id}".to_string()
        }
    };
    
    // Extract player IDs from markdown
    let player_ids = extract_player_ids(&req.participants_text);
    
    eprintln!("Extracted {} player IDs: {:?}", player_ids.len(), player_ids);
    
    if player_ids.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No player IDs found in the provided text"
        }));
    }
    
    // Get OAuth2 access token using client credentials
    let client = reqwest::Client::new();
    eprintln!("Requesting OAuth token from: {}", token_url);
    let token_response = match client
        .post(&token_url)
        .form(&[
            ("grant_type", "client_credentials"),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("scope", "public"),
        ])
        .send()
        .await
    {
        Ok(resp) => {
            let status = resp.status();
            eprintln!("OAuth token response status: {}", status);
            if !status.is_success() {
                let error_text = resp.text().await.unwrap_or_else(|_| "Unable to read error".to_string());
                eprintln!("OAuth error response: {}", error_text);
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("OAuth token request failed with status {}: {}", status, error_text)
                }));
            }
            resp
        },
        Err(e) => {
            eprintln!("Failed to get OAuth token: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to get OAuth token: {}", e)
            }));
        }
    };
    
    let token_json: serde_json::Value = match token_response.json().await {
        Ok(j) => j,
        Err(e) => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": format!("Invalid token response: {}", e)
        })),
    };
    
    let access_token = match token_json.get("access_token").and_then(|v| v.as_str()) {
        Some(t) => t.to_string(),
        None => return HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "No access_token in OAuth response"
        })),
    };
    
    // Fetch player data from osu! API
    let mut players = Vec::new();
    let mut errors = Vec::new();
    
    for player_id in &player_ids {
        let url = userinfo_url.replace("{id}", &player_id.to_string());
        
        match client
            .get(&url)
            .bearer_auth(&access_token)
            .send()
            .await
        {
            Ok(resp) => {
                if !resp.status().is_success() {
                    errors.push(format!("Player {}: HTTP {}", player_id, resp.status()));
                    continue;
                }
                
                match resp.json::<serde_json::Value>().await {
                    Ok(user_info) => {
                        let empty_stats = serde_json::json!({});
                        let stats = user_info.get("statistics").unwrap_or(&empty_stats);
                        
                        let player = Player {
                            id: user_info.get("id").and_then(|v| v.as_i64()).unwrap_or(*player_id as i64) as i32,
                            username: user_info
                                .get("username")
                                .and_then(|v| v.as_str())
                                .unwrap_or(&format!("player_{}", player_id))
                                .to_string(),
                            avatar_url: user_info
                                .get("avatar_url")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            country: user_info
                                .get("country_code")
                                .and_then(|v| v.as_str())
                                .unwrap_or("XX")
                                .to_string(),
                            rank: stats
                                .get("global_rank")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32,
                            eliminated: 0,
                        };
                        
                        players.push(player);
                    }
                    Err(e) => {
                        errors.push(format!("Player {}: parse error - {}", player_id, e));
                    }
                }
            }
            Err(e) => {
                errors.push(format!("Player {}: request failed - {}", player_id, e));
            }
        }
    }
    
    // Bulk insert into database
    if !players.is_empty() {
        match bulk_create_players(pool, players.clone()).await {
            Ok(_) => {
                HttpResponse::Ok().json(ImportParticipantsResponse {
                    count: players.len(),
                    players,
                    errors,
                })
            }
            Err(e) => {
                eprintln!("Database insert failed: {}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": format!("Database insert failed: {}", e),
                    "fetched_players": players.len(),
                }))
            }
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No players could be fetched",
            "errors": errors,
        }))
    }
}
