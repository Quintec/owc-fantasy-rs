use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use std::env;
mod api;
mod config;
mod db;
mod middleware;
mod routes;
mod state;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let cors_origins = env::var("CORS_ORIGINS").unwrap();
    let origins: Vec<String> = cors_origins.split(',').map(|s| s.to_string()).collect();

    let pool = db::pool::create_pool().await;
    HttpServer::new(move || {
        let mut cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        for origin in &origins {
            cors = cors.allowed_origin(origin.trim());
        }

        App::new()
            .wrap(cors)
            .wrap(middleware::session::session_middleware())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state::AppState { pool: pool.clone() }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
