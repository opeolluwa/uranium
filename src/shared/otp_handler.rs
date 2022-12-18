use once_cell::sync::Lazy;
use otp_rs::TOTP;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

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
const CURRENT_TIMESTAMP: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

///generate otp
pub fn generate_otp() -> u32 {
    // Generate code with period and current timestamp
    let generated_otp = OTP.generate(OTP_VALIDITY, *CURRENT_TIMESTAMP).unwrap();
    println!("the generated OPT is :{}", &generated_otp);
    generated_otp
}

/// validate otp
/// accept the otp as function params,
/// verify the
pub fn validate_otp(otp: &u32) -> bool {
    OTP.verify(*otp, OTP_VALIDITY, *CURRENT_TIMESTAMP)
}
