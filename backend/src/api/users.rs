use crate::db::teams::{
    add_player_to_team, create_team, get_round_team_by_user_id, get_teams_by_user_id,
    remove_player_from_team,
};
use crate::db::users::{get_all_users, get_user_by_id};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("")]
async fn users_get(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let users = get_all_users(pool).await;

    match users {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}

#[get("/{id}")]
async fn users_get_by_id(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let id = path.into_inner();

    let user = get_user_by_id(pool, id).await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[get("/{id}/teams")]
async fn users_get_teams(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let id = path.into_inner();

    let teams = get_teams_by_user_id(pool, id).await;
    match teams {
        Ok(teams) => HttpResponse::Ok().json(teams),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[get("/{id}/teams/{round}")]
async fn users_get_team_by_round(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (id, round) = path.into_inner();

    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&round.as_str()) {
        return HttpResponse::BadRequest().body("Invalid round");
    }

    let team = get_round_team_by_user_id(pool, id, round).await;
    match team {
        Ok(team) => HttpResponse::Ok().json(team),
        Err(_) => HttpResponse::NotFound().body("User not found"),
    }
}

#[post("/{id}/teams/{round}/create")]
async fn users_create_team(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (id, round) = path.into_inner();

    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&round.as_str()) {
        return HttpResponse::BadRequest().body("Invalid round");
    }

    let res = create_team(pool, id, round).await;
    match res {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error creating team"),
    }
}

#[derive(Deserialize)]
struct PlayerInfo {
    player_id: i32,
}

#[post("/{id}/teams/{round}")]
async fn users_add_player_to_team(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
    body: web::Json<PlayerInfo>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (id, round) = path.into_inner();
    let player_id = body.player_id;

    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&round.as_str()) {
        return HttpResponse::BadRequest().body("Invalid round");
    }

    let team_res = get_round_team_by_user_id(pool, id, round).await;

    let Ok(team) = team_res else {
        return HttpResponse::NotFound().body("Team not found");
    };

    let res = add_player_to_team(pool, team.id, player_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error adding player to team"),
    }
}

#[delete("/{id}/teams/{round}")]
async fn users_remove_player_from_team(
    data: web::Data<AppState>,
    path: web::Path<(i32, String)>,
    body: web::Json<PlayerInfo>,
) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let (id, round) = path.into_inner();
    let player_id = body.player_id;

    if !["ro64", "ro32", "ro16", "qf", "sf", "f", "gf"].contains(&round.as_str()) {
        return HttpResponse::BadRequest().body("Invalid round");
    }

    let team_res = get_round_team_by_user_id(pool, id, round).await;

    let Ok(team) = team_res else {
        return HttpResponse::NotFound().body("Team not found");
    };

    let res = remove_player_from_team(pool, team.id, player_id).await;
    match res {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Error removing player from team"),
    }
}

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users")
        .service(users_get)
        .service(users_get_by_id)
        .service(users_get_teams)
        .service(users_get_team_by_round)
        .service(users_create_team)
        .service(users_add_player_to_team)
}
