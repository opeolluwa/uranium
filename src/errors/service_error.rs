#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Failed to start up service due to: Err -> {0}")]
    InitializationFailed(String),
}
