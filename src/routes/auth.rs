use axum::{Router, routing::post};

use crate::{
    config::env::AppConfig,
    controllers::auth::{
        create_account, forgotten_password, login, logout, request_new_otp, request_refresh_token,
        set_new_password, verify_account, verify_otp,
    },
    state::ServicesState,
};

pub(super) fn authentication_routes(state: ServicesState, app_config: &AppConfig) -> Router {
    let auth_limited = Router::new()
        .route("/signup", post(create_account))
        .route("/login", post(login))
        .route("/forgotten-password", post(forgotten_password))
        .route("/refresh-token", post(request_refresh_token))
        .route("/reset-password", post(set_new_password))
        .layer(app_config.auth_governor());

    let otp_limited = Router::new()
        .route("/verify-otp", post(verify_otp))
        .route("/verify-account", post(verify_account))
        .route("/request-otp", post(request_new_otp))
        .layer(app_config.otp_governor());

    let unrated = Router::new().route("/logout", post(logout));

    Router::new()
        .merge(auth_limited)
        .merge(otp_limited)
        .merge(unrated)
        .with_state(state)
}
