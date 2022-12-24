use crate::controllers::email_controllers::{
    delete_email, fetch_email, get_all_emails, receive_email, reply_email, send_email, star_email,
};
use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn routes() -> axum::Router {
    Router::new()
        .route("/", post(receive_email))
        .route("/", get(get_all_emails))
        .route("/send", post(send_email))
        .route("/reply", post(receive_email))
        .route("/:email_id", get(fetch_email))
        .route("/:email_id", delete(delete_email))
        .route("/star/:email_id", put(star_email))
        .route("/:email_id", patch(reply_email))
}
