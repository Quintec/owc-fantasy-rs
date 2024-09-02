use crate::{
    api::{auth, players, teams, users},
    middleware::auth::auth_middleware,
};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .wrap(actix_web::middleware::from_fn(auth_middleware))
            .service(users::users_controller())
            .service(auth::auth_controller())
            .service(players::players_controller())
            .service(teams::teams_controller()),
    );
}
