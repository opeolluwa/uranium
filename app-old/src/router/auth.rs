use axum::{routing::get, routing::post};

use crate::{config::app_state::AppState, handlers::auth::UserAuthenticationHandler as Handler};

pub fn router<S>(state: AppState) -> axum::Router<S> {
    axum::Router::new()
        .route("/signup", get(Handler::sign_up))
        .route("/verify", get(Handler::verify_magic_link))
        .route("/login", post(Handler::login))
        .route("/logout", get(Handler::logout))
        .with_state(state)
}
