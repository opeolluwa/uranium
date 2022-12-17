use serde::{Deserialize, Serialize};

///one time password
#[derive(Debug, Serialize, Deserialize)]
pub struct OneTimePswd {
    /// the token itself
    pub token: u32,
}
