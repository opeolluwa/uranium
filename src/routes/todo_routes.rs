//! #user profile routes
// import the user controllers
use crate::controllers::todo_controllers;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/", post(todo_controllers::add_todo))
        .route("/", put(todo_controllers::edit_todo))
        .route("/", get(todo_controllers::get_all_todo))
        .route("/:todo_id", get(todo_controllers::get_todo_by_id))
        .route("/:todo_id", delete(todo_controllers::delete_todo))
}
