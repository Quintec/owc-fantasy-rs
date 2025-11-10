use crate::middleware::auth::admin_middleware;
use crate::util::round::Round;
use actix_web::{post, middleware::from_fn, web, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;

use crate::db::players;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
struct ParseMultiplayerRequest {
    links: Vec<String>,
    round: Round,
}

#[post("/parse-multiplayer", wrap = "from_fn(admin_middleware)")]
async fn parse_multiplayer(
    data: web::Data<AppState>,
    body: web::Json<ParseMultiplayerRequest>,
    _session: Session,
) -> impl Responder {
    let mut failed_links: Vec<String> = Vec::new();
    
    // Map to accumulate scores: player_id -> (total_score, match_count)
    let mut player_scores: std::collections::HashMap<i32, (i32, i32)> = std::collections::HashMap::new();

    for link in &body.links {
        let trimmed = link.trim();
        
        // Skip empty lines or invalid entries
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("link") {
            continue;
        }
        
        // Convert match ID to full URL if needed
        let full_link = if trimmed.parse::<u32>().is_ok() {
            format!("https://osu.ppy.sh/community/matches/{}", trimmed)
        } else {
            trimmed.to_string()
        };
        
        println!("Parsing multiplayer link: {}", full_link);

        match crate::util::match_costs::matchcosts(&full_link, 0).await {
            Ok(match_result) => {
                println!("Match result: {:?}", match_result);

                match match_result {
                    crate::util::match_costs::MatchResult::TeamVS { blue, red, mvp_avatar_url: _, maps } => {
                        let results = crate::util::score_calc::calculate_teamvs_scores(&blue, &red, &maps);
                        println!("Scoring results for match {}:", link);
                        
                        for r in results.iter() {
                            println!("User {} => {} points", r.user_id, r.points);
                            
                            // Accumulate scores
                            let entry = player_scores.entry(r.user_id as i32).or_insert((0, 0));
                            entry.0 += r.points as i32; // Add to total score
                            entry.1 += 1; // Increment match count
                        }
                    }
                    crate::util::match_costs::MatchResult::HeadToHead { .. } => {
                        println!("Head-to-head matches are not supported for score calculation.");
                        failed_links.push(full_link.clone());
                    }
                    crate::util::match_costs::MatchResult::NoGames { description } => {
                        println!("No games: {}", description);
                        failed_links.push(full_link.clone());
                    }
                }
            }
            Err(_) => {
                println!("Failed to parse match from link: {}", full_link);
                failed_links.push(full_link.clone());
            }
        }
    }
    
    // Now save all accumulated scores to database
    let mut save_errors = Vec::new();
    for (player_id, (total_score, match_count)) in player_scores.iter() {
        match players::update_player_round_score(
            &data.pool,
            *player_id,
            &body.round,
            *total_score,
            *match_count,
        )
        .await
        {
            Ok(_) => {
                let avg_score = *total_score as f32 / *match_count as f32;
                println!("Saved score for player {}: total={}, matches={}, avg={:.2}", 
                         player_id, total_score, match_count, avg_score);
            }
            Err(e) => {
                println!("Failed to save score for player {}: {}", player_id, e);
                save_errors.push(format!("Player {}: {}", player_id, e));
            }
        }
    }

    if failed_links.is_empty() && save_errors.is_empty() {
        HttpResponse::Ok().body(format!(
            "Successfully processed {} matches and updated {} players.", 
            body.links.len() - failed_links.len(), 
            player_scores.len()
        ))
    } else {
        let mut error_msg = String::new();
        if !failed_links.is_empty() {
            error_msg.push_str(&format!("Failed to process links: {:?}\n", failed_links));
        }
        if !save_errors.is_empty() {
            error_msg.push_str(&format!("Failed to save scores: {:?}", save_errors));
        }
        HttpResponse::BadRequest().body(error_msg)
    }
}

pub fn admin_controller() -> actix_web::Scope {
    web::scope("/admin").service(parse_multiplayer)
}
