use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error("The record already exists")]
    ConflictError,
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] sqlx::Error),
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            DatabaseError::ConflictError => StatusCode::INTERNAL_SERVER_ERROR,
            DatabaseError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}
