use crate::adapters::auth_request::{
    CreateUserRequest, ForgottenPasswordRequest, LoginRequest, SetNewPasswordRequest,
    VerifyAccountRequest,
};
use crate::adapters::auth_response::{
    ForgottenPasswordResponse, LoginResponse, RefreshTokenResponse, VerifyAccountResponse,
};
use crate::adapters::jwt::Claims;
use crate::adapters::message::Message;
use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::middlewares::validator::ValidatedRequest;
use crate::services::auth_service::{AuthenticationService, AuthenticationServiceTrait};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

pub async fn create_account(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<CreateUserRequest>,
) -> Result<(StatusCode, Json<Message>), AuthenticationServiceError> {
    auth_service.create_account(&request).await?;

    Ok((
        StatusCode::CREATED,
        Json(Message::new("Account created successfully")),
    ))
}
pub async fn login(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<LoginRequest>,
) -> Result<Json<LoginResponse>, AuthenticationServiceError> {
    let login_response = auth_service.login(&request).await?;
    Ok(Json(login_response))
}
pub async fn verify_account(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<VerifyAccountRequest>,
) -> Result<Json<VerifyAccountResponse>, AuthenticationServiceError> {
    let verify_account_response = auth_service.verify_account(&claims, &request).await?;
    Ok(Json(verify_account_response))
}
pub async fn forgotten_password(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<Json<ForgottenPasswordResponse>, AuthenticationServiceError> {
    let forgotten_password_response = auth_service.forgotten_password(&request).await?;
    Ok(Json(forgotten_password_response))
}

pub async fn set_new_password(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
    ValidatedRequest(request): ValidatedRequest<SetNewPasswordRequest>,
) -> Result<Json<Message>, AuthenticationServiceError> {
    let _ = auth_service.set_new_password(&request, &claims).await?;
    Ok(Json(Message::new("password updated successfully")))
}

pub async fn request_refresh_token(
    State(auth_service): State<AuthenticationService>,
    claims: Claims,
) -> Result<Json<RefreshTokenResponse>, AuthenticationServiceError> {
    let refresh_token_response = auth_service.request_refresh_token(&claims).await?;
    Ok(Json(refresh_token_response))
}
