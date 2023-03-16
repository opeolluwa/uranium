use axum::response::{IntoResponse, Redirect};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use std::env;

pub async fn google_auth() -> impl IntoResponse {
    // use std::io::{BufRead, BufReader, Write};
    // use std::net::TcpListener;
    // use url::Url;

    // new discord 0Auth2 config
    // const DISCORD_CLIENT_ID: Lazy<ClientId> =
    //     Lazy::new(|| env::var("DISCORD_CLIENT_ID").expect("Discord client ID not provided"));
    // const DISCORD_CLIENT_ID: ClientId = ClientId::new(
    //     env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    // );

    // const DISCORD_CLIENT_SECRET: ClientSecret = ClientSecret::new(
    //     env::var("GOOGLE_CLIENT_SECRET")
    //         .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    // );
    // const AUTH_URL: AuthUrl = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
    //     .expect("Invalid authorization endpoint URL");
    // const TOKEN_URL: TokenUrl = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
    //     .expect("Invalid token endpoint URL");

    // build new google )auth Client
    // const GOOGLE_CLIENT_ID: ClientId = ClientId::new(
    //     env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    // );
    // const GOOGLE_CLIENT_SECRET: ClientSecret = ClientSecret::new(
    //     env::var("GOOGLE_CLIENT_SECRET")
    //         .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    // );
    // const AUTH_URL: AuthUrl = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
    //     .expect("Invalid authorization endpoint URL");
    // const TOKEN_URL: TokenUrl = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
    //     .expect("Invalid token endpoint URL");
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client = BasicClient::new(
        ClientId::new("client_id".to_string()),
        Some(ClientSecret::new("client_secret".to_string())),
        AuthUrl::new("http://authorize".to_string()).unwrap(),
        Some(TokenUrl::new("http://token".to_string()).unwrap()),
    )
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string()).unwrap());

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new(
            "some authorization code".to_string(),
        ))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await
        .unwrap();

    println!("{:?}", token_result);
}

pub async fn twitter_auth() -> &'static str {
    "Hello, World!"
}

pub async fn discord_auth() -> impl IntoResponse {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let client_id = env::var("DISCORD_CLIENT_ID").expect("Missing  DISCORD_CLIENT_ID!");
    let client_secret = env::var("DISCORD_CLIENT_SECRET").expect("Missing DISCORD_CLIENT_SECRET!");
    let redirect_url = env::var("DISCORD_REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://discord.com/api/oauth2/authorize?response_type=code".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://discord.com/api/oauth2/token".to_string());

    let discord_oauth_client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        AuthUrl::new(auth_url).unwrap(),
        Some(TokenUrl::new(token_url).unwrap()),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap());

    let (auth_url, _csrf_token) = discord_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    // Redirect to Discord's oauth service
    Redirect::to(&auth_url.to_string())
}
// a function to get username

fn _fetch_env(env_var: &str) -> String {
    once_cell::sync::Lazy::new(|| env::var(env_var).expect(&format!("{env_var} not provided")))
        .to_string()
}
