use super::{auth_routes, oauth2};
use axum::Router;

/**
 * this module contains the finale level of route nesting
 * it abstracts routes like /oauth2/discord/verify away using the axum .nest() API
 */
pub fn router() -> axum::Router {
    Router::new()
        .nest("/auth", auth_routes::routes())
        .nest("/oauth2", oauth2::routes())
}
