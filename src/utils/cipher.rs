use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

use crate::errors::app_error::AppError;
use crate::utils::extract_env::extract_env;

pub fn parse_key(hex_key: &str) -> Result<aes_gcm::aead::Key<Aes256Gcm>, AppError> {
    let key_bytes = hex::decode(hex_key)
        .map_err(|e| AppError::OperationFailed(format!("invalid ENCRYPTION_KEY hex: {e}")))?;
    if key_bytes.len() != 32 {
        return Err(AppError::OperationFailed(format!(
            "ENCRYPTION_KEY must be 32 bytes (64 hex chars), got {}",
            key_bytes.len()
        )));
    }
    let key: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| AppError::OperationFailed("ENCRYPTION_KEY must be exactly 32 bytes".into()))?;
    Ok(key.into())
}

// fn get_key() -> Result<aes_gcm::aead::Key<Aes256Gcm>, AppError> {
//     let hex_key: String = extract_env("ENCRYPTION_KEY")?;
//     parse_key(&hex_key)
// }

pub fn encrypt(plaintext: &str) -> Result<String, AppError> {
    let hex_key: String = extract_env("ENCRYPTION_KEY")?;
    encrypt_with_key(&hex_key, plaintext)
}

pub fn decrypt(encoded: &str) -> Result<String, AppError> {
    let hex_key: String = extract_env("ENCRYPTION_KEY")?;
    decrypt_with_key(&hex_key, encoded)
}

pub fn encrypt_with_key(key_hex: &str, plaintext: &str) -> Result<String, AppError> {
    let key = parse_key(key_hex)?;
    let cipher = Aes256Gcm::new(&key);
    let nonce_bytes: [u8; 12] = rand::random();
    let nonce = Nonce::from(nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| AppError::OperationFailed("encryption failed".into()))?;

    let mut combined = Vec::with_capacity(12 + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(combined))
}

pub fn decrypt_with_key(key_hex: &str, encoded: &str) -> Result<String, AppError> {
    let data = BASE64
        .decode(encoded)
        .map_err(|_| AppError::OperationFailed("invalid base64 ciphertext".into()))?;

    if data.len() < 12 + 16 {
        return Err(AppError::OperationFailed("ciphertext too short".into()));
    }

    let (nonce_bytes, ciphertext) = data.split_at(12);
    let nonce_arr: [u8; 12] = nonce_bytes
        .try_into()
        .map_err(|_| AppError::OperationFailed("invalid nonce length".into()))?;
    let nonce = Nonce::from(nonce_arr);

    let key = parse_key(key_hex)?;
    let cipher = Aes256Gcm::new(&key);

    let plaintext = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| AppError::OperationFailed("decryption failed".into()))?;

    String::from_utf8(plaintext)
        .map_err(|_| AppError::OperationFailed("decrypted data is not valid UTF-8".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn roundtrip() {
        let plaintext = "hello, world!";
        let encrypted = encrypt_with_key(TEST_KEY, plaintext).unwrap();
        let decrypted = decrypt_with_key(TEST_KEY, &encrypted).unwrap();
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn different_ciphertext_each_time() {
        let a = encrypt_with_key(TEST_KEY, "same").unwrap();
        let b = encrypt_with_key(TEST_KEY, "same").unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn wrong_key_fails() {
        let other_key = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
        let encrypted = encrypt_with_key(TEST_KEY, "secret").unwrap();
        assert!(decrypt_with_key(other_key, &encrypted).is_err());
    }

    #[test]
    fn invalid_key_length() {
        assert!(parse_key("0123456789abcdef").is_err());
    }

    #[test]
    fn invalid_hex() {
        assert!(
            parse_key("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz").is_err()
        );
    }

    #[test]
    fn truncated_ciphertext_fails() {
        assert!(decrypt_with_key(TEST_KEY, "dG9vc2hvcnQ=").is_err());
    }
}
