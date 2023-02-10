use crate::utils::jwt::JwtEncryptionKeys;
use once_cell::sync::Lazy;
///fetch the JWT defined environment and assign it's value to a life
/// call on the new method of JwtEncryption keys to accept and pass down the secret to the jsonwebtoken crate EncodingKey and DecodingKey modules
pub static JWT_SECRET: Lazy<JwtEncryptionKeys> = Lazy::new(|| -> JwtEncryptionKeys {
    let secret = std::env::var("JWT_SECRET").expect("Invalid or missing JWT Secret");
    JwtEncryptionKeys::new(secret.as_bytes())
});
