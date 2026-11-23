use axum::extract::FromRef;
use jsonwebtoken::DecodingKey;
use sea_orm::DatabaseConnection;

use super::{Contracts, ServicesState};
use crate::config::env::AppConfig;
use crate::errors::app_error::AppError;
use crate::utils::extract_env::extract_env;

#[derive(Clone)]
pub struct AppState {
    pub services: ServicesState,
    pub database_connection: DatabaseConnection,
    pub app_config: AppConfig,
    pub jwt_decoding_key: DecodingKey,
}

impl FromRef<AppState> for ServicesState {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

impl AppState {
    pub async fn new(db: &DatabaseConnection, app_config: AppConfig) -> Result<Self, AppError> {
        let contracts = Contracts::new(&app_config)?;
        let services = ServicesState::new(db, &contracts);

        let jwt_secret: String = extract_env("JWT_SIGNING_KEY")?;
        let jwt_decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

        Ok(Self {
            services,
            database_connection: db.clone(),
            app_config,
            jwt_decoding_key,
        })
    }
}
