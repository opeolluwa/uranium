use axum::{http::StatusCode, response::IntoResponse};

use crate::adapters::api_response::{ApiResponseBuilder, EmptyResponseBody};

#[derive(thiserror::Error, Debug, Clone)]
pub enum AppError {
    #[error("App failed to start up due to {0}")]
    StartupError(String),
    #[error("Error parsing env due to {0}")]
    EnvError(String),
    #[error("{0}")]
    OperationFailed(String),
    #[error("internal server error")]
    InternalServerError,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::StartupError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::EnvError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::StartupError(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<EmptyResponseBody>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
