use axum::extract::FromRef;
use redis::aio::ConnectionManager;
use sea_orm::DatabaseConnection;

use super::Contracts;
use crate::config::env::AppConfig;
use crate::services::auth_service::AuthenticationService;
use crate::services::otp_service::OtpService;
use crate::services::root_service::RootService;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct ServicesState {
    pub user_service: UserService,
    pub root_service: RootService,
    pub auth_service: AuthenticationService,
    pub token_service: TokenService,
    pub otp_service: OtpService,
}

impl ServicesState {
    pub fn new(
        db: &DatabaseConnection,
        contracts: &Contracts,
        redis: ConnectionManager,
        app_config: &AppConfig,
    ) -> Self {
        let token_service = TokenService::new(app_config, redis);
        let otp_service = OtpService::init(db);

        Self {
            user_service: UserService::init(db),
            root_service: RootService::init(),
            auth_service: AuthenticationService::init(
                db,
                contracts.email.clone(),
                token_service.clone(),
                otp_service.clone(),
                app_config.frontend_url.clone(),
            ),
            token_service,
            otp_service,
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

impl FromRef<ServicesState> for TokenService {
    fn from_ref(state: &ServicesState) -> Self {
        state.token_service.clone()
    }
}

impl FromRef<ServicesState> for OtpService {
    fn from_ref(state: &ServicesState) -> Self {
        state.otp_service.clone()
    }
}
