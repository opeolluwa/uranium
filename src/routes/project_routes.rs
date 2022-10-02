//! #user profile routes
// import the user controllers
use crate::controllers::project_controllers;
use axum::{
    routing::{get, post, put},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/", post(project_controllers::add_project))
        .route("/", put(project_controllers::edit_project))
        .route("/", get(project_controllers::get_all_projects))
}
