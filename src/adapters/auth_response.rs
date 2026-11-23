use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
    pub two_factor_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginChallenge {
    pub token: String,
    pub expires_at: i64,
    pub two_factor_enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum LoginResponse {
    Success(LoginResult),
    TwoFactorRequired(LoginChallenge),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForgottenPasswordResult {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetNewPasswordResult {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerifyAccountResult {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResult {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expiry: i64,
    pub refresh_token_expiry: i64,
}
