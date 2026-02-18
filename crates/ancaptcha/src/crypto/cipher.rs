//! Authenticated encryption primitives using ChaCha20-Poly1305.

use chacha20poly1305::{
    ChaCha20Poly1305, KeyInit, Nonce,
    aead::{Aead, OsRng, rand_core::RngCore},
};

use crate::common::Secret;
use crate::common::error::{AnCaptchaError, Result};

const NONCE_SIZE: usize = 12;

/// Encrypts plaintext data using ChaCha20-Poly1305.
///
/// # Errors
///
/// Fails if the encryption operation encounters an internal error.
pub fn encrypt(secret: &Secret, plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = ChaCha20Poly1305::new(secret.into());

    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext)?;

    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypts data using ChaCha20-Poly1305.
///
/// # Errors
///
/// Fails if the input data is too short, or if the authentication check fails.
pub fn decrypt(secret: &Secret, data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < NONCE_SIZE {
        return Err(AnCaptchaError::InvalidToken);
    }

    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);

    let cipher = ChaCha20Poly1305::new(secret.into());
    Ok(cipher.decrypt(nonce, ciphertext)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_secret() -> Secret {
        [42u8; 32]
    }

    #[test]
    fn round_trip() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let secret = test_secret();
        let plaintext = b"test data";

        let encrypted = encrypt(&secret, plaintext)?;
        let decrypted = decrypt(&secret, &encrypted)?;

        assert_eq!(plaintext.as_slice(), decrypted.as_slice());
        Ok(())
    }

    #[test]
    fn tampering_detected() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let secret = test_secret();
        let plaintext = b"original data";

        let mut encrypted = encrypt(&secret, plaintext)?;
        if let Some(byte) = encrypted.get_mut(NONCE_SIZE + 1) {
            *byte ^= 0xFF;
        }

        let result = decrypt(&secret, &encrypted);
        assert!(matches!(result, Err(AnCaptchaError::CryptoError(_))));
        Ok(())
    }

    #[test]
    fn invalid_length() {
        let secret = test_secret();
        let short_data = vec![0u8; 5];

        let result = decrypt(&secret, &short_data);
        assert!(matches!(result, Err(AnCaptchaError::InvalidToken)));
    }

    #[test]
    fn wrong_secret() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let secret1 = test_secret();
        let mut secret2 = test_secret();
        secret2[0] = 99;

        let plaintext = b"secret message";
        let encrypted = encrypt(&secret1, plaintext)?;

        let result = decrypt(&secret2, &encrypted);
        assert!(matches!(result, Err(AnCaptchaError::CryptoError(_))));
        Ok(())
    }

    #[test]
    fn encryption_failures() {
        let secret = test_secret();
        assert!(decrypt(&secret, &[]).is_err());
        assert!(decrypt(&secret, &[0u8; 11]).is_err());
    }

    #[test]
    fn error_conversion() {
        let raw_err = chacha20poly1305::aead::Error;
        let conv_err = AnCaptchaError::from(raw_err);
        assert!(matches!(conv_err, AnCaptchaError::CryptoError(_)));
    }

    #[test]
    fn data_size_variants() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let secret = test_secret();
        let sizes = [0, 1, 32, 1024];

        for size in sizes {
            let data = vec![0u8; size];
            let enc = encrypt(&secret, &data)?;
            let dec = decrypt(&secret, &enc)?;
            assert_eq!(data, dec);
        }
        Ok(())
    }
}

#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::TestCaseError;

    proptest! {
        #[test]
        fn round_trip_fuzz(data in proptest::collection::vec(any::<u8>(), 0..1024), secret_data in any::<[u8; 32]>()) {
            let secret = secret_data;
            let encrypted = encrypt(&secret, &data).map_err(|e| TestCaseError::fail(format!("{e:?}")))?;
            let decrypted = decrypt(&secret, &encrypted).map_err(|e| TestCaseError::fail(format!("{e:?}")))?;
            prop_assert_eq!(data, decrypted);
        }

        #[test]
        fn random_decrypt_fuzz(data in proptest::collection::vec(any::<u8>(), 0..2048), secret_data in any::<[u8; 32]>()) {
            let secret = secret_data;
            let result = decrypt(&secret, &data);
            prop_assert!(result.is_err());
        }

        #[test]
        fn tampering_fuzz(data in proptest::collection::vec(any::<u8>(), 1..1024), secret_data in any::<[u8; 32]>(), byte_idx in any::<usize>(), bit_idx in 0..8u8) {
            let secret = secret_data;
            let mut encrypted = encrypt(&secret, &data).map_err(|e| TestCaseError::fail(format!("{e:?}")))?;

            let idx = byte_idx % encrypted.len();
            if let Some(byte) = encrypted.get_mut(idx) {
                *byte ^= 1 << bit_idx;
            }

            let result = decrypt(&secret, &encrypted);
            prop_assert!(result.is_err());
        }
    }
}
