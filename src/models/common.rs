use serde::{Deserialize, Serialize};
use validator::Validate;

///one time password
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct OneTimePassword {
    /// the token itself
    pub token: String,
}
