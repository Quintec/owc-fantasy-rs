use actix_session::{storage::CookieSessionStore, CookieContentSecurity, SessionMiddleware};
use actix_web::cookie::Key;

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    let session_secret = std::env::var("SESSION_SECRET")
        .unwrap_or_else(|_| "0123".repeat(16))
        .into_bytes();
    let builder =
        SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&session_secret));
    builder
        .cookie_content_security(CookieContentSecurity::Private)
        .build()
}
