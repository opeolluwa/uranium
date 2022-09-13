//! #user profile routes
// import the user controllers
use crate::controllers::auth;
use axum::{
    routing::{get, post, put},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/sign-up", post(auth::sign_up))
        .route("/login", post(auth::login))
        .route("/reset-password", post(auth::reset_password))
        .route("/me", get(auth::user_profile))
        .route("/update/me", put(auth::update_user_profile))
}
