use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, Form, FromRequest},
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

pub mod auth;

// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidatedRequest<T>(pub T);

// #[async_trait]
// impl<T, S> FromRequest<S, T> for ValidatedRequest<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Form<T>: FromRequest<S, T, Rejection = FormRejection>,
// {
//     type Rejection = ServerError;

//     async fn from_request(req: Request<T>, state: &S) -> Result<Self, Self::Rejection> {
//         let Form(value) = Form::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidatedRequest(value))
//     }
// }

// #[derive(Debug, Error)]
// pub enum ServerError {
//     #[error(transparent)]
//     ValidationError(#[from] validator::ValidationErrors),

//     #[error(transparent)]
//     AxumFormRejection(#[from] FormRejection),
// }

// impl IntoResponse for ServerError {
//     fn into_response(self) -> Response {
//         match self {
//             ServerError::ValidationError(_) => {
//                 let message = format!("Input validation error: [{}]", self).replace('\n', ", ");
//                 (StatusCode::BAD_REQUEST, message)
//             }
//             ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
//         }
//         .into_response()
//     }
// }
