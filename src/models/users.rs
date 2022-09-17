use serde::{Deserialize, Serialize};

/// define the user data structure
/// implement debug, serialize, deserializing and #[derive(sqlx::FromRow
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserInformation {
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub username: Option<String>,
}
