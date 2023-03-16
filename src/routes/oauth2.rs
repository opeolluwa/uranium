use crate::controllers::oauth2 as handler;
use axum::{routing::get, Router};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/google", get(handler::google_auth))
        .route("/twitter", get(handler::twitter_auth))
}
