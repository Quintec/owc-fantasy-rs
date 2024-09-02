use actix_web::{web, App, HttpServer};
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
            .wrap(middleware::session::session_middleware())
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(state::AppState { pool: pool.clone() }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
