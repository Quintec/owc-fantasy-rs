use crate::middleware::auth::admin_middleware;
use actix_web::{post, middleware::from_fn, web, HttpResponse, Responder};
use actix_session::Session;
use serde::Deserialize;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
struct ParseMultiplayerRequest {
    links: Vec<String>,
    round: String,
}

#[post("/parse-multiplayer", wrap = "from_fn(admin_middleware)")]
async fn parse_multiplayer(
    _data: web::Data<AppState>,
    body: web::Json<ParseMultiplayerRequest>,
    session: Session,
) -> impl Responder {
    let mut failed_links: Vec<String> = Vec::new();
    for link in &body.links {
        println!("Parsing multiplayer link: {}", link);
        if let Ok(match_result) = crate::util::match_costs::matchcosts(link, 0).await {
            println!("Match result: {:?}", match_result);
            // TODO calc scores
        } else {
            println!("Failed to parse match from link: {}", link);
            failed_links.push(link.clone());
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
