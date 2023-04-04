use axum::extract::Query;
use axum::response::{IntoResponse, Redirect};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

// The user data we'll get back from Discord.
// https://discord.com/developers/docs/resources/user#user-object-user-structure
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
    avatar: Option<String>,
    username: String,
    discriminator: String,
    email: Option<String>,
}

/**
 * 1) Create a new application at <https://discord.com/developers/applications>
* 2) Visit the OAuth2 tab to get your CLIENT_ID and CLIENT_SECRET
*/
pub async fn request_auth() -> impl IntoResponse {
    let (auth_url, _csrf_token) = discord_oauth_client()
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    // Redirect to Discord's oauth service
    Redirect::to(auth_url.as_ref())
}
/// a function to login the user using the returned token
pub async fn verify_auth(Query(query): Query<AuthRequest>) -> impl IntoResponse {
    let token = discord_oauth_client()
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    // Fetch user data from discord
    let client = ::reqwest::Client::new();
    let user_data: User = client
        // https://discord.com/developers/docs/resources/user#get-current-user
        .get("https://discordapp.com/api/users/@me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .unwrap()
        .json::<User>()
        .await
        .unwrap();

    //TODO: use the user data
    println!("\nthe user data is {:?}\n", user_data);
}

// oauth client to interface with discord API
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
