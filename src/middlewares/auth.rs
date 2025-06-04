use axum::extract::FromRequestParts;

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let request_headers = &parts.headers;
        let provider_identifier = extract_header(request_headers, PROVIDER_AUTHORIZATION_HEADER)
            .map(|result| AuthenticatedUser {
                provider_identifier: result,
            })?;

        Ok(provider_identifier)
    }
}





fn extract_header(request_headers: &HeaderMap, header: &str) -> Result<String, AuthError> {
    request_headers
        .get(header)
        .ok_or(AuthError::InvalidOrMissingAuthorizationHeader)?
        .to_str()
        .map(String::from)
        .map_err(|_| AuthError::InvalidOrMissingAuthorizationHeader)
}
