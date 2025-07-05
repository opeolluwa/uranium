use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};

use crate::{
    adapters::dto::jwt::AuthenticatedUser, errors::auth_service_error::AuthenticationServiceError,
    shared::extract_env::extract_env,
};
use crate::adapters::dto::jwt::Keys;

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthenticationServiceError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = extract_env::<String>("JWT_SIGNING_KEY")
            .map_err(AuthenticationServiceError::from)?;

        let decoding_key = Keys::new(secret.as_bytes()).decoding;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthenticationServiceError::InvalidToken)?;
        // Decode the user data
        let token_data =
            decode::<AuthenticatedUser>(bearer.token(), &decoding_key, &Validation::default())
                .map_err(|_| AuthenticationServiceError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// fn extract_header(request_headers: &HeaderMap, header: &str) -> Result<String, AuthenticationServiceError> {
//     request_headers
//         .get(header)
//         .ok_or(AuthenticationServiceError::InvalidOrMissingAuthorizationHeader)?
//         .to_str()
//         .map(String::from)
//         .map_err(|_| AuthenticationServiceError::InvalidOrMissingAuthorizationHeader)
// }
