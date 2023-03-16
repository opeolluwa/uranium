use axum::response::{IntoResponse, Redirect};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use std::env;

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct AuthRequest {
//     code: String,
//     state: String,
// }

pub async fn request_auth() -> impl IntoResponse {
    let (auth_url, _csrf_token) = discord_oauth_client()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    // Redirect to Discord's oauth service
    Redirect::to(&auth_url.to_string())
}
// a function to get username

fn discord_oauth_client() -> BasicClient {
    //TODO: use better error handling
    let client_id = env::var("DISCORD_CLIENT_ID").expect("Missing  DISCORD_CLIENT_ID!");
    let client_secret = env::var("DISCORD_CLIENT_SECRET").expect("Missing DISCORD_CLIENT_SECRET!");
    let redirect_url = env::var("DISCORD_REDIRECT_URL").expect("missing DISCORD_REDIRECT URL");
    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });
    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}
