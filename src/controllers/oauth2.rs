use axum::response::IntoResponse;
/// use social authentication strategies
/// !. Google https://support.google.com/googleapi/answer/6158849
use oauth2::{basic::BasicClient, revocation::StandardRevocableToken, TokenResponse};
// Alternatively, this can be oauth2::curl::http_client or a custom.
use oauth2::reqwest::{async_http_client, http_client};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
// use std::env;
// use std::io::{BufRead, BufReader, Write};
// use std::net::TcpListener;
// use url::Url;

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

pub async fn google_auth() -> impl IntoResponse {
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
    let (auth_url, csrf_token) = client
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
