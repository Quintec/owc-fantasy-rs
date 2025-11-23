use crate::db::{
    models::Player,
    players::{bulk_create_players, get_all_players, update_player_price},
};
use crate::middleware::auth::admin_middleware;
use crate::state::AppState;
use actix_web::{middleware::from_fn, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::env;
use std::collections::HashMap;
use rosu_v2::{Osu, prelude::*};

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

#[post("/import_from_participants", wrap = "from_fn(admin_middleware)")]
pub async fn players_import_from_participants(
    data: web::Data<AppState>,
    req: web::Json<ImportParticipantsRequest>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    
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
    
    let player_ids = extract_player_ids(&req.participants_text);
    
    eprintln!("Extracted {} player IDs: {:?}", player_ids.len(), player_ids);
    
    if player_ids.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No player IDs found in the provided text"
        }));
    }
    
    let osu = match Osu::builder()
        .client_id(client_id.parse().unwrap_or(0))
        .client_secret(client_secret)
        .build()
        .await
    {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create osu! client: {}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to create osu! client: {}", e)
            }));
        }
    };
    
    let mut players = Vec::new();
    let mut errors = Vec::new();
    
    for player_id in &player_ids {
        match osu.user(*player_id as u32).mode(GameMode::Osu).await {
            Ok(user) => {
                let rank = user.statistics
                    .as_ref()
                    .and_then(|s| s.global_rank)
                    .unwrap_or(0) as i32;
                
                let player = Player {
                    id: user.user_id as i32,
                    username: user.username.to_string(),
                    avatar_url: user.avatar_url.to_string(),
                    country: user.country_code.to_string(),
                    rank,
                    eliminated: 0,
                };
                
                eprintln!("✓ Fetched {} (id: {}, rank: {})", player.username, player.id, player.rank);
                players.push(player);
            }
            Err(e) => {
                eprintln!("✗ Failed to fetch player {}: {}", player_id, e);
                errors.push(format!("Player {}: {}", player_id, e));
            }
        }
    }
    
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

#[derive(Debug, Deserialize)]
pub struct ImportPScoresRequest {
    pub pscore_text: String,
    pub round: String,
}

#[derive(Debug, Serialize)]
pub struct ImportPScoresResponse {
    pub updated_count: usize,
    pub skipped: Vec<String>,
    pub errors: Vec<String>,
}

/// Parse pScore text in format: "identifier\tpscore" (one per line)
/// Identifier can be either user_id (integer) or username (string)
/// Returns HashMap of identifier -> pscore
pub fn parse_pscores_flexible(text: &str) -> HashMap<String, f64> {
    let mut scores = HashMap::new();
    
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        
        if line.contains('\t') {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                let identifier = parts[0].trim().to_string();
                if let Ok(pscore) = parts[parts.len() - 1].trim().parse::<f64>() {
                    scores.insert(identifier, pscore);
                }
            }
        } else {
            if let Some(last_space_idx) = line.rfind(char::is_whitespace) {
                let identifier = line[..last_space_idx].trim().to_string();
                let score_str = line[last_space_idx..].trim();
                
                if let Ok(pscore) = score_str.parse::<f64>() {
                    scores.insert(identifier, pscore);
                }
            }
        }
    }
    
    scores
}

/// pScore-based pricing: 6M-27M range with exponential curve (steepness=1.7)
/// Normalized against max pScore in dataset to ensure consistent pricing across rounds
fn pscore_to_price(pscore: f64) -> i32 {
    const TOTAL_BUDGET: f64 = 100_000_000.0;
    const MIN_PRICE: i32 = 4_000_000;
    const MAX_PRICE: i32 = 35_000_000;
    const MEAN_PSCORE: f64 = 1.0;
    const STDDEV_PSCORE: f64 = 0.5;
    
    let z_score = (pscore - MEAN_PSCORE) / STDDEV_PSCORE;
    const BASE_PRICE: f64 = 7_500_000.0;
    const CURVE_STEEPNESS: f64 = 0.45;
    
    let price = BASE_PRICE * (CURVE_STEEPNESS * z_score).exp();
    let rounded_price = ((price / 1000.0).round() * 1000.0) as i32;
    rounded_price.clamp(MIN_PRICE, MAX_PRICE)
}

/// pScore-based pricing: 6M-27M range with exponential curve (steepness=1.7)
/// Normalized against max pScore in dataset to ensure consistent pricing across rounds
fn pscore_to_price_relative(pscore: f64, max_pscore: f64) -> i32 {
    const MIN_PRICE: i32 = 6_000_000;
    const MAX_PRICE: i32 = 27_000_000;
    const MIN_VIABLE_PSCORE: f64 = 0.3;
    
    let normalized = if max_pscore <= MIN_VIABLE_PSCORE {
        0.5
    } else {
        ((pscore - MIN_VIABLE_PSCORE) / (max_pscore - MIN_VIABLE_PSCORE))
            .max(0.0)
            .min(1.0)
    };
    
    const CURVE_STEEPNESS: f64 = 1.7;
    let price_range = (MAX_PRICE - MIN_PRICE) as f64;
    let price = MIN_PRICE as f64 + price_range * normalized.powf(CURVE_STEEPNESS);
    
    let rounded_price = ((price / 1000.0).round() * 1000.0) as i32;
    rounded_price.clamp(MIN_PRICE, MAX_PRICE)
}

