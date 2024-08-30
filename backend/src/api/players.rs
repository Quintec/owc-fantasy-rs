use crate::db::{
    models::Player,
    players::{
        bulk_create_players, create_player, delete_player, eliminate_player, get_all_players,
        get_player_by_id, get_player_price, get_remaining_players, update_player_price,
    },
};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;
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
async fn players_get_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let player_id = path.into_inner();

    let player = get_player_by_id(pool, player_id).await;
    match player {
        Ok(player) => HttpResponse::Ok().json(player),
        Err(_) => HttpResponse::NotFound().body("Player not found"),
    }
}

#[post("/{id}/eliminate")]
async fn players_eliminate(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let player_id = path.into_inner();

    let res = eliminate_player(pool, player_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error eliminating player"),
    }
}

#[post("")]
async fn players_create(data: web::Data<AppState>, player: web::Json<Player>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let res = create_player(pool, player.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error creating player"),
    }
}

#[delete("/{id}")]
async fn players_delete(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let player_id = path.into_inner();

    let res = delete_player(pool, player_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting player"),
    }
}

#[post("/bulk_create")]
async fn players_bulk_create(
    data: web::Data<AppState>,
    players: web::Json<Vec<Player>>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let res = bulk_create_players(pool, players.into_inner()).await;
    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error creating players"),
    }
}

#[get("/{id}/price/{round}")]
async fn players_get_price(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (player_id, round) = path.into_inner();

    let price = get_player_price(pool, player_id, round).await;
    match price {
        Ok(price) => HttpResponse::Ok().json(price),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching player price"),
    }
}

#[derive(Debug, Deserialize)]
struct PlayerPrice {
    price: i32,
}

#[post("/{id}/price/{round}")]
async fn players_set_price(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
    price: web::Json<PlayerPrice>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (player_id, round) = path.into_inner();

    let res = update_player_price(pool, player_id, round, price.price).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error setting player price"),
    }
}

pub fn players_controller() -> actix_web::Scope {
    web::scope("/players")
        .service(players_get)
        .service(players_get_remaining)
        .service(players_get_by_id)
        .service(players_eliminate)
        .service(players_create)
        .service(players_delete)
        .service(players_bulk_create)
        .service(players_get_price)
        .service(players_set_price)
}
