use crate::api::{auth, players, teams, users};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(users::users_controller())
            .service(auth::auth_controller())
            .service(teams::teams_controller())
            .service(players::players_controller()),
    );
}
