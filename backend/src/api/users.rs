use crate::db::models::User;
use crate::db::users::get_all_users;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("/get")]
async fn users_get(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let users: Vec<User> = get_all_users(pool).await;

    HttpResponse::Ok().json(users)
}

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users").service(users_get)
}
