use once_cell::sync::Lazy;
use otp_rs::TOTP;
use racoon_macros::debug_print::debug_print;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use validator::Validate;

static OTP: Lazy<TOTP> = Lazy::new(|| -> TOTP {
    TOTP::new(
        &env::var("SECRET").unwrap_or_else(|_| {
            String::from(
                "zlsAAnVChDQJEZDW9pAq7ks98gjolpfASBHAi8BJ3Y9TeUDHcX9HovV5BzrS4hUKX5tBmB4acfQ",
            )
        }), /* .expect("TOPT secret missing") */
    )
});
/// set otp validity period to 5 minutes
const OTP_VALIDITY: u64 = 60 * 5; // 5 minutes
                                  //get the current time stamp
static CURRENT_TIMESTAMP: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

///generate otp
pub fn _generate_otp() -> u32 {
    // Generate code with period and current timestamp
    let generated_otp = OTP.generate(OTP_VALIDITY, *CURRENT_TIMESTAMP).unwrap();
    debug_print("the generated OPT is", generated_otp);
    generated_otp
}

/// validate otp
/// accept the otp as function params,
/// verify the
pub fn validate_otp(otp: &str) -> bool {
    let otp = otp.trim().parse::<u32>().unwrap();
    OTP.verify(otp, OTP_VALIDITY, *CURRENT_TIMESTAMP)
}

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
    pub fn new(otp_validity_period: u8) -> Self {
        let id = Uuid::new_v4();
        let token = OTP
            .generate((otp_validity_period * 60) as u64, *CURRENT_TIMESTAMP)
            .unwrap();
        Self {
            id,
            token: token.to_string(),
            ..Default::default()
        }
    }

    /// save a newly created OTP to the database
    pub async fn save(&self, db_connection: &Pool<Postgres>) -> Self {
        let sql_query = r#"
       INSERT INTO one_time_password (id, token)
       VALUES ($1, $2) RETURNING *
       "#;
        let otp = sqlx::query_as::<_, Self>(sql_query)
            .bind(&self.id)
            .bind(&self.token)
            .fetch_one(db_connection)
            .await
            .ok();
        Self { ..otp.unwrap() }
    }

    /// link a newly created otp to a user using the user Id
    pub async fn link_to_user(&self, user_id: Uuid, db_connection: &Pool<Postgres>) -> Self {
        let sql_query = r#"
       INSERT INTO user_information (otp_id)
       VALUES ($1) RETURNING *
       "#;
        let otp = sqlx::query_as::<_, Self>(sql_query)
            .bind(Uuid::from(user_id))
            .fetch_one(db_connection)
            .await
            .ok();
        Self { ..otp.unwrap() }
    }
}
