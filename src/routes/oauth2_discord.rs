use crate::controllers::oauth2_discord as handler;
use axum::{routing::get, Router};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new().route("/", get(handler::request_auth))
    // .route("/verify", get(handler::twitter_auth))
    // .route("/logout", get(handler::discord_auth))
}
