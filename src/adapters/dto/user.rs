use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserDto {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
}
