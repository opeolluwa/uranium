use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

///defines fields in the JWT encryption and decryption payload
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub exp: usize,
}

//implement Display for JwtClaims to allow easy debugging
impl Display for JwtClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\nemail: {}\nfullname: {}\nexp:{}",
            self.id, self.email, self.fullname, self.exp
        )
    }
}



///define JWT encryption and decryption secretes
pub struct JwtEncryptionKeys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl JwtEncryptionKeys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
///Define jwt payload structure
/// the payload will have a token and a type
/// the structure will be used as the basis of sending out JTW from the server
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JwtPayload {
    pub token: String,
    pub token_type: String,
}
