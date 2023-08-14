use crate::config::app_state::AppState;

mod auth;
mod users;

pub fn routes(state: &AppState) -> axum::Router {
    axum::Router::new()
        .nest("/users", users::router(state.clone()))
        .nest("/auth", auth::router(state.clone()))
}
