//! #user profile routes
// import the user controllers
use crate::controllers::notes_controllers;
use axum::{
    routing::{get, post, put, delete},
    Router,
};

// mount the controllers to the route
pub fn routes() -> axum::Router {
    Router::new()
        .route("/", post(notes_controllers::add_notes))
        .route("/", get(notes_controllers::get_all_notes))
        .route("/:note_id", get(notes_controllers::get_notes_by_id))
        .route("/:note_id", put(notes_controllers::edit_note))
        .route("/:note_id", delete(notes_controllers::delete_note))
}
