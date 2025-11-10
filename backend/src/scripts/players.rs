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

// ============= pScore Import =============

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
        
        // Split by tab or multiple spaces
        let parts: Vec<&str> = if line.contains('\t') {
            line.split('\t').collect()
        } else {
            line.split_whitespace().collect()
        };
        
        if parts.len() >= 2 {
            // First part is identifier (user_id or username)
            let identifier = parts[0].trim().to_string();
            // Last part should be pscore (float)
            if let Ok(pscore) = parts[parts.len() - 1].trim().parse::<f64>() {
                scores.insert(identifier, pscore);
            }
        }
    }
    
    scores
}

/// Convert pscore to price using normalized convex curve
/// This creates exponentially higher prices for elite players, forcing trade-offs
/// 
/// PSCORE FORMULA ANALYSIS:
/// ════════════════════════════════════════════════════════════════════
/// pScore = [Σ(S/M) / n] × √(n / ΣN/m)
/// where:
/// - S = player score on a map
/// - M = median score on that map
/// - n = maps played by player
/// - N = mean maps per player in a match
/// - m = matches played by player
///
/// What this means:
/// • pScore rewards BOTH performance (S/M ratio) AND participation (√n factor)
/// • Elite (2.2): Consistently 2.2x median + high participation
/// • Good (1.6): Consistently 1.6x median + decent participation  
/// • Avg (1.0): At median + average participation
/// 
/// STRATEGIC BALANCE ANALYSIS:
/// ════════════════════════════════════════════════════════════════════
/// Fantasy scoring system (from score_calc.rs):
/// - Participation: 3-5 pts (30-65% or >65% of maps)
/// - Performance: 1 pt per highest score on map, 1 pt per above-avg score
/// - Match cost: 1-5 pts (2nd on team, 1st on team, 1st in match)
/// - Team win: +2 pts
/// 
/// KEY STRATEGIC INSIGHT:
/// High pScore ≠ guaranteed high fantasy points!
/// • Elite (2.2) players might skip maps → lose participation pts
/// • Good (1.6) players with full participation can outscore elites
/// • Match cost rankings depend on team composition, not just skill
/// • Team win bonus (2pts) adds randomness
///
/// PRICING STRATEGY:
/// ════════════════════════════════════════════════════════════════════
/// Target: elite=30M, good=15M, avg=10M
/// 
/// This creates multiple viable strategies:
/// 1. "Balanced" (2 elite + 2 good + 4 avg = 60+30+40 = 130M) ✗ Over
/// 2. "Value Play" (1 elite + 4 good + 3 avg = 30+60+30 = 120M) ✗ Over  
/// 3. "All-Around" (7 good + 1 avg = 105+10 = 115M) ✗ Over
/// 4. "Safe Floor" (1 elite + 3 good + 4 avg = 30+45+40 = 115M) ✗ Over
/// 5. "Depth" (5 good + 3 avg = 75+30 = 105M) ✗ Close but over
/// 6. "Participation" (1 elite + 2 good + 5 avg = 30+30+50 = 110M) ✗ Over
fn pscore_to_price(pscore: f64) -> i32 {
    // PScore is a normalized performance metric centered around 1.0
    // Uses z-score based pricing with exponential curve
    
    const TOTAL_BUDGET: f64 = 100_000_000.0;
    const MIN_PRICE: i32 = 4_000_000; // 4M floor
    const MAX_PRICE: i32 = 35_000_000; // 35M ceiling
    
    // Population parameters (empirically derived from tournament data)
    const MEAN_PSCORE: f64 = 1.0;
    const STDDEV_PSCORE: f64 = 0.5; // Adjusted for realistic variance
    
    // Calculate z-score (standard deviations from mean)
    let z_score = (pscore - MEAN_PSCORE) / STDDEV_PSCORE;
    
    // Base price for average player (z=0)
    const BASE_PRICE: f64 = 7_500_000.0; // 7.5M for average (cheaper)
    
    // Exponential curve: price = base * e^(k*z)
    // k = 0.45 gives steeper curve for elite players:
    // - pscore 0.5 (z=-1): ~5M (cheaper)
    // - pscore 1.0 (z=0): 7.5M (cheaper)
    // - pscore 1.5 (z=1): ~11.7M (cheaper)
    // - pscore 2.0 (z=2): ~18.3M (good)
    // - pscore 2.5 (z=3): ~28.6M (elite, more expensive)
    // Allows: 1 elite (28M) + 2 good (36M) + 5 average (37.5M) = 101.5M (close)
    const CURVE_STEEPNESS: f64 = 0.45;
    
    let price = BASE_PRICE * (CURVE_STEEPNESS * z_score).exp();
    
    // Round to nearest thousand and enforce bounds
    let rounded_price = ((price / 1000.0).round() * 1000.0) as i32;
    rounded_price.clamp(MIN_PRICE, MAX_PRICE)
}

/// Convert rank to a default price (used for players without pScores)
/// Uses logarithmic curve: higher rank (lower number) = higher price
fn rank_to_price(rank: i32) -> i32 {
    const MIN_PRICE: i32 = 4_000_000; // 4M floor
    const MAX_PRICE: i32 = 15_000_000; // 15M ceiling for rank-based
    
    if rank <= 0 {
        return MIN_PRICE;
    }
    
    // Logarithmic curve: price = max - k * log(rank)
    // Top 100: ~15M, Top 1000: ~12M, Top 10000: ~8M, Top 100000: ~5M
    const K: f64 = 2_500_000.0; // scaling factor
    let log_price = MAX_PRICE as f64 - K * (rank as f64).ln();
    
    // Clamp between min and max, round to nearest thousand
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
    
    // Parse pScores from text (accepts both user_id and username)
    let pscores = parse_pscores_flexible(&req.pscore_text);
    
    if pscores.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "No valid pScore data found. Expected format: 'identifier\\tpscore' (identifier can be user_id or username)"
        }));
    }
    
    // Get all players from database
    let all_players = match get_all_players(pool).await {
        Ok(players) => players,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to fetch players: {}", e)
            }));
        }
    };
    
    // Filter out eliminated players (they won't have pScores for future rounds)
    let active_players: Vec<_> = all_players.iter()
        .filter(|p| p.eliminated == 0)
        .collect();
    
    // Match players and update prices
    let mut updated_count = 0;
    let mut skipped_eliminated = Vec::new();
    let mut skipped_not_found = Vec::new();
    let mut errors = Vec::new();
    
    eprintln!("Attempting to match {} pScore entries against {} active players", 
              pscores.len(), active_players.len());
    
    for (identifier, pscore) in &pscores {
        // Try to match by user_id first (if identifier is numeric), then by username
        let player = if let Ok(user_id) = identifier.parse::<i32>() {
            active_players.iter().find(|p| p.id == user_id)
        } else {
            // Case-insensitive username match
            active_players.iter().find(|p| p.username.eq_ignore_ascii_case(identifier))
        };
        
        match player {
            Some(player) => {
                let price = pscore_to_price(*pscore);
                
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
                // Check if player exists but is eliminated (try both ID and username)
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
    
    // Get all players
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
