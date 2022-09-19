use serde::{Deserialize, Serialize};
use uuid::Uuid;

///the email will be stored as a struct having `id` and a nested `context` types
/// the `id ` is essentially a uuid and the context is a json
/// the fields of the context are optional and the fields are `fullname`, `subject` , `email` and `message`
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailSchema {
    /// email id
    pub id: Uuid,
    /// email context will abe an growable array of email-context
    pub context: Vec<EmailContext>,
}

/// the email context
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailContext {
    /// the sender fullname
    pub fullname: String,
    /// the sender or recipient email address
    pub email: String,
    ///the email subject
    pub subject: String,
    /// the message content
    pub message: String,
}
