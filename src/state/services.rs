use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use super::Contracts;
use crate::services::auth_service::AuthenticationService;
use crate::services::root_service::RootService;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
}

impl ServicesState {
    pub fn new(db: &DatabaseConnection, contracts: &Contracts) -> Self {
        Self {
            user_service: UserService::init(db),
            root_service: RootService::init(),
            auth_service: AuthenticationService::init(db, contracts.email.clone()),
        }
    }
}

impl FromRef<ServicesState> for UserService {
    fn from_ref(state: &ServicesState) -> Self {
        state.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(state: &ServicesState) -> Self {
        state.root_service.clone()
    }
}

impl FromRef<ServicesState> for AuthenticationService {
    fn from_ref(state: &ServicesState) -> Self {
        state.auth_service.clone()
    }
}
