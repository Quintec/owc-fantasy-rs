use crate::api::{auth, teams, users};
use actix_web::web::{self, service};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(users::users_controller())
            .service(auth::auth_controller())
            .service(teams::teams_controller()),
    );
}
