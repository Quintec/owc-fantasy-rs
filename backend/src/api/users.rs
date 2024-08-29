use crate::db::teams::{get_round_team_by_user_id, get_teams_by_user_id};
use crate::db::users::{get_all_users, get_user_by_id};
use actix_web::{get, web, HttpResponse, Responder};
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

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users")
        .service(users_get)
        .service(users_get_by_id)
}
