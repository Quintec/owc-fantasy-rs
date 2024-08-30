use actix_session::storage::CookieSessionStore;
use actix_session::{CookieContentSecurity, SessionMiddleware};
use actix_web::cookie::Key;
use actix_web::{web, App, HttpServer};
use rand::Rng;
mod api;
mod config;
mod db;
mod routes;
mod state;

fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..64).map(|_| rng.gen()).collect();
    let builder =
        SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&random_bytes));
    builder
        .cookie_content_security(CookieContentSecurity::Private)
        .build()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    config::init();

    let pool = db::pool::create_pool().await;
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(session_middleware())
            .app_data(web::Data::new(state::AppState { pool: pool.clone() }))
            .configure(routes::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
