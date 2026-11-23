use chrono::Utc;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::errors::otp_service_error::OtpServiceError;
use crate::repositories::otp_repository::OtpRepository;

const OTP_LENGTH: usize = 6;
const MAX_ATTEMPTS: i32 = 5;
const OTP_EXPIRY_MINUTES: i64 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtpPurpose {
    AccountVerification,
    TwoFactor,
}

impl OtpPurpose {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AccountVerification => "account_verification",
            Self::TwoFactor => "two_factor",
        }
    }
}

#[derive(Clone)]
pub struct OtpService {
    repo: OtpRepository,
}

impl OtpService {
    pub fn init(db: &DatabaseConnection) -> Self {
        Self {
            repo: OtpRepository::init(db),
        }
    }

    pub async fn generate_otp(
        &self,
        owner_id: &Uuid,
        purpose: OtpPurpose,
    ) -> Result<String, OtpServiceError> {
        self.repo
            .revoke_all_for_owner(owner_id, purpose.as_str())
            .await?;

        let code = generate_otp_code();
        let hashed = bcrypt::hash(&code, bcrypt::DEFAULT_COST)?;
        let expires_at = Utc::now() + chrono::Duration::minutes(OTP_EXPIRY_MINUTES);

        self.repo
            .create_otp(*owner_id, purpose.as_str(), &hashed, expires_at)
            .await?;

        Ok(code)
    }

    pub async fn validate_otp(
        &self,
        owner_id: &Uuid,
        purpose: OtpPurpose,
        submitted_code: &str,
    ) -> Result<(), OtpServiceError> {
        let otp = self
            .repo
            .find_active_otp(owner_id, purpose.as_str())
            .await?
            .ok_or(OtpServiceError::NotFound)?;

        if Utc::now() > otp.expires_at.with_timezone(&Utc) {
            self.repo.revoke_otp(otp).await?;
            return Err(OtpServiceError::Expired);
        }

        if otp.attempts >= MAX_ATTEMPTS {
            self.repo.revoke_otp(otp).await?;
            return Err(OtpServiceError::MaxAttemptsExceeded);
        }

        let is_valid = bcrypt::verify(submitted_code, &otp.code)?;
        if !is_valid {
            let new_attempts = self.repo.increment_attempts(otp.clone()).await?;
            if new_attempts >= MAX_ATTEMPTS {
                self.repo.revoke_otp(otp).await?;
            }
            return Err(OtpServiceError::InvalidCode);
        }

        self.repo.revoke_otp(otp).await?;
        Ok(())
    }
}

fn generate_otp_code() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    (0..OTP_LENGTH)
        .map(|_| char::from(b'0' + rng.random_range(0..10)))
        .collect()
}
