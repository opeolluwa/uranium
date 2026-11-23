use std::time::Duration;

use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, DatabaseConnection};
use secrecy::ExposeSecret;

use crate::config::env::AppConfig;
use crate::errors::app_error::AppError;

pub async fn init_db_pool(app_config: &AppConfig) -> Result<DatabaseConnection, AppError> {
    let database_url = app_config.database_url.to_owned();

    let mut opt = ConnectOptions::new(database_url.expose_secret());
    opt.max_connections(app_config.max_db_connections)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(app_config.is_development())
        .sqlx_logging_level(log::LevelFilter::Info);

    let db = sea_orm::Database::connect(opt).await.map_err(|err| {
        tracing::error!("Failed to connect to the database: {}", err);
        AppError::InternalServerError
    })?;

    Migrator::up(&db, None).await.map_err(|err| {
        tracing::error!("failed to run database migration due to {}", err);
        AppError::InternalServerError
    })?;

    tracing::info!("Database pool initialized");

    Ok(db)
}
