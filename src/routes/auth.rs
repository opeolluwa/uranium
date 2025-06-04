use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::auth::{forgotten_password, login, set_new_password, sign_up},
    states::services_state::ServicesState,
};

pub(super) fn authentication_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/signup", post(sign_up))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/reset-password", post(set_new_password))
        .with_state(state)
}
