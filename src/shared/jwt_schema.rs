use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSchema {
    // pub id: String,
    pub email: String,
    pub firstname: String,
}
