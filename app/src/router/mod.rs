mod auth;
mod users;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .nest("/users", users::router())
        .nest("/auth", auth::auth_routes())
}
