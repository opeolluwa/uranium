use serde::{Deserialize, Serialize};

/// define the user data structure
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInformation {
    pub fullname: String,
    pub email: String,
    pub password: String,
    pub username: String,
}

///user authorization information
/// to be used for making access retrieve user information
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserAuthCredentials {
    pub email: String,
    pub password: String,
}
