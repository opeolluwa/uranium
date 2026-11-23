use std::{str::FromStr, time::Duration};

use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::{errors::app_error::AppError, utils::extract_env::extract_env};

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "default_environment")]
    pub environment: Environment,

    #[serde(default = "default_body_limit_megabytes")]
    pub body_limit_megabytes: usize,

    #[serde(default = "default_allowed_origins")]
    pub allowed_origins: Vec<String>,

    // Email (SMTP)
    pub email_from_address: String,
    pub email_from_name: String,
    pub smtp_host: SecretString,
    pub smtp_port: u16,
    pub smtp_username: SecretString,
    pub smtp_password: SecretString,
    pub smtp_tls_mode: SmtpTlsMode,

    // Database
    pub database_url: SecretString,
    pub max_db_connections: u32,

    #[serde(default = "default_requests_time_out")]
    pub requests_time_out_secs: Duration,

    // Rate limiting
    #[serde(default = "default_rate_limit_requests_per_second")]
    pub rate_limit_requests_per_second: u32,

    #[serde(default = "default_rate_limit_burst")]
    pub rate_limit_burst: u32,

    #[serde(default = "default_rate_limit_requests_per_minute")]
    pub rate_limit_requests_per_minute: u32,

    #[serde(default = "default_rate_limit_max_keys")]
    pub rate_limit_max_keys: usize,

    #[serde(default = "default_rate_limit_auth_per_minute")]
    pub rate_limit_auth_per_minute: u32,

    #[serde(default = "default_rate_limit_auth_per_hour")]
    pub rate_limit_auth_per_hour: u32,

    #[serde(default = "default_rate_limit_otp_per_minute")]
    pub rate_limit_otp_per_minute: u32,

    #[serde(default = "default_rate_limit_otp_per_hour")]
    pub rate_limit_otp_per_hour: u32,

    // JWT
    pub jwt_key: SecretString,

    // Frontend
    #[serde(default = "default_frontend_url")]
    pub frontend_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        tracing::info!("Loading application configuration...");

        let database_url = extract_env::<String>("DATABASE_URL")?;
        let database_url = database_url.into_boxed_str();

        let requests_time_out = extract_env::<u64>("REQUESTS_TIME_OUT_SECS")
            .unwrap_or_else(|_| default_requests_time_out().as_secs());

        let environment =
            extract_env::<Environment>("ENVIRONMENT").unwrap_or_else(|_| default_environment());

        let smtp_tls_mode =
            extract_env("SMTP_TLS_MODE").unwrap_or_else(|_| default_smtp_tls_mode());

        let smtp_host = extract_optional_env::<String>("SMTP_HOST").unwrap_or_default();
        let smtp_username =
            extract_optional_env::<String>("SMTP_USERNAME").unwrap_or_default();
        let smtp_password =
            extract_optional_env::<String>("SMTP_PASSWORD").unwrap_or_default();

        if smtp_host.trim().is_empty() {
            return Err(AppError::EnvError(
                "SMTP_HOST is required".to_string(),
            ));
        }

        if environment == Environment::Production && smtp_tls_mode == SmtpTlsMode::None {
            return Err(AppError::EnvError(
                "SMTP_TLS_MODE=none is not allowed when ENVIRONMENT=production".to_string(),
            ));
        }

        let jwt_key = extract_env::<String>("JWT_SIGNING_KEY")?;

        if environment == Environment::Production && jwt_key.trim().len() < 32 {
            return Err(AppError::EnvError(
                "JWT_SIGNING_KEY must be at least 32 characters in production".to_string(),
            ));
        }

        Ok(Self {
            port: extract_env("PORT").unwrap_or_else(|_| default_port()),
            environment,
            body_limit_megabytes: extract_env("BODY_LIMIT_MEGA_BYTES")
                .unwrap_or_else(|_| default_body_limit_megabytes()),

            allowed_origins: extract_env::<String>("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string())
                .split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(ToOwned::to_owned)
                .collect(),

            email_from_address: extract_env("EMAIL_FROM_ADDRESS")
                .unwrap_or_else(|_| "accounts@uranium.dev".to_string()),
            email_from_name: extract_env("EMAIL_FROM_NAME")
                .unwrap_or_else(|_| "Uranium".to_string()),

            smtp_host: SecretString::from(smtp_host),
            smtp_port: extract_env("SMTP_PORT").unwrap_or(1025),
            smtp_username: SecretString::from(smtp_username),
            smtp_password: SecretString::from(smtp_password),
            smtp_tls_mode,

            database_url: SecretString::new(database_url),
            max_db_connections: extract_env("MAX_DB_CONNECTIONS").unwrap_or(10),

            requests_time_out_secs: Duration::from_secs(requests_time_out),

            rate_limit_requests_per_second: extract_env("RATE_LIMIT_RPS")
                .unwrap_or_else(|_| default_rate_limit_requests_per_second()),
            rate_limit_burst: extract_env("RATE_LIMIT_BURST")
                .unwrap_or_else(|_| default_rate_limit_burst()),
            rate_limit_requests_per_minute: extract_env("RATE_LIMIT_PER_MINUTE")
                .unwrap_or_else(|_| default_rate_limit_requests_per_minute()),
            rate_limit_max_keys: extract_env("RATE_LIMIT_MAX_KEYS")
                .unwrap_or_else(|_| default_rate_limit_max_keys()),
            rate_limit_auth_per_minute: extract_env("RATE_LIMIT_AUTH_PER_MINUTE")
                .unwrap_or_else(|_| default_rate_limit_auth_per_minute()),
            rate_limit_auth_per_hour: extract_env("RATE_LIMIT_AUTH_PER_HOUR")
                .unwrap_or_else(|_| default_rate_limit_auth_per_hour()),
            rate_limit_otp_per_minute: extract_env("RATE_LIMIT_OTP_PER_MINUTE")
                .unwrap_or_else(|_| default_rate_limit_otp_per_minute()),
            rate_limit_otp_per_hour: extract_env("RATE_LIMIT_OTP_PER_HOUR")
                .unwrap_or_else(|_| default_rate_limit_otp_per_hour()),

            jwt_key: SecretString::from(jwt_key),

            frontend_url: if environment == Environment::Production {
                let value = extract_env::<String>("FRONTEND_URL")?;
                let parsed = url::Url::parse(&value).map_err(|_| {
                    AppError::EnvError(
                        "FRONTEND_URL must be an absolute URL in production".to_string(),
                    )
                })?;
                if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
                    return Err(AppError::EnvError(
                        "FRONTEND_URL must be an absolute http(s) URL in production".to_string(),
                    ));
                }
                value
            } else {
                extract_env("FRONTEND_URL").unwrap_or_else(|_| default_frontend_url())
            },
        })
    }

    pub fn current_env() -> Environment {
        match std::env::var("ENVIRONMENT")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "development" | "dev" => Environment::Development,
            "production" | "prod" => Environment::Production,
            "test" | "testing" => Environment::Test,
            _ => Environment::Development,
        }
    }

    pub fn is_production(&self) -> bool {
        matches!(self.environment, Environment::Production)
    }

    pub fn is_test(&self) -> bool {
        matches!(self.environment, Environment::Test)
    }

    pub fn is_development(&self) -> bool {
        matches!(self.environment, Environment::Development)
    }
}

