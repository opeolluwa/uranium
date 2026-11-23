use crate::config::env::{AppConfig, Environment};

pub struct AppLogger {}

pub fn init_tracing(app_config: &AppConfig) {
    let log_level = match &app_config.environment {
        Environment::Production => tracing::Level::INFO,
        _ => tracing::Level::DEBUG,
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .compact()
        .init();

    tracing::info!("Logger initialized");
}
