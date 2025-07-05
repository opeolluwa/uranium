use crate::adapters::requests::auth::VerifyAccountRequest;
use crate::adapters::response::api_response::ApiResponseBuilder;
use crate::middlewares::validator::ValidatedRequest;
use crate::{
    adapters::{
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, SetNewPasswordRequest,
        },
        response::{
            api_response::ApiResponse,
            auth::{CreateUserResponse, LoginResponse, VerifyAccountResponse},
        },
    },
    errors::auth_service_error::AuthenticationServiceError,
    services::auth_service::{AuthenticationService, AuthenticationServiceTrait},
};
use axum::http::StatusCode;
use axum::{extract::State, Json};

pub async fn create_account(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<ApiResponse<CreateUserResponse>, AuthenticationServiceError> {
    auth_service.create_account(&request).await?;
    
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::CREATED)
        .message("Account created successfully")
        .build())
}
pub async fn login(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<ApiResponse<LoginResponse>, AuthenticationServiceError> {
    let login_response = auth_service.login(&request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(login_response)
        .build())
}
pub async fn verify_account(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<VerifyAccountRequest>,
) -> Result<ApiResponse<VerifyAccountResponse>, AuthenticationServiceError> {
    let verify_account_response = auth_service.verify_account(&request).await?;
    Ok(ApiResponseBuilder::new()
        .status_code(StatusCode::OK)
        .data(verify_account_response)
        .build())
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
