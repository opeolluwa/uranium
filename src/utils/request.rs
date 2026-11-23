use axum::{RequestPartsExt, extract::Request};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};

use crate::errors::authentication_error::AuthenticationError;

pub async fn extract_jwt_from_header(request: Request) -> Result<String, AuthenticationError> {
    let (mut parts, _) = request.into_parts();
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthenticationError::MissingCredentials)?;

    Ok(bearer.token().to_string())
}
