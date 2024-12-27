use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::config::CONFIG;

// use crate::{config::CONFIG, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub user_email: String,
    pub user_id: String,
    exp: usize,
}

impl JwtClaims {
    pub fn new(user_email: String, user_id: String) -> Self {
        Self {
            user_email,
            user_id,
            exp: 2000000000, // May 2
        }
    }

    /// generate a  new token
    pub fn gen_token(&self) -> Result<std::string::String, jsonwebtoken::errors::Error> {
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(CONFIG.jwt_signing_key.as_bytes()),
        )
    }

    /// get the claim from the token
    pub fn parse_token(token: String) -> anyhow::Result<JwtClaims> {
        match decode::<Self>(
            &token,
            &DecodingKey::from_secret(CONFIG.jwt_signing_key.as_bytes()),
            &Validation::default(),
        ) {
            Ok(claim) => Ok(claim.claims),
            Err(error_message) => {
                Err(error_message.into())
            }
        }
    }
}
