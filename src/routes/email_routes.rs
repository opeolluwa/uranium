use crate::controllers::email_controllers::{
    archive_email, delete_email, get_email_by_id, get_all_emails, receive_email, reply_email,
    send_email, star_email, un_star_email,
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
        .route("/:email_id", get(get_email_by_id))
        .route("/delete/:email_id", delete(delete_email))
        .route("/star/:email_id", put(star_email))
        .route("/un-star/:email_id", put(un_star_email))
        .route("/archive/:email_id", put(archive_email))
        .route("/reply/:email_id", patch(reply_email))
}
