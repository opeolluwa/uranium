use axum::Router;
use sqlx::{Pool, Postgres};

use crate::{
    services::{root_service::RootService, user_service::UserService},
    states::services_state::ServicesState,
};

pub fn load_routes(pool: Pool<Postgres>) -> Router {
    let user_service = UserService::init(&pool);


    let root_service = RootService::init();

    let services_state = ServicesState {
        user_service,
        root_service,
    };

    Router::new().merge(super::public_routes::public_routes(services_state))
}
