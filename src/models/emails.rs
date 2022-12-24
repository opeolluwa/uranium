use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

/// email folders
/// a enum on the possible grouping of emails
/// the email context
#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[sqlx(type_name = "email_folder")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum EmailFolder {
    Inbox,
    Draft,
    Important,
    Trash,
    Custom,
    Sent
}

/// email read status
/// tells us if the mail has been read/opened
#[derive(sqlx::Type, Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[sqlx(type_name = "email_status")] // only for PostgreSQL to match a type definition
#[sqlx(rename_all = "lowercase")]
pub enum EmailStatus {
    ///email has been opened
    Read,
    ///email has not been opened
    Unread,
}
/// derive sqlx::FromRow  trait to make the struct queryable as database model
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct EmailModel {
    ///a uniques identifier for the email item essentially a UUID
    id: Uuid,
    /// the sender fullname
    pub sender_name: String,
    /// the sender or recipient email address
    pub sender_email: String,
    ///the email subject
    pub email_subject: String,
    /// the message content
    pub email_body: String,
    ///email status , values, read or unread, default to unread
    pub email_status: Option<EmailStatus>,
    ///the grouping of the email, default to inbox
    pub email_folder: Option<EmailFolder>,
    /// is the email moved to archive, default to false
    pub is_archived: Option<bool>,
    ///is the email starred, default to false
    pub is_starred: Option<bool>,
    /// the date the email was added or sent, default to current time
    pub date_sent: Option<sqlx::types::chrono::NaiveDateTime>,
}

/// the email context, essentially an HTTP request payload
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
