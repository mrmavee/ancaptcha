//! Error types and handling for the anCaptcha library.

use thiserror::Error;

/// Core error type for all captcha operations.
#[derive(Debug, Error)]
pub enum AnCaptchaError {
    #[error("Invalid or expired token")]
    /// AEAD decryption failure.
    CryptoError(chacha20poly1305::aead::Error),

    #[error("Invalid or expired token")]
    /// Token serialization failure.
    SerializationError(#[from] bincode_next::error::EncodeError),

    #[error("Invalid or expired token")]
    /// Token deserialization failure.
    DeserializationError(#[from] bincode_next::error::DecodeError),

    #[error("Invalid or expired token")]
    /// Base64 decoding failure.
    Base64Error(#[from] base64::DecodeError),

    #[error("Invalid or expired token")]
    /// Token is structurally invalid or unrecognized.
    InvalidToken,

    #[error("Invalid or expired token")]
    /// Token timestamp has exceeded the TTL.
    ExpiredToken,

    #[error("Invalid or expired token")]
    /// Token HMAC or ciphertext integrity check failed.
    TamperedToken,

    #[error("Internal system error")]
    /// Unrecoverable internal error.
    Internal(String),
}

impl From<chacha20poly1305::aead::Error> for AnCaptchaError {
    fn from(err: chacha20poly1305::aead::Error) -> Self {
        Self::CryptoError(err)
    }
}

/// Specialized Result type for library operations.
pub type Result<T> = std::result::Result<T, AnCaptchaError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        assert_eq!(
            AnCaptchaError::InvalidToken.to_string(),
            "Invalid or expired token"
        );
        assert_eq!(
            AnCaptchaError::Internal("test".to_string()).to_string(),
            "Internal system error"
        );
    }
}
