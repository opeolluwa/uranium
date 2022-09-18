use crate::controllers::email_controllers::{
    delete_email, fetch_email, receive_email, reply_email, send_email, star_email,
};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn routes() -> axum::Router {
    Router::new()
        .route("/", post(receive_email))
        .route("/send", post(send_email))
        .route("/reply", post(receive_email))
        .route("/:emailId", get(fetch_email))
        .route("/:emailId", delete(delete_email))
        .route("/:emailId", put(star_email))
        .route("/:emailId", patch(reply_email))
}
