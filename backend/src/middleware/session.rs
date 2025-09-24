use actix_session::{storage::CookieSessionStore, CookieContentSecurity, SessionMiddleware};
use actix_web::cookie::Key;

pub fn session_middleware() -> SessionMiddleware<CookieSessionStore> {
    let session_secret = std::env::var("SESSION_SECRET")
        .unwrap_or_else(|_| "0123".repeat(16))
        .into_bytes();
        let session_secure = std::env::var("SESSION_SECURE").unwrap_or_else(|_| "false".to_string()) == "true";
        let mut builder = SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&session_secret));
        builder = builder
            .cookie_name("owc_fantasy_session".to_string())
            .cookie_content_security(CookieContentSecurity::Private)
            .cookie_secure(session_secure)
            .cookie_same_site(if session_secure {
                actix_web::cookie::SameSite::None
            } else {
                actix_web::cookie::SameSite::Lax
            });
    builder.build()
}
