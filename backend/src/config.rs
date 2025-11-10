use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    env::var("OAUTH_CLIENT_ID").expect("OAUTH_CLIENT_ID must be set");
    env::var("OAUTH_CLIENT_SECRET").expect("OAUTH_CLIENT_SECRET must be set");
    env::var("OAUTH_URL").expect("OAUTH_URL must be set");
    env::var("OAUTH_TOKEN_URL").expect("OAUTH_TOKEN_URL must be set");
    env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");

    env::var("SESSION_SECRET").expect("SESSION_SECRET must be set");
    env::var("SESSION_SECURE").expect("SESSION_SECURE must be set");
    
    env::var("API_BASE_URL").expect("API_BASE_URL must be set");
    env::var("FRONTEND_URL").expect("FRONTEND_URL must be set");
    env::var("CORS_ORIGINS").expect("CORS_ORIGINS must be set");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}
