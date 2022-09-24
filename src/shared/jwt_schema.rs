use super::api_response::ApiErrorResponse as AuthError;
use axum::async_trait;
use axum::extract::{FromRequestParts, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use axum::http::request::Parts;
use jsonwebtoken::decode;
use jsonwebtoken::Validation;
use jsonwebtoken::{DecodingKey, EncodingKey};
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

#[async_trait]
impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AuthError::InvalidToken {
                    error: "error_message".to_String(),
                })?;
        // Decode the user data
        let token_data = decode::<JwtClaims>(
            bearer.token(),
            &JwtEncryptionKeys::decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken {
            error: "error_message".to_String(),
        })?;
        Ok(token_data.claims)
    }
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
