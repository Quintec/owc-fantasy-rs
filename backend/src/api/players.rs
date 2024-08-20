use crate::db::models::Player;
use crate::db::players::{get_all_players, get_player_by_id, get_remaining_players};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("")]
async fn players_get(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let players = get_all_players(pool).await;

    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching players"),
    }
}

#[get("/remaining")]
async fn players_get_remaining(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let players = get_remaining_players(pool).await;

    match players {
        Ok(players) => HttpResponse::Ok().json(players),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching remaining players"),
    }
}

#[get("/{id}")]
async fn players_get_by_id(data: web::Data<AppState>, id: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let player = get_player_by_id(pool, id.into_inner()).await;
    match player {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::NotFound().body("Player not found"),
    }
}

pub fn players_controller() -> actix_web::Scope {
    web::scope("/players")
        .service(players_get)
        .service(players_get_remaining)
}
