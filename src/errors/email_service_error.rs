use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::adapters::message::Message;

#[derive(thiserror::Error, Debug)]
pub enum EmailServiceError {
    #[error("failed to send email")]
    SendEmailError,
    #[error("failed due to {0}")]
    OperationFailed(String),
    #[error("failed due to {0}")]
    ProviderError(String),
    #[error("failed due to {0}")]
    DeliveryError(String),
}

impl EmailServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::SendEmailError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::OperationFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ProviderError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DeliveryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for EmailServiceError {
    fn into_response(self) -> axum::response::Response {
        (self.status_code(), Json(Message::new(&self.to_string()))).into_response()
    }
}
