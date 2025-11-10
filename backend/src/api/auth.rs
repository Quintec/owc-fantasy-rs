use std::env;

use crate::db::models::User;
use crate::db::users::{create_user, get_user_by_id};
use crate::state::AppState;
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct OAuth2Callback {
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

async fn get_oauth2_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(env::var("OAUTH_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("OAUTH_CLIENT_SECRET").unwrap())),
        AuthUrl::new(env::var("OAUTH_URL").unwrap()).unwrap(),
        Some(TokenUrl::new(env::var("OAUTH_TOKEN_URL").unwrap()).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(env::var("REDIRECT_URL").unwrap()).unwrap())
}

#[get("/login")]
async fn oauth2_login(session: Session) -> impl Responder {
    let client = get_oauth2_client().await;
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .add_scope(Scope::new("identify".to_string()))
        .url();

    if let Err(_) = session.insert("csrf_token", csrf_token.secret()) {
        return HttpResponse::InternalServerError().body("Auth challenge error");
    }

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

#[get("/callback")]
async fn oauth2_callback(
    query: web::Query<OAuth2Callback>,
    session: Session,
    data: web::Data<AppState>,
) -> impl Responder {
    // Check if OAuth provider returned an error
    if let Some(error) = &query.error {
        let error_desc = query.error_description.as_deref().unwrap_or("Unknown error");
        return HttpResponse::BadRequest().body(format!("OAuth error: {} - {}", error, error_desc));
    }

    // Ensure we have the code parameter
    let Some(code) = &query.code else {
        return HttpResponse::BadRequest().body("Missing authorization code");
    };

    let pool = &data.pool;
    let client = get_oauth2_client().await;
    
    // Remove CSRF token from session
    session.remove("csrf_token");
    
    let token_result = client
        .exchange_code(AuthorizationCode::new(code.clone()))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token_response) => {
            let user_req = reqwest::Client::new()
                .get("https://osu.ppy.sh/api/v2/me")
                .header(
                    "Authorization",
                    format!("Bearer {}", token_response.access_token().secret()),
                )
                .send()
                .await
                .expect("Error sending request to osu! api");
            let user_info = user_req
                .json::<User>()
                .await
                .expect("Error parsing user info");

            let user_id = user_info.id;

            let user = get_user_by_id(pool, user_info.id).await;
            if user.is_err() {
                let res = create_user(pool, user_info).await;
                if res.is_err() {
                    return HttpResponse::InternalServerError().body("Error creating user");
                }
            }

            if let Err(_) = session.insert("user_id", user_id) {
                return HttpResponse::InternalServerError().body("Error saving user info");
            }
            // store access token in session so endpoints can call osu! on behalf of the user
            let _ = session.insert("osu_token", token_response.access_token().secret());
            
            let frontend_url = env::var("FRONTEND_URL").unwrap();
            HttpResponse::Found()
                .append_header(("Location", frontend_url))
                .finish()
        }
        Err(_) => HttpResponse::InternalServerError().body("Auth token error"),
    }
}

#[post("/logout")]
async fn oauth2_logout(session: Session) -> impl Responder {
    session.remove("user_id");
    session.remove("osu_token");
    HttpResponse::Ok().body("Logged out")
}

pub fn auth_controller() -> actix_web::Scope {
    web::scope("/auth")
        .service(oauth2_login)
        .service(oauth2_callback)
        .service(oauth2_logout)
}
