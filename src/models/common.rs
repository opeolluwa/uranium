use serde::{Deserialize, Serialize};
use validator::Validate;

//TOD: add validation
///one time password
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OneTimePassword {
    /// the token itself
    pub token: String,
}

/// user email for requesting account verification
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmailVerification {
    pub email: String,
}
