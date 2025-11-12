use crate::db::teams::{
    get_round_team_by_user_id, get_teams_by_user_id,
};
use crate::db::users::{get_all_users, get_user_by_id, get_leaderboard, get_leaderboard_breakdown, get_user_data_counts};
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

#[get("/me")]
async fn users_get_me(session: actix_session::Session, data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;
    let user_id = session.get::<i32>("user_id").unwrap_or(None);
    match user_id {
        Some(id) => {
            let user = get_user_by_id(pool, id).await;
            match user {
                Ok(user) => HttpResponse::Ok().json(user),
                Err(_) => HttpResponse::NotFound().body("User not found"),
            }
        }
        None => HttpResponse::Unauthorized().body("Not logged in"),
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

#[get("/leaderboard")]
async fn users_get_leaderboard(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let leaderboard = get_leaderboard(pool).await;
    match leaderboard {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching leaderboard"),
    }
}

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users")
        .service(users_get)
        .service(users_get_me)
        .service(users_get_leaderboard)
        .service(users_get_by_id)
        .service(users_get_teams)
        .service(users_get_team_by_round)
}
