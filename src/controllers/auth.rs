use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::{
    adapters::{
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, SetNewPasswordRequest,
        },
        response::auth::{CreateUserResponse, VerifyAccountResponse},
    },
    errors::{auth_service_error::AuthenticationServiceError, shared_service_error::ServiceError},
    services::auth_service::{AuthenticationService, AuthenticationServiceTrait},
};

pub async fn create_account(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<Value>, AuthenticationServiceError> {
    let _ = auth_service.create_account(&request).await?;
    Ok(Json(json!({
        "message":"account created successfully"
    })))
}
pub async fn login(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<LoginRequest>,
) {
}
pub async fn verify_account(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<VerifyAccountResponse>,
) {
}
pub async fn forgotten_password(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<ForgottenPasswordRequest>,
) {
}
pub async fn set_new_password(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<SetNewPasswordRequest>,
) {
}
