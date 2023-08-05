use axum::{routing::get, Router};

use crate::handlers::users::UserInformationHandler;

pub fn router() -> axum::Router {
    Router::new()
        .route("/", get(UserInformationHandler::get_user_information))
        .route(
            "/id",
            get(UserInformationHandler::get_user_information_by_id),
        )
}
