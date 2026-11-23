use sea_orm::DatabaseConnection;

use crate::adapters::{
    auth_request::{
        CreateUserRequest, ForgottenPasswordRequest, LoginRequest, RefreshTokenRequest,
        SetNewPasswordRequest, VerifyAccountRequest, VerifyOtpRequest,
    },
    auth_response::{
        ForgottenPasswordResult, LoginChallenge, LoginResponse, LoginResult, RefreshTokenResult,
        SetNewPasswordResult, VerifyAccountResult,
    },
};
use crate::entities::users;
use crate::errors::auth_service_error::AuthenticationServiceError;
use crate::errors::user_service_error::UserServiceError;
use crate::repositories::user_repository::{UserRepository, UserRepositoryTrait};
use crate::services::{
    email_dispatch_service::EmailDispatchService,
    mailer::SmtpEmailSender,
    otp_service::{OtpPurpose, OtpService},
    token_service::{TokenClaims, TokenService, TokenType},
    user_helper_service::{UserHelperService, UserHelperServiceTrait},
};

#[derive(Clone)]
pub struct AuthenticationService {
    user_repository: UserRepository,
    user_helper_service: UserHelperService,
    email_sender: SmtpEmailSender,
    token_service: TokenService,
    otp_service: OtpService,
    frontend_url: String,
}

impl AuthenticationService {
    pub fn init(
        db: &DatabaseConnection,
        email_sender: SmtpEmailSender,
        token_service: TokenService,
        otp_service: OtpService,
        frontend_url: String,
    ) -> Self {
        Self {
            user_repository: UserRepository::init(db),
            user_helper_service: UserHelperService::init(),
            email_sender,
            token_service,
            otp_service,
            frontend_url,
        }
    }

    pub async fn create_account(
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
                UserServiceError::ConflictError("User with this email already exists".to_string()),
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

        self.user_repository
            .create_user(user)
            .await
            .map_err(|err| {
                tracing::error!("{}", err.to_string());
                AuthenticationServiceError::from(err)
            })?;

        let sender = self.email_sender.clone();
        tokio::task::spawn(async move {
            if let Err(err) = sender
                .send_welcome_email(&email, &first_name, "/dashboard")
                .await
            {
                tracing::error!("Failed to send welcome email: {}", err);
            }
        });

        Ok(())
    }

    pub async fn login(
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

        if !user.is_active {
            return Err(AuthenticationServiceError::AccountInactive);
        }

        if user.two_factor_enabled {
            return self.handle_two_factor_login(&user).await;
        }

        self.issue_login_tokens(&user).await
    }

