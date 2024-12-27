use std::str::FromStr;

use tonic::{metadata::MetadataValue, Request};

use crate::jwt;

pub fn check_and_validate_jwt(
    mut request: tonic::Request<()>,
) -> Result<Request<()>, tonic::Status> {
    let authorization_header = request.metadata().get("authorization");

    match authorization_header {
        Some(value) => {
            let value = value.to_str().map_err(|_| {
                tonic::Status::unauthenticated("invalid or missing authorization header")
            });

            if !value.clone()?.starts_with("Bearer") {
                return Err(tonic::Status::unauthenticated(
                    "Invalid authorization header",
                ));
            };

            let token = value?
                .split_ascii_whitespace()
                .into_iter()
                .collect::<Vec<_>>()[1];

            let user_information = jwt::JwtClaims::parse_token(token.into())
                .map_err(|err| tonic::Status::unauthenticated(err.to_string()))?;

            let user_id = MetadataValue::from_str(&user_information.user_id).map_err(|_| {
                tonic::Status::internal(
                    "An unexpected error occurred while validating authorization  header",
                )
            })?;
            let user_email =
                MetadataValue::from_str(&user_information.user_email).map_err(|_| {
                    tonic::Status::internal(
                        "An unexpected error occurred while validating authorization  header",
                    )
                })?;
            // append the user information to the request head
            request.metadata_mut().append("user_id", user_id);
            request.metadata_mut().append("user_email", user_email);

            Ok(request)
        }
        None => Err(tonic::Status::unauthenticated(
            "Invalid or missing authorization header",
        )),
    }
}
