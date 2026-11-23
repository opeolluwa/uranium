use std::time::Duration;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header},
    transport::smtp::{AsyncSmtpTransportBuilder, authentication::Credentials},
};

use super::{EmailContext, EmailSender};
use crate::config::env::SmtpTlsMode;
use crate::errors::email_service_error::EmailServiceError;

pub const DEFAULT_SEND_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Clone)]
pub struct SmtpEmailSender {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
}

fn transport_builder(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    tls_mode: SmtpTlsMode,
    timeout: Option<Duration>,
) -> Result<AsyncSmtpTransportBuilder, EmailServiceError> {
    let mut builder = match tls_mode {
        SmtpTlsMode::None => {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host).port(port)
        }
        SmtpTlsMode::Starttls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(host)
            .map_err(|e| EmailServiceError::ProviderError(e.to_string()))?
            .port(port),
        SmtpTlsMode::Opportunistic => {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(host).port(port)
        }
        SmtpTlsMode::Implicit => AsyncSmtpTransport::<Tokio1Executor>::relay(host)
            .map_err(|e| EmailServiceError::ProviderError(e.to_string()))?
            .port(port),
    };

    if !username.trim().is_empty() {
        builder = builder.credentials(Credentials::new(username.to_string(), password.to_string()));
    }

    if let Some(timeout) = timeout {
        builder = builder.timeout(Some(timeout));
    }

    Ok(builder)
}

impl SmtpEmailSender {
    pub fn new(
        host: &str,
        port: u16,
        username: &str,
        password: &str,
        tls_mode: SmtpTlsMode,
    ) -> Result<Self, EmailServiceError> {
        let builder =
            transport_builder(host, port, username, password, tls_mode, Some(DEFAULT_SEND_TIMEOUT))?;
        Ok(Self {
            mailer: builder.build(),
        })
    }

    pub async fn test_connection(&self) -> Result<bool, EmailServiceError> {
        self.mailer
            .test_connection()
            .await
            .map_err(|err| EmailServiceError::ProviderError(err.to_string()))
    }
}

impl EmailSender for SmtpEmailSender {
    async fn send_email(&self, message: EmailContext) -> Result<(), EmailServiceError> {
        let from = Mailbox::new(
            Some(message.from_name),
            message.from_address.parse().map_err(|e| {
                EmailServiceError::OperationFailed(format!("invalid from address: {e}"))
            })?,
        );

        let to = Mailbox::new(
            Some(message.to_name),
            message.to_address.parse().map_err(|e| {
                EmailServiceError::OperationFailed(format!("invalid to address: {e}"))
            })?,
        );

        let html_part = MultiPart::alternative().singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(message.html_body),
        );

        let email = Message::builder()
            .from(from)
            .to(to)
            .subject(message.subject)
            .multipart(html_part)
            .map_err(|e| EmailServiceError::OperationFailed(e.to_string()))?;

        self.mailer
            .send(email)
            .await
            .map_err(|err| EmailServiceError::DeliveryError(err.to_string()))?;

        Ok(())
    }
}