    async fn handle_two_factor_login(
        &self,
        user: &users::Model,
    ) -> Result<LoginResponse, AuthenticationServiceError> {
        let two_factor_token = self
            .token_service
            .generate_two_factor_token(&user.email, &user.identifier)?;

        let otp_code = self
            .otp_service
            .generate_otp(&user.identifier, OtpPurpose::TwoFactor)
            .await?;

        let sender = self.email_sender.clone();
        let email = user.email.clone();
        let name = user.first_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = sender.send_otp_email(&email, &name, &otp_code).await {
                tracing::error!("Failed to send OTP email: {}", err);
            }
        });

        Ok(LoginResponse::TwoFactorRequired(LoginChallenge {
            token: two_factor_token.token,
            expires_at: two_factor_token.expires_at,
            two_factor_enabled: true,
        }))
    }

    async fn issue_login_tokens(
        &self,
        user: &users::Model,
    ) -> Result<LoginResponse, AuthenticationServiceError> {
        let access = self
            .token_service
            .generate_access_token(&user.email, &user.identifier)?;
        let refresh = self
            .token_service
            .generate_refresh_token(&user.email, &user.identifier)
            .await?;

        Ok(LoginResponse::Success(LoginResult {
            access_token: access.token,
            refresh_token: refresh.token,
            access_token_expiry: access.expires_at,
            refresh_token_expiry: refresh.expires_at,
            two_factor_enabled: user.two_factor_enabled,
        }))
    }

    pub async fn forgotten_password(
        &self,
        request: &ForgottenPasswordRequest,
    ) -> Result<ForgottenPasswordResult, AuthenticationServiceError> {
        if let Some(user) = self.user_repository.find_by_email(&request.email).await {
            let reset_token = self
                .token_service
                .generate_reset_token(&user.email, &user.identifier)?;

            let reset_link = format!(
                "{}/reset-password?token={}",
                self.frontend_url, reset_token.token
            );

            let sender = self.email_sender.clone();
            let email = user.email.clone();
            let name = user.first_name.clone();
            tokio::task::spawn(async move {
                if let Err(err) = sender
                    .send_password_reset_email(&email, &name, &reset_link)
                    .await
                {
                    tracing::error!("Failed to send password reset email: {}", err);
                }
            });
        }

        Ok(ForgottenPasswordResult {})
    }

    pub async fn set_new_password(
        &self,
        request: &SetNewPasswordRequest,
        claims: &TokenClaims,
        raw_token: &str,
    ) -> Result<SetNewPasswordResult, AuthenticationServiceError> {
        if claims.token_type != TokenType::PasswordReset {
            return Err(AuthenticationServiceError::InvalidTokenType);
        }

        if self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        }

        let new_password = self.user_helper_service.hash_password(&request.password)?;
        self.user_repository
            .update_password(&claims.identifier, &new_password)
            .await?;

        self.token_service
            .blacklist_token(raw_token, claims.exp)
            .await?;
        self.token_service
            .revoke_all_refresh_tokens(&claims.identifier)
            .await?;

        Ok(SetNewPasswordResult {})
    }

    pub async fn verify_account(
        &self,
        claims: &TokenClaims,
        request: &VerifyAccountRequest,
    ) -> Result<VerifyAccountResult, AuthenticationServiceError> {
        if self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
            .is_none()
        {
            return Err(AuthenticationServiceError::InvalidToken);
        }

        self.otp_service
            .validate_otp(
                &claims.identifier,
                OtpPurpose::AccountVerification,
                &request.otp,
            )
            .await?;

        self.user_repository
            .update_account_status(&claims.identifier)
            .await?;

        Ok(VerifyAccountResult {})
    }

    pub async fn verify_otp(
        &self,
        claims: &TokenClaims,
        request: &VerifyOtpRequest,
    ) -> Result<LoginResult, AuthenticationServiceError> {
        if claims.token_type != TokenType::TwoFactor {
            return Err(AuthenticationServiceError::InvalidTokenType);
        }

        let Some(user) = self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
        else {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        if !user.is_active {
            return Err(AuthenticationServiceError::AccountInactive);
        }

        self.otp_service
            .validate_otp(&claims.identifier, OtpPurpose::TwoFactor, &request.otp)
            .await?;

        let access = self
            .token_service
            .generate_access_token(&user.email, &user.identifier)?;
        let refresh = self
            .token_service
            .generate_refresh_token(&user.email, &user.identifier)
            .await?;

        Ok(LoginResult {
            access_token: access.token,
            refresh_token: refresh.token,
            access_token_expiry: access.expires_at,
            refresh_token_expiry: refresh.expires_at,
            two_factor_enabled: user.two_factor_enabled,
        })
    }

    pub async fn request_refresh_token(
        &self,
        request: &RefreshTokenRequest,
    ) -> Result<RefreshTokenResult, AuthenticationServiceError> {
        let claims = self
            .token_service
            .validate_token(&request.refresh_token, &[TokenType::Refresh])?;

        let Some(user) = self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
        else {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        if !user.is_active {
            return Err(AuthenticationServiceError::AccountInactive);
        }

        let consumed = self
            .token_service
            .consume_refresh_token(&request.refresh_token, &claims.identifier)
            .await?;

        if !consumed {
            self.token_service
                .revoke_all_refresh_tokens(&claims.identifier)
                .await?;
            return Err(AuthenticationServiceError::RefreshTokenConsumed);
        }

        let access = self
            .token_service
            .generate_access_token(&user.email, &user.identifier)?;
        let refresh = self
            .token_service
            .generate_refresh_token(&user.email, &user.identifier)
            .await?;

        Ok(RefreshTokenResult {
            access_token: access.token,
            refresh_token: refresh.token,
            access_token_expiry: access.expires_at,
            refresh_token_expiry: refresh.expires_at,
        })
    }

    pub async fn logout(
        &self,
        claims: &TokenClaims,
        raw_token: &str,
    ) -> Result<(), AuthenticationServiceError> {
        self.token_service
            .blacklist_token(raw_token, claims.exp)
            .await?;
        self.token_service
            .revoke_all_refresh_tokens(&claims.identifier)
            .await?;
        Ok(())
    }

    pub async fn request_otp(
        &self,
        claims: &TokenClaims,
    ) -> Result<(), AuthenticationServiceError> {
        let Some(user) = self
            .user_repository
            .find_by_identifier(&claims.identifier)
            .await
        else {
            return Err(AuthenticationServiceError::InvalidToken);
        };

        let otp_code = self
            .otp_service
            .generate_otp(&claims.identifier, OtpPurpose::TwoFactor)
            .await?;

        let sender = self.email_sender.clone();
        let email = user.email.clone();
        let name = user.first_name.clone();
        tokio::task::spawn(async move {
            if let Err(err) = sender.send_otp_email(&email, &name, &otp_code).await {
                tracing::error!("Failed to send OTP email: {}", err);
            }
        });

        Ok(())
    }
}
