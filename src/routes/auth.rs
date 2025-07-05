use axum::{Router, routing::post};

use crate::{
    controllers::auth::{create_account, forgotten_password, login, set_new_password},
    states::services_state::ServicesState,
};

pub(super) fn authentication_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/signup", post(create_account))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/reset-password", post(set_new_password))
        .with_state(state)
}
