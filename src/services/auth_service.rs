use sqlx::{Pool, Postgres};

use crate::{
    adapters::{
        dto::otp::OtpKind,
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshOtpRequest,
            SetNewPasswordRequest, VerifyAccountRequest,
        },
        response::auth::{
            CreateUserResponse, ForgottenPasswordResponse, LoginResponse, RefreshOtpResponse,
            SetNewPasswordResponse, VerifyAccountResponse,
        },
    },
    errors::service_error::ServiceError,
    repositories::user_repository::UserRepository,
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
}

impl AuthenticationService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
        }
    }
}
pub trait AuthenticationServiceTrait {
    async fn sign_up(request: &CreateUserRequest) -> Result<CreateUserResponse, ServiceError>;

    async fn login(request: &LoginRequest) -> Result<LoginResponse, ServiceError>;

    async fn forgotten_password(
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, ServiceError>;

    async fn set_new_password(
        request: &SetNewPasswordRequest,
    ) -> Result<SetNewPasswordResponse, ServiceError>;

    async fn verify_account(
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError>;

    async fn refresh_otp(
        otp_kind: &OtpKind,
        request: &RefreshOtpRequest,
    ) -> Result<RefreshOtpResponse, ServiceError>;
}

impl AuthenticationServiceTrait for AuthenticationService {
    async fn sign_up(request: &CreateUserRequest) -> Result<CreateUserResponse, ServiceError> {
        todo!()
    }

    async fn login(request: &LoginRequest) -> Result<LoginResponse, ServiceError> {
        todo!()
    }

    async fn forgotten_password(
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, ServiceError> {
        todo!()
    }

    async fn set_new_password(
        request: &SetNewPasswordRequest,
    ) -> Result<SetNewPasswordResponse, ServiceError> {
        todo!()
    }

    async fn verify_account(
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, ServiceError> {
        todo!()
    }

    async fn refresh_otp(
        otp_kind: &OtpKind,
        request: &RefreshOtpRequest,
    ) -> Result<RefreshOtpResponse, ServiceError> {
        todo!()
    }
}
