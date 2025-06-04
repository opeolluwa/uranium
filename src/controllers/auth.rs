use axum::{Json, extract::State};

use crate::{
    adapters::{
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, SetNewPasswordRequest,
        },
        response::auth::VerifyAccountResponse,
    },
    services::auth_service::AuthenticationService,
};

pub async fn sign_up(
    State(auth_service): State<AuthenticationService>,
    Json(request): Json<CreateUserRequest>,
) {
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
