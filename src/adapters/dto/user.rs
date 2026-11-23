use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub identifier: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
