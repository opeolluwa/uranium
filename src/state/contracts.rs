use secrecy::ExposeSecret;

use crate::config::env::AppConfig;
use crate::errors::app_error::AppError;
use crate::services::mailer::smtp::SmtpEmailSender;
use crate::services::mailer::SenderIdentity;

#[derive(Clone)]
pub struct Contracts {
    pub email: SmtpEmailSender,
    pub sender_identity: SenderIdentity,
}

impl Contracts {
    pub fn new(app_config: &AppConfig) -> Result<Self, AppError> {
        let email = SmtpEmailSender::new(
            app_config.smtp_host.expose_secret(),
            app_config.smtp_port,
            app_config.smtp_username.expose_secret(),
            app_config.smtp_password.expose_secret(),
            app_config.smtp_tls_mode,
        )
        .map_err(|e| AppError::StartupError(e.to_string()))?;

        let sender_identity = SenderIdentity {
            address: app_config.email_from_address.clone(),
            name: app_config.email_from_name.clone(),
        };

        Ok(Self {
            email,
            sender_identity,
        })
    }
}
