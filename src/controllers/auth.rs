use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

use crate::adapters::auth_request::{
    CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshTokenRequest,
    SetNewPasswordRequest, VerifyAccountRequest, VerifyOtpRequest,
};
use crate::adapters::auth_response::{
    ForgottenPasswordResult, LoginResponse, LoginResult, RefreshTokenResult, VerifyAccountResult,
};
use crate::adapters::message::Message;
use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::middlewares::auth::{AuthenticatedUser, ChallengeToken, RawToken};
use crate::middlewares::validator::ValidatedRequest;
use crate::services::auth_service::AuthenticationService;

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
    ChallengeToken(claims): ChallengeToken,
    ValidatedRequest(request): ValidatedRequest<VerifyAccountRequest>,
) -> Result<Json<VerifyAccountResult>, AuthenticationServiceError> {
    let result = auth_service.verify_account(&claims, &request).await?;
    Ok(Json(result))
}

pub async fn forgotten_password(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<ForgottenPasswordRequest>,
) -> Result<Json<ForgottenPasswordResult>, AuthenticationServiceError> {
    let result = auth_service.forgotten_password(&request).await?;
    Ok(Json(result))
}

pub async fn set_new_password(
    State(auth_service): State<AuthenticationService>,
    RawToken(claims, raw): RawToken,
    ValidatedRequest(request): ValidatedRequest<SetNewPasswordRequest>,
) -> Result<Json<Message>, AuthenticationServiceError> {
    auth_service
        .set_new_password(&request, &claims, &raw)
        .await?;
    Ok(Json(Message::new("Password updated successfully")))
}

pub async fn request_refresh_token(
    State(auth_service): State<AuthenticationService>,
    ValidatedRequest(request): ValidatedRequest<RefreshTokenRequest>,
) -> Result<Json<RefreshTokenResult>, AuthenticationServiceError> {
    let result = auth_service.request_refresh_token(&request).await?;
    Ok(Json(result))
}

pub async fn logout(
    State(auth_service): State<AuthenticationService>,
    RawToken(claims, raw): RawToken,
) -> Result<Json<Message>, AuthenticationServiceError> {
    auth_service.logout(&claims, &raw).await?;
    Ok(Json(Message::new("Logged out successfully")))
}

pub async fn verify_otp(
    State(auth_service): State<AuthenticationService>,
    ChallengeToken(claims): ChallengeToken,
    ValidatedRequest(request): ValidatedRequest<VerifyOtpRequest>,
) -> Result<Json<LoginResult>, AuthenticationServiceError> {
    let result = auth_service.verify_otp(&claims, &request).await?;
    Ok(Json(result))
}

pub async fn request_new_otp(
    State(auth_service): State<AuthenticationService>,
    AuthenticatedUser(claims): AuthenticatedUser,
) -> Result<Json<Message>, AuthenticationServiceError> {
    auth_service.request_otp(&claims).await?;
    Ok(Json(Message::new("OTP sent successfully")))
}
