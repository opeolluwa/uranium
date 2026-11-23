use sea_orm::DatabaseConnection;

use super::{Contracts, Repositories, ServicesState};
use crate::config::env::AppConfig;
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
        let contracts = Contracts::new(&config)?;
        let repositories = Repositories::new(db);
        let services = ServicesState::new(db, &contracts);

        Ok(Self {
            services,
            db: db.clone(),
            config,
            repositories,
        })
    }
}
