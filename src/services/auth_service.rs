use sqlx::{Pool, Postgres};

use crate::{
    adapters::{
        dto::otp::OtpKind,
        requests::auth::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshOtpRequest,
            SetNewPasswordRequest, VerifyAccountRequest,
        },
        response::auth::{
            ForgottenPasswordResponse, LoginResponse, RefreshOtpResponse,
            SetNewPasswordResponse, VerifyAccountResponse,
        },
    },
    errors::{
        auth_service_error::AuthenticationServiceError, user_service_error::UserServiceError,
    },
    repositories::user_repository::{UserRepository, UserRepositoryTrait},
    services::user_helper_service::{UserHelperService, UserHelperServiceTrait},
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
    user_helper_service: UserHelperService,
}

impl AuthenticationService {
    pub fn init(pool: &Pool<Postgres>) -> Self {
        Self {
            user_repository: UserRepository::init(pool),
            user_helper_service: UserHelperService::init(),
        }
    }
}
pub trait AuthenticationServiceTrait {
    async fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<(), AuthenticationServiceError>;

    async fn login(request: &LoginRequest) -> Result<LoginResponse, AuthenticationServiceError>;

    async fn forgotten_password(
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, AuthenticationServiceError>;

    async fn set_new_password(
        request: &SetNewPasswordRequest,
    ) -> Result<SetNewPasswordResponse, AuthenticationServiceError>;

    async fn verify_account(
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, AuthenticationServiceError>;

    async fn refresh_otp(
        otp_kind: &OtpKind,
        request: &RefreshOtpRequest,
    ) -> Result<RefreshOtpResponse, AuthenticationServiceError>;
}

impl AuthenticationServiceTrait for AuthenticationService {
    async fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> Result<(), AuthenticationServiceError> {
        if self
            .user_repository
            .find_by_email(&request.email)
            .await
            .is_some()
        {
            return Err(AuthenticationServiceError::from(
                UserServiceError::ConflictError("User with the email already exists".to_string()),
            ));
        }

        let password_hash = self.user_helper_service.hash_password(&request.password)?;
        let user = CreateUserRequest {
            password: password_hash,
            first_name: request.first_name.to_owned(),
            email: request.email.to_owned(),
            last_name: request.last_name.to_owned(),
        };

        self.user_repository
            .create_user(user)
            .await
            .map_err(AuthenticationServiceError::from)
    }

    async fn login(request: &LoginRequest) -> Result<LoginResponse, AuthenticationServiceError> {
        todo!()
    }

    async fn forgotten_password(
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, AuthenticationServiceError> {
        todo!()
    }

    async fn set_new_password(
        request: &SetNewPasswordRequest,
    ) -> Result<SetNewPasswordResponse, AuthenticationServiceError> {
        todo!()
    }

    async fn verify_account(
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, AuthenticationServiceError> {
        todo!()
    }

    async fn refresh_otp(
        otp_kind: &OtpKind,
        request: &RefreshOtpRequest,
    ) -> Result<RefreshOtpResponse, AuthenticationServiceError> {
        todo!()
    }
}
