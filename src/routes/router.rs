
use axum::Router;
use sqlx::{Pool, Postgres};

use crate::{
    routes::{auth::authentication_routes, public::public_routes},
    services::{
        auth_service::AuthenticationService, root_service::RootService, user_service::UserService,
    },
    states::services_state::ServicesState,
};

pub fn load_routes(pool: Pool<Postgres>) -> Router {
    let state = ServicesState {
        user_service: UserService::init(&pool),
        root_service: RootService::init(),
        auth_service: AuthenticationService::init(&pool),
    };

    Router::new()
        .merge(public_routes(state.clone()))
        .merge(authentication_routes(state.clone()))
}
