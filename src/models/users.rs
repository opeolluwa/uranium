use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

/// define the user data structure
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInformation {
    pub id: Uuid,
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub username: String,
    // uuid : Uuid
}

///user authorization information
/// to be used for making access retrieve user information
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthCredentials {
    pub email: String,
    pub password: String,
}
