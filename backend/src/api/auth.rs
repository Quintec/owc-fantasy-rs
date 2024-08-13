use std::env;

use crate::state::AppState;
use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use reqwest;
use serde::Deserialize;

#[derive(Deserialize)]
struct OAuth2Callback {
    code: String,
}

#[derive(Deserialize, Debug)]
struct OsuUser {
    id: u32,
    username: String,
    avatar_url: String,
}

async fn get_oauth2_client() -> BasicClient {
    BasicClient::new(
        ClientId::new(env::var("OAUTH_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(env::var("OAUTH_CLIENT_SECRET").unwrap())),
        AuthUrl::new(env::var("OAUTH_URL").unwrap()).unwrap(),
        Some(TokenUrl::new(env::var("OAUTH_TOKEN_URL").unwrap()).unwrap()),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080/api/auth/callback".to_string()).unwrap(),
    )
}

#[get("/login")]
async fn oauth2_login(session: Session) -> impl Responder {
    let client = get_oauth2_client().await;
    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    session
        .insert("pkce_verifier", pkce_verifier.secret())
        .unwrap();
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("public".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();
    // print auth url
    println!("Auth URL: {}", auth_url);
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
    let client = get_oauth2_client().await;
    let pkce_verifier = PkceCodeVerifier::new(session.get("pkce_verifier").unwrap().unwrap());
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token_response) => {
            // make request to https://osu.ppy.sh/api/v2/me
            let user_req = reqwest::Client::new()
                .get("https://osu.ppy.sh/api/v2/me")
                .header(
                    "Authorization",
                    format!("Bearer {}", token_response.access_token().secret()),
                )
                .send()
                .await
                .unwrap();
            let user_info = user_req.json::<OsuUser>().await.unwrap();
            // print user info
            println!("User info: {:?}", user_info);
            HttpResponse::Ok().finish()
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn auth_controller() -> actix_web::Scope {
    web::scope("/auth")
        .service(oauth2_login)
        .service(oauth2_callback)
}
