use axum::{http::StatusCode, response::IntoResponse};

use crate::errors::database_error::DatabaseError;

#[derive(thiserror::Error, Debug, Clone,)]
pub enum ServiceError {
    #[error("Failed to start up service")]
    InitializationFailed,
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] DatabaseError),
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            ServiceError::InitializationFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
