use axum::extract::State;
use axum::Json;

use crate::{
    adapters::{jwt::Claims, user::UserDto},
    errors::user_service_error::UserServiceError,
    services::user_service::UserService,
};

use crate::services::user_service::UserServiceTrait;
pub async fn retrieve_information(
    State(user_service): State<UserService>,
    claim: Claims,
) -> Result<Json<UserDto>, UserServiceError> {
    let user_data = user_service.retrieve_information(claim.identifier).await?;
    Ok(Json(user_data))
}
