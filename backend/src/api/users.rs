use crate::db::models::User;
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

// get user by id under path /users/get/{id}
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

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users")
        .service(users_get)
        .service(users_get_by_id)
}
