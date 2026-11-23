use crate::adapters::message::Message;
use crate::services::root_service::RootServiceTrait;
use crate::{errors::app_error::AppError, services::root_service::RootService};
use axum::extract::State;
use axum::Json;

pub async fn health_check(
    State(root_service): State<RootService>,
) -> Result<Json<Message>, AppError> {
    root_service.health_check()?;
    Ok(Json(Message::new("service is healthy")))
}
