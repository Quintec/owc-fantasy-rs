use std::env;

use crate::db::models::User;
use crate::db::users::{create_user, get_user_by_id};
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
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    if let Err(_) = session.insert("pkce_verifier", pkce_verifier.secret()) {
        return HttpResponse::InternalServerError().body("Auth challenge error");
    }
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("public".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

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
    let pool = &data.pool;
    let client = get_oauth2_client().await;
    let pkce_verifier_secret = session.get::<String>("pkce_verifier").unwrap_or(None);
    let Some(pkce_verifier_secret_data) = pkce_verifier_secret else {
        return HttpResponse::InternalServerError().body("Auth flow error");
    };
    session.remove("pkce_verifier");
    let pkce_verifier = PkceCodeVerifier::new(pkce_verifier_secret_data);
    let token_result = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .set_pkce_verifier(pkce_verifier)
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
            HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()
        }
        Err(_) => HttpResponse::InternalServerError().body("Auth token error"),
    }
}

pub fn auth_controller() -> actix_web::Scope {
    web::scope("/auth")
        .service(oauth2_login)
        .service(oauth2_callback)
}
