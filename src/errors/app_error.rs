use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug, Clone)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (message, status_code) = match self {
            AppError::StartupError(err) => (err, StatusCode::INTERNAL_SERVER_ERROR),
            AppError::EnvError(err) => (err, StatusCode::INTERNAL_SERVER_ERROR),
            AppError::OperationFailed(err) => (err, StatusCode::INTERNAL_SERVER_ERROR),
        };

        (status_code, message).into_response()
    }
}
