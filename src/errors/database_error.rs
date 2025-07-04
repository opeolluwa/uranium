use axum::{http::StatusCode, response::IntoResponse};

#[derive(thiserror::Error, Debug, Clone,)]
pub enum DatabaseError {
    #[error("The record already exists")]
    ConflictError,
   
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            DatabaseError::ConflictError => StatusCode::INTERNAL_SERVER_ERROR,
       
        };

        (status, self.to_string()).into_response()
    }
}
