// use axum::async_trait;
// use axum::extract::rejection::FormRejection;
// use axum::extract::Form;
// use axum::extract::FromRequest;
// use axum::http::Request;
// use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
// use crate::shared::api_response::ApiErrorResponse;

///the email will be stored as a struct having `id` and a nested `context` types
/// the `id ` is essentially a uuid and the context is a json
/// the fields of the context are optional and the fields are `fullname`, `subject` , `email` and `message`
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmailSchema {
    /// email id
    pub id: Uuid,
    /// email context will abe an growable array of email-context
    pub context: Vec<EmailContext>,
}

/// the email context
/// derive sqlx::FromRow  trait to make the struct queryable as database model
#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EmailModel {
    ///a uniques identifier for the email item essentially a UUID 
    id:Uuid,
    /// the sender fullname
    #[validate(length(min = 1, "sender name seems invalid")) ]
    pub sender_name: String,
    /// the sender or recipient email address
    #[validate(email)]
    pub sender_email: String,
    ///the email subject
    #[validate(length(min = 1, "email subject cannot be empty "))]
    pub email_subject: String,
    /// the message content
    #[validate(length(min = 10, "message body may only be longer than 10 characters"))]
    pub email_body: String,
}


/// the email context
/// derive sqlx::FromRow  trait to make the struct queryable as database model
#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow)]
pub struct EmailContext {
    /// the sender fullname
    #[validate(length(min = 1, "sender name seems invalid"))]
    pub fullname: String,
    /// the sender or recipient email address
    #[validate(email)]
    pub email: String,
    ///the email subject
    #[validate(length(min = 1, "email subject cannot be empty "))]
    pub subject: String,
    /// the message content
    #[validate(length(min = 10, "message body may only be longer than 10 characters"))]
    pub message: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

// implement validation for the email payload
// #[async_trait]
// impl<T, S, B> FromRequest<S, Rejection = B> for ValidatedForm<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Form<T>: FromRequest<S, B, Rejection = FormRejection>,
//     B: Send + 'static,
// {
//     type Rejection = ApiErrorResponse;

//     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
//         let Form(value) = Form::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidatedForm(value))
//     }
// }


// TODO implement enumerate field for email context