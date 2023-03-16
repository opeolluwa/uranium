use axum::Router;

use super::oauth2_discord as discord;

/**
 * an  abstraction over all oauth2 route services
 * the module encapsulate the main routs available for 0auth services
 * -/  to connect to the auth server
 * /verify - to validate the returned token
 * /logout - to destroy the token
 *
 * example route will be
 * /discord
 * /discord/verify
 * /discord/logout
 *
 * /google
 * /google/verify
 * /google/logout
 */
pub fn routes() -> axum::Router {
    Router::new().nest("/discord", discord::routes())
}
