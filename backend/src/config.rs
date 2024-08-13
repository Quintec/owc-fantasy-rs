use dotenv::dotenv;
use std::env;

pub fn init() {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
}
