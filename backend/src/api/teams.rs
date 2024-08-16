use crate::db::models::Team;
use crate::db::teams::get_teams_by_user_id;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::MySqlPool;

use crate::state::AppState;

#[get("/get")]
async fn teams_get(data: web::Data<AppState>) -> impl Responder {
    let pool: &MySqlPool = &data.pool;

    let users: Vec<Team> = get_teams_by_user_id(pool, 0).await;

    HttpResponse::Ok().json(users)
}

pub fn teams_controller() -> actix_web::Scope {
    web::scope("/teams").service(teams_get)
}
