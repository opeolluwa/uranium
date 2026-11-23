use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::adapters::message::Message;
use crate::errors::common_service_error::ServiceError;

#[derive(thiserror::Error, Debug)]
pub enum UserServiceError {
    #[error("{0}")]
    OperationFailed(String),
    #[error("duplicate record: {0}")]
    ConflictError(String),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
}

impl UserServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ConflictError(_) => StatusCode::CONFLICT,
            Self::ServiceError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for UserServiceError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), Json(Message::new(&self.to_string()))).into_response()
    }
}
