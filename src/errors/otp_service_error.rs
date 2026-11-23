use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::adapters::message::Message;

#[derive(Debug, thiserror::Error)]
pub enum OtpServiceError {
    #[error("Invalid OTP code")]
    InvalidCode,
    #[error("OTP has expired")]
    Expired,
    #[error("Maximum OTP attempts exceeded")]
    MaxAttemptsExceeded,
    #[error("No active OTP found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Hashing error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
}

impl OtpServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCode => StatusCode::UNAUTHORIZED,
            Self::Expired => StatusCode::BAD_REQUEST,
            Self::MaxAttemptsExceeded => StatusCode::TOO_MANY_REQUESTS,
            Self::NotFound => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) | Self::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for OtpServiceError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "an internal error occurred".to_string(),
            _ => self.to_string(),
        };
        (status, Json(Message::new(&message))).into_response()
    }
}
