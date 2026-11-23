use sea_orm::DatabaseConnection;

use super::{Contracts, Repositories, ServicesState};
use crate::config::env::AppConfig;
use crate::config::redis::init_redis_pool;
use crate::errors::app_error::AppError;

pub struct Application {
    pub config: AppConfig,
    pub db: DatabaseConnection,
    pub repositories: Repositories,
    pub services: ServicesState,
}

impl Application {
    pub async fn new(db: &DatabaseConnection) -> Result<Self, AppError> {
        let config = AppConfig::from_env()?;
        let redis = init_redis_pool(&config).await?;
        let contracts = Contracts::new(&config)?;
        let repositories = Repositories::new(db);
        let services = ServicesState::new(db, &contracts, redis, &config);

        Ok(Self {
            services,
            db: db.clone(),
            config,
            repositories,
        })
    }
}
