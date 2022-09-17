use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSchema {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub username: String,
}

///Define jwt payload
/// the payload will have a token and a type
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub token: String,
    //  #[derive(serde(rename="type"))]
    pub token_type: String,
}
