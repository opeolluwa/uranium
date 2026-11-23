use axum::extract::FromRef;
use sea_orm::DatabaseConnection;

use super::{Contracts, ServicesState};
use crate::config::env::AppConfig;
use crate::config::redis::init_redis_pool;
use crate::errors::app_error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub services: ServicesState,
    pub database_connection: DatabaseConnection,
    pub app_config: AppConfig,
}

impl FromRef<AppState> for ServicesState {
    fn from_ref(state: &AppState) -> Self {
        state.services.clone()
    }
}

impl AppState {
    pub async fn new(db: &DatabaseConnection, app_config: AppConfig) -> Result<Self, AppError> {
        let redis = init_redis_pool(&app_config).await?;
        let contracts = Contracts::new(&app_config)?;
        let services = ServicesState::new(db, &contracts, redis, &app_config);

        Ok(Self {
            services,
            database_connection: db.clone(),
            app_config,
        })
    }
}
