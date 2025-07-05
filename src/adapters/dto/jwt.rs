use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::shared::extract_env::extract_env;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const JWT_VALIDITY: Duration = Duration::from_secs(10 * 60 * 60); // 10 minutes in seconds
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub identifier: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtCredentials {
    pub email: String,
    pub identifier: String,
}

pub struct Keys {
    encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    pub(crate) fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Claim {
    pub email: String,
    pub identifier: String,
    pub iat: i64,
    pub exp: i64,
}

impl JwtCredentials {
    pub fn new(email: &str, identifier: &str) -> Self {
        Self {
            email: email.to_string(),
            identifier: identifier.to_string(),
        }
    }

    pub fn generate_token(&self) -> Result<String, AuthenticationServiceError> {
        let now = chrono::Utc::now().timestamp();
        let claim = Claim {
            email: self.email.to_string(),
            identifier: self.identifier.to_string(),
            iat: now,
            exp: now + JWT_VALIDITY.as_secs() as i64,
        };

        let secret =
            extract_env::<String>("JWT_SIGNING_KEY").map_err(AuthenticationServiceError::from)?;

        let encoding_key = Keys::new(secret.as_bytes()).encoding;
        let token = encode(&Header::default(), &claim, &encoding_key)
            .map_err(AuthenticationServiceError::from)?;

        Ok(token)
    }
}
