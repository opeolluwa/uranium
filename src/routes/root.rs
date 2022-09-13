use super::auth;
use axum::Router;

pub fn router() -> axum::Router {
    Router::new().nest("/auth", auth::routes())
}
