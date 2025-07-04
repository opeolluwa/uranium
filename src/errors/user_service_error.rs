use axum::{http::StatusCode, response::IntoResponse};
use bcrypt::BcryptError;

use crate::errors::shared_service_error::ServiceError;

#[derive(thiserror::Error, Debug, Clone)]
pub enum UserServiceError {
    #[error("Invalid password")]
    InvalidPassword,
    #[error("{0}")]
    OperationFailed(String),
    #[error("Dulicate record: {0}")]
    ConflictError(String),   
    #[error(transparent)]
    ServiceError(#[from]  ServiceError),
}

impl IntoResponse for UserServiceError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            UserServiceError::InvalidPassword => StatusCode::UNAUTHORIZED,
            UserServiceError::OperationFailed(_)
            | UserServiceError::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            UserServiceError::ConflictError(_) => StatusCode::CONFLICT,
        };

        (status, self.to_string()).into_response()
    }
}
