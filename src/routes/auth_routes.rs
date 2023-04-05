//! #user profile routes
// import the user controllers
use crate::controllers::auth_controllers as handler;
use axum::{
    routing::{get, post, put},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/sign-up", post(handler::sign_up))
        .route("/login", post(handler::login))
        .route("/logout", post(handler::logout))
        .route("/verify-email", post(handler::verify_email))
        .route(
            "/request-verification",
            post(handler::request_account_verification),
        )
        .route("/request-new-otp", post(handler::request_new_otp))
        .route(
            "/request-password-reset",
            post(handler::request_password_reset),
        )
        .route("/reset-password", put(handler::reset_password))
        .route("/me", get(handler::fetch_user_profile))
        .route("/me", put(handler::update_user_profile))
        .route("/", get(handler::get_refresh_token))
}
