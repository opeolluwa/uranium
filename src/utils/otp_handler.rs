use once_cell::sync::Lazy;
use otp_rs::TOTP;
use racoon_macros::debug_print::debug_print;
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
static CURRENT_TIMESTAMP: Lazy<u64> = Lazy::new(|| {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
});

///generate otp
pub fn generate_otp() -> u32 {
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
