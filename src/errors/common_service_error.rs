use axum::extract::rejection::{FormRejection, JsonRejection};
use axum::response::Response;
use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::adapters::message::Message;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("an internal database error has occurred")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl ServiceError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ServiceError::AxumFormRejection(_) => StatusCode::BAD_REQUEST,
            ServiceError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
        }
    }
}
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        (self.status_code(), Json(Message::new(&self.to_string()))).into_response()
    }
}
