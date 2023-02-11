use once_cell::sync::Lazy;
use otp_rs::TOTP;
use racoon_macros::racoon_error;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use validator::Validate;

// TOTP encryption key
pub static OTP: Lazy<TOTP> = Lazy::new(|| -> TOTP {
    TOTP::new(&env::var("TOTP_ENCRYPTION_KEY").expect("TOTP secret missing"))
});

/// OTP VALIDITY PERIOD SET TO 5 MINUTES
pub const OTP_VALIDITY: u64 = 60 * 5;

//get the current time stamp
static CURRENT_TIMESTAMP: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

///one time password
#[derive(Debug, Serialize, Deserialize, Validate, sqlx::FromRow, Default)]
pub struct Otp {
    pub id: Uuid,
    pub token: String,
    pub is_expired: bool,
}

/// ipl display for Otp
impl std::fmt::Display for Otp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}\ntoken{}is_expired\n{}",
            self.id, self.token, self.is_expired
        )
    }
}
/// helper function for OTP
impl Otp {
    /// create new otp;
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        let token = OTP.generate(OTP_VALIDITY, *CURRENT_TIMESTAMP).unwrap();
        Self {
            id,
            token: token.to_string(),
            ..Default::default()
        }
    }

    /// save a newly created OTP to the database
    pub async fn save(&self, db_connection: &Pool<Postgres>) -> Self {
        let otp = sqlx::query_as::<_, Otp>(
            "INSERT INTO one_time_passwords (id, token)
       VALUES ($1, $2) RETURNING *",
        )
        .bind(&self.id)
        .bind(&self.token)
        .fetch_one(db_connection)
        .await;

        if otp.is_err() {
            racoon_error!("An exception  was encountered while inserting OTP into the database");
            println!("{:?}\n", otp);
        }
        Self { ..otp.unwrap() }
    }

    /// link a newly created otp to a user using the user Id
    pub async fn _link_to_user(&self, user_id: Uuid, db_connection: &Pool<Postgres>) -> Self {
        let otp = sqlx::query_as::<_, Otp>(
            "INSERT INTO user_information (otp_id)
       VALUES ($1) RETURNING *",
        )
        .bind(Uuid::from(user_id))
        .fetch_one(db_connection)
        .await;
        if otp.is_err() {
            racoon_error!("An exception  was encountered while linking user Id to OTP");
            println!("{:?}\n", otp);
        }
        Self { ..otp.unwrap() }
    }

    /// validate otp
    /// accept the otp that was generated  as function params,
    /// verify the
    pub fn validate_otp(otp: &str) -> bool {
        let otp = otp.trim().parse::<u32>().unwrap();
        OTP.verify(otp, OTP_VALIDITY, *CURRENT_TIMESTAMP)
    }
}

/// tests
#[cfg(test)]
mod test {
    use super::Otp;
    #[test]
    fn otp_is_generated() {
        let otp = Otp::new();
        let otp: u32 = otp.token.parse().unwrap();
        assert!(otp > 0)
    }
}
