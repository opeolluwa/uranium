//! #user profile routes
// import the user controllers
use crate::controllers::auth_controllers;
use axum::{
    routing::{get, post, put},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/sign-up", post(auth_controllers::sign_up))
        .route("/login", post(auth_controllers::login))
        // .route("/verify-email", put(auth_controllers::verify_email))
        .route("/reset-password", put(auth_controllers::reset_password))
        .route("/me", get(auth_controllers::user_profile))
        .route("/me", put(auth_controllers::update_user_profile))
        .route("/", get(auth_controllers::get_refresh_token))
}
