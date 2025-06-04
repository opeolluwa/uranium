use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Failed to start up service due to: Err -> ")]
    WrongCredentials,
    #[error("Failed to start up service due to: Err -> ")]
    MissingCredentials,
    #[error("Failed to start up service due to: Err -> ")]
    TokenCreation,
    #[error("Failed to start up service due to: Err ->")]
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };

        (status, error_message).into_response()
    }
}