/// Rank-based pricing fallback: 6M-15M logarithmic curve
/// Used when no pScore available (higher rank = higher price)
fn rank_to_price(rank: i32) -> i32 {
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

/// Import pScores and calculate prices for players
#[post("/import_pscores", wrap = "from_fn(admin_middleware)")]
pub async fn players_import_pscores(
    data: web::Data<AppState>,
    req: web::Json<ImportPScoresRequest>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    
    // Validate round
    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&req.round.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid round"
        }));
    }
    
    let pscores = parse_pscores_flexible(&req.pscore_text);
    
    if pscores.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No valid pScore data found. Expected format: 'identifier\\tpscore' (identifier can be user_id or username)"
        }));
    }
    
    let all_players = match get_all_players(pool).await {
        Ok(players) => players,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch players: {}", e)
            }));
        }
    };
    
    let active_players: Vec<_> = all_players.iter()
        .filter(|p| p.eliminated == 0)
        .collect();
    
    let max_pscore = pscores.values()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max);
    
    eprintln!("Maximum pScore in dataset: {:.3}", max_pscore);
    eprintln!("Attempting to match {} pScore entries against {} active players", 
              pscores.len(), active_players.len());
    
    let mut updated_count = 0;
    let mut skipped_eliminated = Vec::new();
    let mut skipped_not_found = Vec::new();
    let mut errors = Vec::new();
    
    for (identifier, pscore) in &pscores {
        let player = if let Ok(user_id) = identifier.parse::<i32>() {
            active_players.iter().find(|p| p.id == user_id)
        } else {
            active_players.iter().find(|p| p.username.eq_ignore_ascii_case(identifier))
        };
        
        match player {
            Some(player) => {
                let price = pscore_to_price_relative(*pscore, max_pscore);
                
                match update_player_price(pool, player.id, req.round.clone(), price).await {
                    Ok(_) => {
                        updated_count += 1;
                        eprintln!("✓ Updated {} (id: {}) with pScore {:.2} → ${}", 
                                  player.username, player.id, pscore, price);
                    }
                    Err(e) => {
                        errors.push(format!("Failed to update player '{}': {}", identifier, e));
                    }
                }
            }
            None => {
                let is_eliminated = if let Ok(user_id) = identifier.parse::<i32>() {
                    all_players.iter().any(|p| p.id == user_id && p.eliminated != 0)
                } else {
                    all_players.iter().any(|p| p.username.eq_ignore_ascii_case(identifier) && p.eliminated != 0)
                };
                
                if is_eliminated {
                    eprintln!("⊘ Player '{}' is eliminated, skipping", identifier);
                    skipped_eliminated.push(identifier.clone());
                } else {
                    eprintln!("✗ Player '{}' not found in database", identifier);
                    skipped_not_found.push(identifier.clone());
                }
            }
        }
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "updated_count": updated_count,
        "skipped_eliminated": skipped_eliminated,
        "skipped_not_found": skipped_not_found,
        "errors": errors,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RefreshPlayersRequest {
    /// Optional list of player IDs to refresh. If empty/null, refreshes all players.
    pub player_ids: Option<Vec<i32>>,
}

/// Set default prices for all players based on their rank
/// This is useful for initializing prices or refreshing players without pScores
#[derive(Debug, Deserialize)]
pub struct SetDefaultPricesRequest {
    pub round: String,
}

#[post("/set_default_prices", wrap = "from_fn(admin_middleware)")]
pub async fn players_set_default_prices(
    data: web::Data<AppState>,
    req: web::Json<SetDefaultPricesRequest>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    
    // Validate round
    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&req.round.as_str()) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Invalid round"
        }));
    }
    
    let all_players = match get_all_players(pool).await {
        Ok(players) => players,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch players: {}", e)
            }));
        }
    };
    
    let mut updated_count = 0;
    let mut errors = Vec::new();
    
    for player in all_players {
        let default_price = rank_to_price(player.rank);
        
        match update_player_price(pool, player.id, req.round.clone(), default_price).await {
            Ok(_) => {
                updated_count += 1;
                eprintln!("✓ Set default price for {} (rank: {}) → ${}", 
                          player.username, player.rank, default_price);
            }
            Err(e) => {
                errors.push(format!("Failed to update player '{}': {}", player.username, e));
            }
        }
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "updated_count": updated_count,
        "errors": errors,
    }))
}
