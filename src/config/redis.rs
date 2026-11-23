use redis::aio::ConnectionManager;
use secrecy::ExposeSecret;

use crate::{config::env::AppConfig, errors::app_error::AppError};

pub async fn init_redis_pool(app_config: &AppConfig) -> Result<ConnectionManager, AppError> {
    let url: &str = app_config.redis_url.expose_secret();
    let client = redis::Client::open(url)
        .map_err(|e| AppError::StartupError(format!("Failed to create Redis client: {e}")))?;

    let manager = ConnectionManager::new(client)
        .await
        .map_err(|e| AppError::StartupError(format!("Failed to connect to Redis: {e}")))?;

    tracing::info!("Redis connection established");
    Ok(manager)
}
