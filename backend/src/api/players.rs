use crate::db::models::Player;
use crate::db::players::{get_all_players, get_remaining_players};
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("")]
async fn players_get(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let players: Vec<Player> = get_all_players(pool).await;

    HttpResponse::Ok().json(players)
}

#[get("/remaining")]
async fn players_get_remaining(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let players: Vec<Player> = get_remaining_players(pool).await;

    HttpResponse::Ok().json(players)
}

pub fn players_controller() -> actix_web::Scope {
    web::scope("/players")
        .service(players_get)
        .service(players_get_remaining)
}
