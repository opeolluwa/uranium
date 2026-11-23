use askama::Template;

use super::mailer::templates::{
    account_confirmation::AccountConfirmationTemplate, otp_code::OtpCodeTemplate,
    password_reset::PasswordResetTemplate, welcome::WelcomeTemplate,
};
use super::mailer::{EmailContext, EmailSender};
use crate::errors::email_service_error::EmailServiceError;
use crate::utils::extract_env::extract_env;

fn sender_identity() -> (String, String) {
    let address = extract_env::<String>("EMAIL_FROM_ADDRESS")
        .unwrap_or_else(|_| "noreply@uranium.dev".to_string());
    let name = extract_env::<String>("EMAIL_FROM_NAME").unwrap_or_else(|_| "Uranium".to_string());
    (address, name)
}

pub trait EmailDispatchService {
    fn send_welcome_email(
        &self,
        to_address: &str,
        to_name: &str,
        dashboard_link: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;

    fn send_password_reset_email(
        &self,
        to_address: &str,
        to_name: &str,
        reset_link: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;

    fn send_otp_email(
        &self,
        to_address: &str,
        to_name: &str,
        otp_code: &str,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;

    fn send_account_confirmation_email(
        &self,
        to_address: &str,
        to_name: &str,
        otp_code: Option<&str>,
        confirmation_link: Option<&str>,
    ) -> impl std::future::Future<Output = Result<(), EmailServiceError>> + Send;
}

impl<E: EmailSender> EmailDispatchService for E {
    async fn send_welcome_email(
        &self,
        to_address: &str,
        to_name: &str,
        dashboard_link: &str,
    ) -> Result<(), EmailServiceError> {
        let html_body = WelcomeTemplate {
            name: to_name,
            dashboard_link,
        }
        .render()
        .map_err(|err| EmailServiceError::OperationFailed(err.to_string()))?;

        let (from_address, from_name) = sender_identity();

        self.send_email(EmailContext {
            from_address,
            from_name,
            to_address: to_address.to_string(),
            to_name: to_name.to_string(),
            subject: "Welcome to Uranium".to_string(),
            html_body,
        })
        .await
    }

    async fn send_password_reset_email(
        &self,
        to_address: &str,
        to_name: &str,
        reset_link: &str,
    ) -> Result<(), EmailServiceError> {
        let html_body = PasswordResetTemplate {
            name: to_name,
            reset_link,
        }
        .render()
        .map_err(|err| EmailServiceError::OperationFailed(err.to_string()))?;

        let (from_address, from_name) = sender_identity();

        self.send_email(EmailContext {
            from_address,
            from_name,
            to_address: to_address.to_string(),
            to_name: to_name.to_string(),
            subject: "Reset your password".to_string(),
            html_body,
        })
        .await
    }

    async fn send_otp_email(
        &self,
        to_address: &str,
        to_name: &str,
        otp_code: &str,
    ) -> Result<(), EmailServiceError> {
        let html_body = OtpCodeTemplate {
            name: to_name,
            code: otp_code,
        }
        .render()
        .map_err(|err| EmailServiceError::OperationFailed(err.to_string()))?;

        let (from_address, from_name) = sender_identity();

        self.send_email(EmailContext {
            from_address,
            from_name,
            to_address: to_address.to_string(),
            to_name: to_name.to_string(),
            subject: "Your verification code".to_string(),
            html_body,
        })
        .await
    }

    async fn send_account_confirmation_email(
        &self,
        to_address: &str,
        to_name: &str,
        otp_code: Option<&str>,
        confirmation_link: Option<&str>,
    ) -> Result<(), EmailServiceError> {
        let html_body = AccountConfirmationTemplate {
            name: to_name,
            code: otp_code,
            confirmation_link,
        }
        .render()
        .map_err(|err| EmailServiceError::OperationFailed(err.to_string()))?;

        let (from_address, from_name) = sender_identity();

        self.send_email(EmailContext {
            from_address,
            from_name,
            to_address: to_address.to_string(),
            to_name: to_name.to_string(),
            subject: "Confirm your account".to_string(),
            html_body,
        })
        .await
    }
}
