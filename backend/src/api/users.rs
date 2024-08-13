use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

use serde::Serialize;

#[derive(Debug, Serialize)]
struct User {
    id: i32,
    username: String,
    email: Option<String>,
}


#[get("/get")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let users: Vec<User> = sqlx::query_as!(
        User,
        "SELECT id, username, email FROM Users"
    )
    .fetch_all(pool)
    .await
    .unwrap();

    HttpResponse::Ok().json(users)
}

pub fn users_controller() -> actix_web::Scope {
    web::scope("/users").service(get_users)
}
