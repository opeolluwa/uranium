use crate::config::app_state::AppState;

mod auth;
mod users;

pub fn routes() -> axum::Router {
    let state = AppState {
        // database: connection,
    };
    axum::Router::new()
        .nest("/users", users::router(state))
        .nest("/auth", auth::router(state))
}