pub fn load_config() -> Result<AppConfig, AppError> {
    AppConfig::from_env()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Development,
    Production,
    Test,
}

impl Environment {
    pub fn is_production(&self) -> bool {
        self == &Environment::Production
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "development" | "dev" => Ok(Self::Development),
            "production" | "prod" => Ok(Self::Production),
            "test" | "testing" => Ok(Self::Test),
            _ => Err(format!("Invalid environment: {value}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SmtpTlsMode {
    None,
    #[default]
    Starttls,
    Opportunistic,
    Implicit,
}

impl FromStr for SmtpTlsMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.to_lowercase().as_str() {
            "none" | "plain" | "plaintext" => Ok(Self::None),
            "starttls" | "required" => Ok(Self::Starttls),
            "opportunistic" => Ok(Self::Opportunistic),
            "implicit" | "wrapper" | "tls" => Ok(Self::Implicit),
            _ => Err(format!("Invalid SMTP TLS mode: {value}")),
        }
    }
}

fn default_smtp_tls_mode() -> SmtpTlsMode {
    SmtpTlsMode::default()
}

fn extract_optional_env<T: FromStr>(env_key: &str) -> Option<T> {
    std::env::var(env_key).ok().and_then(|raw| raw.parse().ok())
}

fn default_port() -> u16 {
    5006
}

fn default_environment() -> Environment {
    Environment::Development
}

fn default_body_limit_megabytes() -> usize {
    10
}

fn default_allowed_origins() -> Vec<String> {
    vec![
        "http://localhost:3000".into(),
        "http://localhost:5173".into(),
    ]
}

fn default_requests_time_out() -> Duration {
    Duration::from_secs(10)
}

fn default_rate_limit_requests_per_second() -> u32 {
    5
}

fn default_rate_limit_burst() -> u32 {
    10
}

fn default_rate_limit_requests_per_minute() -> u32 {
    50
}

fn default_rate_limit_max_keys() -> usize {
    100_000
}

fn default_rate_limit_auth_per_minute() -> u32 {
    5
}

fn default_rate_limit_auth_per_hour() -> u32 {
    30
}

fn default_rate_limit_otp_per_minute() -> u32 {
    2
}

fn default_rate_limit_otp_per_hour() -> u32 {
    10
}

fn default_frontend_url() -> String {
    "http://localhost:3000".into()
}
