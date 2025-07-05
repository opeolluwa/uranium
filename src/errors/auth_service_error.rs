use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::errors::{
    app_error::AppError, shared_service_error::ServiceError, user_service_error::UserServiceError,
};

#[derive(Debug, thiserror::Error, Clone)]
pub enum AuthenticationServiceError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation failed")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error(transparent)]
    ServiceError(#[from] ServiceError),
    #[error(transparent)]
    UserServiceError(#[from] UserServiceError),
    #[error(transparent)]
    AppError(#[from] AppError),
    #[error("error processing authorization token")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

impl IntoResponse for AuthenticationServiceError {
    fn into_response(self) -> Response {
        let status = match &self {
            AuthenticationServiceError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthenticationServiceError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthenticationServiceError::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            AuthenticationServiceError::InvalidToken => StatusCode::BAD_REQUEST,

            AuthenticationServiceError::ServiceError(err) => {
                err.to_owned().into_response().status()
            }

            AuthenticationServiceError::UserServiceError(err) => {
                err.to_owned().into_response().status()
            }
            AuthenticationServiceError::AppError(err) => err.to_owned().into_response().status(),
            AuthenticationServiceError::JwtError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
