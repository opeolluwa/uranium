use axum::Json;
use axum::extract::State;

use crate::services::user_service::UserServiceTrait;
use crate::{
    adapters::user::UserDto, errors::user_service_error::UserServiceError,
    middlewares::auth::AuthenticatedUser, services::user_service::UserService,
};

pub async fn retrieve_information(
    State(user_service): State<UserService>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<Json<UserDto>, UserServiceError> {
    let user_data = user_service.retrieve_information(claims.identifier).await?;
    Ok(Json(user_data))
}
