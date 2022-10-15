use super::api_response::ApiErrorResponse as AuthError;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::{authorization::Bearer, Authorization};
use jsonwebtoken::Validation;
use jsonwebtoken::{decode, Algorithm};
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::SystemTime;

///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});
///defines fields in the JWT encryption and decryption payload
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub email: String,
    pub fullname: String,
    pub exp: u64,
}

#[async_trait]
impl<S> FromRequest<S> for JwtClaims
where
    S: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<S>) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|err| AuthError::InvalidToken {
                    message: err.to_string(),
                })?;

        /*
         * Decode the user data
         * the encoding uses a custom algorithm,
         * reconfigure the jsonwebtoken crate to use the custom algorithm that was used for encryption
         *
         * typically, the decryption ought to be
         * Validation::default())
                .map_err(|err| AuthError::InvalidToken {
                    error: err.to_string(),
                })?;

        * how ever we will be using a custom algorithm below
         */
        let validation = Validation::new(Algorithm::HS512);
        let token_data = decode::<JwtClaims>(bearer.token(), &JWT_SECRET.decoding, &validation)
            .map_err(|err| AuthError::InvalidToken {
                message: err.to_string(),
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

/// set the expiration of token
/// accept the exp as the minutes from now ehn the token will be  invalidated 
pub fn set_jtw_exp(exp: u64) -> u64 {
    // the current time
    let now = SystemTime::now();
    // unix epoch elapsed time
    let unix_epoch_elapsed_time = now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("something went wrong");
    //accept the exp, convert it to seconds
    let exp_minutes_to_second = (exp as u64) * 60;
    // return the token expiration as the summation of current unix epoch elapsed time
    let hours_from_now =
        unix_epoch_elapsed_time + std::time::Duration::from_secs(exp_minutes_to_second);

    //return the result as seconds
    hours_from_now.as_secs()
}
