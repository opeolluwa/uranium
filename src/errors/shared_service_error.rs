use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::errors::database_error::DatabaseError;
use axum::extract::rejection::FormRejection;
use axum::response::Response;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Failed to start up service")]
    InitializationFailed,
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error("failed to perform operation")]
    OperationFailed(String),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::InitializationFailed
            | ServiceError::DatabaseError(_)
            | ServiceError::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        ApiResponseBuilder::<()>::new()
            .status_code(self.status_code())
            .message(&self.to_string())
            .build()
            .into_response()
    }
}
