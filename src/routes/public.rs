use axum::{
    Router,
    routing::{get, post},
};

use crate::{
    controllers::root::{health_check, shut_down},
    states::services_state::ServicesState,
};

pub(super) fn public_routes(state: ServicesState) -> Router {
    Router::new()
        .route("/shutdown", post(shut_down))
        .route("/health", get(health_check))
        .with_state(state)
}
