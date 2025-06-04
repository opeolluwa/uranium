use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};

use crate::{
    adapters::dto::jwt::AuthenticatedUser, errors::auth_error::AuthError,
    shared::extract_env::extract_env,
};

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let secret = extract_env::<String>("JWT_SIGNING_KEY")
            .map_err(|err| AuthError::ServerError(err.to_string()))?;

        let decoding_key = Keys::new(secret.as_bytes()).decoding;
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data =
            decode::<AuthenticatedUser>(bearer.token(), &decoding_key, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

// fn extract_header(request_headers: &HeaderMap, header: &str) -> Result<String, AuthError> {
//     request_headers
//         .get(header)
//         .ok_or(AuthError::InvalidOrMissingAuthorizationHeader)?
//         .to_str()
//         .map(String::from)
//         .map_err(|_| AuthError::InvalidOrMissingAuthorizationHeader)
// }
