use sea_orm::DatabaseConnection;

use crate::adapters::jwt::{Claims, JwtCredentials, TEN_MINUTES, TWENTY_FIVE_MINUTES};
use crate::entities::users;
use crate::{
    adapters::{
        auth_request::{
            CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshTokenRequest,
            SetNewPasswordRequest, VerifyAccountRequest,
        },
        auth_response::{
            ForgottenPasswordResponse, LoginResponse, RefreshTokenResponse, SetNewPasswordResponse,
            VerifyAccountResponse,
        },
    },
    errors::{
        auth_service_error::AuthenticationServiceError, user_service_error::UserServiceError,
    },
    repositories::user_repository::{UserRepository, UserRepositoryTrait},
    services::{
        mailer::SmtpEmailSender,
        email_dispatch_service::EmailDispatchService,
        user_helper_service::{UserHelperService, UserHelperServiceTrait},
    },
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
    user_helper_service: UserHelperService,
    email_sender: SmtpEmailSender,
}

impl AuthenticationService {
    pub fn init(db: &DatabaseConnection, email_sender: SmtpEmailSender) -> Self {
        Self {
            user_repository: UserRepository::init(db),
            user_helper_service: UserHelperService::init(),
            email_sender,
        }
    }
}
pub trait AuthenticationServiceTrait {
    fn create_account(
        &self,
        request: &CreateUserRequest,
    ) -> impl std::future::Future<Output = Result<(), AuthenticationServiceError>> + Send;

    fn login(
        &self,
        request: &LoginRequest,
    ) -> impl std::future::Future<Output = Result<LoginResponse, AuthenticationServiceError>> + Send;

    fn forgotten_password(
        &self,

        request: &ForgottenPasswordRequest,
    ) -> impl std::future::Future<Output = Result<ForgottenPasswordResponse, AuthenticationServiceError>> + Send;

    fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> impl std::future::Future<Output = Result<SetNewPasswordResponse, AuthenticationServiceError>> + Send;

    fn verify_account(
        &self,
        claims: &Claims,
        request: &VerifyAccountRequest,
    ) -> impl std::future::Future<Output = Result<VerifyAccountResponse, AuthenticationServiceError>> + Send;

    fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> impl std::future::Future<Output = Result<RefreshTokenResponse, AuthenticationServiceError>> + Send;
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

        let email = user.email.clone();
        let first_name = user.first_name.clone();

        self.user_repository.create_user(user).await.map_err(|err| {
            log::error!("{}", err.to_string());
            AuthenticationServiceError::from(err)
        })?;

        let sender = self.email_sender.clone();
        tokio::task::spawn(async move {
            if let Err(err) = sender.send_welcome_email(&email, &first_name, "/dashboard").await {
                tracing::error!("Failed to send welcome email: {}", err);
            }
        });

        Ok(())
    }

    async fn login(
        &self,
        request: &LoginRequest,
    ) -> Result<LoginResponse, AuthenticationServiceError> {
        let Some(user) = self.user_repository.find_by_email(&request.email).await else {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let valid_password = self
            .user_helper_service
            .validate_password(&request.password, &user.password)?;
        if !valid_password {
            return Err(AuthenticationServiceError::WrongCredentials);
        }

        let token =
            JwtCredentials::new(&user.email, &user.identifier).generate_token(TEN_MINUTES)?;

        Ok(LoginResponse { token })
    }

    async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResponse, AuthenticationServiceError> {
        let user = self.user_repository.find_by_email(&request.email).await;
        if user.is_none() {
            return Err(AuthenticationServiceError::WrongCredentials);
        };

        let users::Model {
            email, identifier, first_name, ..
        } = user.unwrap();

        let token = JwtCredentials::new(&email, &identifier).generate_token(TEN_MINUTES)?;

        let sender = self.email_sender.clone();
        let reset_link = format!("/reset-password?token={}", token);
        let user_email = email.clone();
        let user_name = first_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = sender.send_password_reset_email(&user_email, &user_name, &reset_link).await {
                tracing::error!("Failed to send password reset email: {}", err);
            }
        });

        Ok(ForgottenPasswordResponse { token })
    }

    async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &Claims,
    ) -> Result<SetNewPasswordResponse, AuthenticationServiceError> {
        let new_password = self.user_helper_service.hash_password(&request.password)?;

        if self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        self.user_repository
            .update_password(&claims.identifier, &new_password)
            .await?;

        Ok(SetNewPasswordResponse {})
    }

    async fn verify_account(
        &self,
        claims: &Claims,
        _request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResponse, AuthenticationServiceError> {
        if self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        self.user_repository
            .update_account_status(&claims.identifier)
            .await?;
        Ok(VerifyAccountResponse {})
    }

    async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResponse, AuthenticationServiceError> {
        let refresh_token = JwtCredentials::new(&request.email, &request.identifier)
            .generate_token(TWENTY_FIVE_MINUTES)?;

        Ok(RefreshTokenResponse {
            token: refresh_token,
        })
    }
}
