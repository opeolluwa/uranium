use super::{auth_routes, oauth2};
use axum::Router;

pub fn router() -> axum::Router {
    Router::new()
        .nest("/auth", auth_routes::routes())
        .nest("/oauth2", oauth2::routes())
}
