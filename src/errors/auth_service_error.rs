use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::adapters::message::Message;
use crate::errors::{
    app_error::AppError, common_service_error::ServiceError,
    email_service_error::EmailServiceError, otp_service_error::OtpServiceError,
    user_service_error::UserServiceError,
};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationServiceError {
    #[error("Invalid email or password")]
    WrongCredentials,
    #[error("Missing authorization headers")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid token type for this operation")]
    InvalidTokenType,
    #[error("Token has been revoked")]
    TokenBlacklisted,
    #[error("Refresh token has already been used")]
    RefreshTokenConsumed,
    #[error("Account is inactive")]
    AccountInactive,
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error(transparent)]
    OtpError(#[from] OtpServiceError),
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),
    #[error(transparent)]
    AppError(#[from] AppError),
    #[error("error processing authorization token")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    EmailError(#[from] EmailServiceError),
}

impl AuthenticationServiceError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::WrongCredentials => StatusCode::UNAUTHORIZED,
            Self::MissingCredentials => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::InvalidTokenType => StatusCode::UNAUTHORIZED,
            Self::TokenBlacklisted => StatusCode::UNAUTHORIZED,
            Self::RefreshTokenConsumed => StatusCode::UNAUTHORIZED,
            Self::AccountInactive => StatusCode::FORBIDDEN,
            Self::RedisError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::OtpError(err) => err.status_code(),
            Self::ServiceError(err) => err.status_code(),
            Self::UserServiceError(err) => err.status_code(),
            Self::AppError(err) => err.status_code(),
            Self::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::EmailError(err) => err.status_code(),
        }
    }
}

impl IntoResponse for AuthenticationServiceError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let message = match status {
            StatusCode::INTERNAL_SERVER_ERROR => "an internal error occurred".to_string(),
            _ => self.to_string(),
        };
        (status, Json(Message::new(&message))).into_response()
    }
}
