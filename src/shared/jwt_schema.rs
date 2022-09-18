use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSchema {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub exp: usize,
}

///Define jwt payload
/// the payload will have a token and a type
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub token: String,
    pub token_type: String,
}
