use crate::db::players::create_team_from_players;
use crate::db::teams::get_players_by_team_id;
use crate::middleware::auth::same_id_middleware;
use crate::util::round::compute_round;
use actix_web::{get, middleware::from_fn, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("/{id}")]
async fn teams_get_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let team_id = path.into_inner();

    let players = get_players_by_team_id(pool, team_id).await;
    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(e) => {
            log::error!("Error retrieving team {}: {:?}", team_id, e);
            HttpResponse::InternalServerError().body("Error retreiving team")
        }
    }
}

#[derive(Deserialize)]
struct UpdateTeamRequest {
    player_ids: Vec<i32>,
    captain_id: Option<i32>,
}

#[post("/{user_id}/{round}", wrap = "from_fn(same_id_middleware)")]
async fn teams_update_players(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
    body: web::Json<UpdateTeamRequest>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (user_id, round) = path.into_inner();
    let player_ids = body.player_ids.clone();
    let captain_id = body.captain_id;

    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&round.as_str()) {
        return HttpResponse::BadRequest().body("Invalid round");
    }

    let current_round = compute_round();
    if round != current_round.as_str() {
        return HttpResponse::BadRequest().body(format!(
            "Can only update team for current round ({})",
            current_round.as_str()
        ));
    }

    if let Some(captain) = captain_id {
        if !player_ids.contains(&captain) {
            return HttpResponse::BadRequest().body(
                "Captain must be one of the 8 players in the team"
            );
        }
    }

    let res = create_team_from_players(pool, user_id, player_ids, round, captain_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(sqlx::Error::Protocol(msg)) => {
            log::error!("Team update validation error: {}", msg);
            HttpResponse::BadRequest().body(msg)
        }
        Err(e) => {
            log::error!("Team update database error: {:?}", e);
            HttpResponse::InternalServerError().body("Error updating team")
        }
    }
}

pub fn teams_controller() -> actix_web::Scope {
    web::scope("/teams")
        .service(teams_get_by_id)
        .service(teams_update_players)
}
