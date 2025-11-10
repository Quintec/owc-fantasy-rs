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
                        
                        let mut all_success = true;
                        for r in results.iter() {
                            println!("User {} => {} points", r.user_id, r.points);

                            match players::update_player_round_score(
                                &data.pool,
                                r.user_id as i32,
                                &body.round,
                                r.points as i32,
                            )
                            .await
                            {
                                Ok(_) => {
                                    println!("Saved score for user {}", r.user_id);
                                }
                                Err(e) => {
                                    println!("Failed to save score for user {}: {}", r.user_id, e);
                                    all_success = false;
                                    break;
                                }
                            }
                        }

                        if !all_success {
                            failed_links.push(full_link.clone());
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

    if failed_links.is_empty() {
        HttpResponse::Ok().body("Multiplayer links processed successfully.")
    } else {
        HttpResponse::BadRequest().body(format!("Failed to process the following links: {:?}", failed_links))
    }
}

pub fn admin_controller() -> actix_web::Scope {
    web::scope("/admin").service(parse_multiplayer)
}
