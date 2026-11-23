use std::fmt;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::config::env::AppConfig;
use crate::errors::auth_service_error::AuthenticationServiceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    Access,
    Refresh,
    PasswordReset,
    TwoFactor,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Access => write!(f, "access"),
            Self::Refresh => write!(f, "refresh"),
            Self::PasswordReset => write!(f, "password_reset"),
            Self::TwoFactor => write!(f, "two_factor"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub email: String,
    pub identifier: Uuid,
    pub token_type: TokenType,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneratedToken {
    pub token: String,
    pub expires_at: i64,
}

#[derive(Clone)]
pub struct TokenService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    access_ttl: u64,
    refresh_ttl: u64,
    reset_ttl: u64,
    two_factor_ttl: u64,
    redis: redis::aio::ConnectionManager,
}

impl TokenService {
    pub fn new(app_config: &AppConfig, redis: redis::aio::ConnectionManager) -> Self {
        use secrecy::ExposeSecret;
        let secret = app_config.jwt_key.expose_secret().as_bytes().to_vec();
        Self {
            encoding_key: EncodingKey::from_secret(&secret),
            decoding_key: DecodingKey::from_secret(&secret),
            access_ttl: app_config.access_token_ttl_secs,
            refresh_ttl: app_config.refresh_token_ttl_secs,
            reset_ttl: app_config.reset_token_ttl_secs,
            two_factor_ttl: app_config.two_factor_token_ttl_secs,
            redis,
        }
    }

    fn ttl_for(&self, token_type: TokenType) -> u64 {
        match token_type {
            TokenType::Access => self.access_ttl,
            TokenType::Refresh => self.refresh_ttl,
            TokenType::PasswordReset => self.reset_ttl,
            TokenType::TwoFactor => self.two_factor_ttl,
        }
    }

    fn generate_token(
        &self,
        email: &str,
        identifier: &Uuid,
        token_type: TokenType,
    ) -> Result<GeneratedToken, AuthenticationServiceError> {
        let now = chrono::Utc::now().timestamp();
        let ttl = self.ttl_for(token_type) as i64;
        let exp = now + ttl;

        let claims = TokenClaims {
            email: email.to_string(),
            identifier: *identifier,
            token_type,
            iat: now,
            exp,
        };

        let token = encode(&Header::default(), &claims, &self.encoding_key)?;

        Ok(GeneratedToken {
            token,
            expires_at: exp,
        })
    }

    pub fn generate_access_token(
        &self,
        email: &str,
        identifier: &Uuid,
    ) -> Result<GeneratedToken, AuthenticationServiceError> {
        self.generate_token(email, identifier, TokenType::Access)
    }

    pub async fn generate_refresh_token(
        &self,
        email: &str,
        identifier: &Uuid,
    ) -> Result<GeneratedToken, AuthenticationServiceError> {
        let generated = self.generate_token(email, identifier, TokenType::Refresh)?;
        self.store_refresh_token(&generated.token, identifier).await?;
        Ok(generated)
    }

    pub fn generate_reset_token(
        &self,
        email: &str,
        identifier: &Uuid,
    ) -> Result<GeneratedToken, AuthenticationServiceError> {
        self.generate_token(email, identifier, TokenType::PasswordReset)
    }

    pub fn generate_two_factor_token(
        &self,
        email: &str,
        identifier: &Uuid,
    ) -> Result<GeneratedToken, AuthenticationServiceError> {
        self.generate_token(email, identifier, TokenType::TwoFactor)
    }

    pub fn validate_token(
        &self,
        raw_token: &str,
        allowed_types: &[TokenType],
    ) -> Result<TokenClaims, AuthenticationServiceError> {
        let token_data = decode::<TokenClaims>(raw_token, &self.decoding_key, &Validation::default())
            .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        let claims = token_data.claims;
        if !allowed_types.contains(&claims.token_type) {
            return Err(AuthenticationServiceError::InvalidTokenType);
        }

        Ok(claims)
    }

    pub async fn blacklist_token(
        &self,
        raw_token: &str,
        exp: i64,
    ) -> Result<(), AuthenticationServiceError> {
        let now = chrono::Utc::now().timestamp();
        let remaining = (exp - now).max(0) as u64;
        if remaining == 0 {
            return Ok(());
        }

        let key = format!("blacklist:{}", token_hash(raw_token));
        let mut conn = self.redis.clone();
        conn.set_ex::<_, _, ()>(&key, "1", remaining)
            .await
            .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;

        Ok(())
    }

    pub async fn is_blacklisted(
        &self,
        raw_token: &str,
    ) -> Result<bool, AuthenticationServiceError> {
        let key = format!("blacklist:{}", token_hash(raw_token));
        let mut conn = self.redis.clone();
        let exists: bool = conn
            .exists(&key)
            .await
            .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;
        Ok(exists)
    }

    async fn store_refresh_token(
        &self,
        raw_token: &str,
        user_id: &Uuid,
    ) -> Result<(), AuthenticationServiceError> {
        let key = format!("refresh:{}:{}", user_id, token_hash(raw_token));
        let mut conn = self.redis.clone();
        conn.set_ex::<_, _, ()>(&key, "1", self.refresh_ttl)
            .await
            .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;
        Ok(())
    }

    pub async fn consume_refresh_token(
        &self,
        raw_token: &str,
        user_id: &Uuid,
    ) -> Result<bool, AuthenticationServiceError> {
        let key = format!("refresh:{}:{}", user_id, token_hash(raw_token));
        let mut conn = self.redis.clone();
        let existed: Option<String> = redis::cmd("GETDEL")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;
        Ok(existed.is_some())
    }

    pub async fn revoke_all_refresh_tokens(
        &self,
        user_id: &Uuid,
    ) -> Result<(), AuthenticationServiceError> {
        let pattern = format!("refresh:{}:*", user_id);
        let mut conn = self.redis.clone();

        let mut cursor: u64 = 0;
        loop {
            let (next_cursor, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await
                .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;

            if !keys.is_empty() {
                conn.del::<_, ()>(&keys)
                    .await
                    .map_err(|e| AuthenticationServiceError::RedisError(e.to_string()))?;
            }

            cursor = next_cursor;
            if cursor == 0 {
                break;
            }
        }

        Ok(())
    }
}

fn token_hash(token: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    let result = hasher.finalize();
    hex::encode(&result[..16])
}
