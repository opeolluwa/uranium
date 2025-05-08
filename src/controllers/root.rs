use crate::adapters::response::api_response::ApiResponse;
use crate::services::root_service::RootServiceTrait;
use crate::{
    adapters::response::api_response::HandlerResponse, errors::app_error::AppError,
    services::root_service::RootService,
};
use axum::Json;
use axum::extract::State;

// #[axum::debug_handler]
pub async fn shut_down(
    State(root_service): State<RootService>,
) -> Result<HandlerResponse<()>, AppError> {
    root_service.shut_down()?;
    Ok(Json(ApiResponse::from("command received", None)))
}

pub async fn health_check(
    State(root_service): State<RootService>,
) -> Result<HandlerResponse<()>, AppError> {
    root_service.health_check()?;
    Ok(Json(ApiResponse::from("service is healthy", None)))
}
