use axum::extract::FromRef;

use crate::services::{root_service::RootService, user_service::UserService};

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
}

impl FromRef<ServicesState> for UserService {
    fn from_ref(input: &ServicesState) -> UserService {
        input.user_service.clone()
    }
}

impl FromRef<ServicesState> for RootService {
    fn from_ref(input: &ServicesState) -> RootService {
        input.root_service.clone()
    }
}
