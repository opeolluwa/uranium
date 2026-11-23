pub mod smtp;
pub mod templates;

use crate::errors::email_service_error::EmailServiceError;

pub use smtp::SmtpEmailSender;

#[derive(Clone, Debug)]
pub struct EmailContext {
    pub from_address: String,
    pub from_name: String,
    pub to_address: String,
    pub to_name: String,
    pub subject: String,
    pub html_body: String,
}

#[derive(Clone, Debug)]
pub struct SenderIdentity {
    pub address: String,
    pub name: String,
}

pub trait EmailSender: Send + Sync + Clone {
    fn send_email(
        &self,
        message: EmailContext,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;
}
