use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ServerError { message: Option<String> },
    BadCredentialsError { message: Option<String> },
    WrongCredentialsError { message: Option<String> },
    DatabaseError { message: Option<String> },
    NotFoundError { message: Option<String> },
    ConflictError { message: Option<String> },
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AppError::ServerError { message } | AppError::DatabaseError { message } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                message.or(Some(
                    "The server couldn't process the request at this time, please try again later "
                        .to_string(),
                )),
            ),
            AppError::BadCredentialsError { message } => (StatusCode::BAD_REQUEST, message.or(Some("The request was badly formatted".to_string()))),
            AppError::WrongCredentialsError { message } => (StatusCode::UNAUTHORIZED, message.or(Some("Incorrect authorization credential".to_string()))),
            AppError::NotFoundError { message } => (StatusCode::NOT_FOUND, message.or(Some("The resource you are looking for does not exist on this server or has been permanently  removed".to_string()))),
            AppError::ConflictError { message } => (StatusCode::CONFLICT, message.or(Some("The resource you are trying to create already exist".to_string()))),
        };

        let response_body = Json(json!({
            "message":Some(error_message)
        }));

        (status, response_body).into_response()
    }
}
