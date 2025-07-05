use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::errors::shared_service_error::ServiceError;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("invalid password")]
    InvalidPassword,
    #[error("{0}")]
    OperationFailed(String),
    #[error("duplicate record: {0}")]
    ConflictError(String),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
}

impl UserServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidPassword => StatusCode::UNAUTHORIZED,
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConflictError(_) => StatusCode::CONFLICT,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for UserServiceError {
    fn into_response(self) -> axum::response::Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
