use std::{env, str::FromStr};

use crate::errors::app_error::AppError;

pub fn extract_env<T: FromStr>(env_key: &str) -> Result<T, AppError> {
    let env = env::var(env_key)
        .map_err(|err| {
            tracing::error!("error fetching env {}: {}", env_key, err);
            AppError::EnvError(env_key.to_string())
        })?
        .parse::<T>()
        .map_err(|_| {
            tracing::error!("error parsing env {}", env_key);
            AppError::EnvError("error parsing env".into())
        })?;

    Ok(env)
}
