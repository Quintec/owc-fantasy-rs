use crate::db::teams::get_players_by_team_id;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("/{id}")]
async fn teams_get_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let team_id = path.into_inner();

    let players = get_players_by_team_id(pool, team_id).await;
    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(_) => HttpResponse::InternalServerError().body("Error retreiving team"),
    }
}

pub fn teams_controller() -> actix_web::Scope {
    web::scope("/teams").service(teams_get_by_id)
}
