use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::services::token_service::{TokenClaims, TokenService, TokenType};
use crate::state::ServicesState;

async fn extract_and_validate(
    parts: &mut Parts,
    token_service: &TokenService,
    allowed_types: &[TokenType],
) -> Result<(TokenClaims, String), AuthenticationServiceError> {
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthenticationServiceError::MissingCredentials)?;

    let raw_token = bearer.token().to_string();
    let claims = token_service.validate_token(&raw_token, allowed_types)?;

    if token_service.is_blacklisted(&raw_token).await? {
        return Err(AuthenticationServiceError::TokenBlacklisted);
    }

    Ok((claims, raw_token))
}

pub struct AuthenticatedUser(pub TokenClaims);

impl FromRequestParts<ServicesState> for AuthenticatedUser {
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServicesState,
    ) -> Result<Self, Self::Rejection> {
        let (claims, _) =
            extract_and_validate(parts, &state.token_service, &[TokenType::Access]).await?;
        Ok(Self(claims))
    }
}

pub struct ChallengeToken(pub TokenClaims);

impl FromRequestParts<ServicesState> for ChallengeToken {
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServicesState,
    ) -> Result<Self, Self::Rejection> {
        let (claims, _) = extract_and_validate(
            parts,
            &state.token_service,
            &[
                TokenType::Access,
                TokenType::TwoFactor,
                TokenType::PasswordReset,
            ],
        )
        .await?;
        Ok(Self(claims))
    }
}

pub struct RawToken(pub TokenClaims, pub String);

impl FromRequestParts<ServicesState> for RawToken {
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ServicesState,
    ) -> Result<Self, Self::Rejection> {
        let (claims, raw) =
            extract_and_validate(parts, &state.token_service, &[TokenType::Access]).await?;
        Ok(Self(claims, raw))
    }
}
