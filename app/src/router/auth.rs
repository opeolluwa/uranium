use axum::{routing::get, routing::post, Router};

use crate::handlers::auth::UserAuthenticationHandler as Handler;

pub fn auth_routes() -> Router {
    axum::Router::new()
        .route("/signup", post(Handler::sign_up))
        .route("/verify", get(Handler::verify_magic_link))
        .route("/login", post(Handler::login))
        .route("/logout", get(Handler::logout))
}
