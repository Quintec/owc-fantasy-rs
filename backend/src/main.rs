use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
mod api;
mod config;
mod db;
mod middleware;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let pool = db::pool::create_pool().await;
    HttpServer::new(move || {
        App::new()
            // TODO: change allowed_origin for production environment
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .wrap(middleware::session::session_middleware())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state::AppState { pool: pool.clone() }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
