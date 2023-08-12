use axum::{routing::get, Router};

use crate::{handlers::users::UserInformationHandler,config::app_state::AppState, };

pub fn router<S>(state: AppState) -> axum::Router<S> {
    Router::new()
        .route("/", get(UserInformationHandler::get_user_information))
        .route(
            "/id",
            get(UserInformationHandler::get_user_information_by_id),
        )
        .with_state(state)
}
