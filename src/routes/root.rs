use super::{auth_routes, email_routes};
use axum::Router;

pub fn router() -> axum::Router {
    Router::new()
        .nest("/auth", auth_routes::routes())
        .nest("/email", email_routes::routes())
}
