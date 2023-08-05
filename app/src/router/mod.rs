mod users;

pub fn root() -> axum::Router {
    axum::Router::new().nest("/users", users::router())
}
