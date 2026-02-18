//! Encrypted payload structures and expiration logic.

use bincode_next::{Decode, Encode};

pub const DEFAULT_TTL_SECONDS: u64 = 300;

/// Encrypted state representing a captcha challenge.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct TokenPayload {
    pub solution: Vec<u8>,
    pub difficulty: u8,
    pub timestamp: u64,
    pub seed: u64,
}

impl TokenPayload {
    /// Creates a new payload with the current timestamp and a random seed.
    #[must_use]
    pub fn new(solution: Vec<u8>, difficulty: u8) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let seed = crate::common::get_random_u64();

        Self {
            solution,
            difficulty,
            timestamp,
            seed,
        }
    }

    /// Checks if the payload timestamp has exceeded the specified TTL.
    #[must_use]
    pub fn is_expired(&self, ttl_seconds: u64) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        now.saturating_sub(self.timestamp) > ttl_seconds
    }

    /// Checks if the payload has exceeded the default TTL (300 seconds).
    #[must_use]
    pub fn is_expired_default(&self) -> bool {
        self.is_expired(DEFAULT_TTL_SECONDS)
    }

    /// Serializes the payload into a binary vector.
    ///
    /// # Errors
    ///
    /// Fails if the serialization process encounters an error.
    pub fn to_bytes(&self) -> crate::common::error::Result<Vec<u8>> {
        Ok(bincode_next::encode_to_vec(
            self,
            bincode_next::config::standard(),
        )?)
    }

    /// Deserializes a binary slice into a `TokenPayload` instance.
    ///
    /// # Errors
    ///
    /// Fails if the input data is malformed or not a valid payload.
    pub fn from_bytes(bytes: &[u8]) -> crate::common::error::Result<Self> {
        let (payload, _) =
            bincode_next::decode_from_slice(bytes, bincode_next::config::standard())?;
        Ok(payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payload_serialization() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let payload = TokenPayload {
            solution: vec![1, 2, 3, 4, 5],
            difficulty: 2,
            timestamp: 1_234_567_890,
            seed: 0,
        };

        let bytes = payload.to_bytes()?;
        let decoded = TokenPayload::from_bytes(&bytes)?;

        assert_eq!(payload.solution, decoded.solution);
        assert_eq!(payload.difficulty, decoded.difficulty);
        assert_eq!(payload.timestamp, decoded.timestamp);
        Ok(())
    }

    #[test]
    fn new_sets_timestamp() {
        let payload = TokenPayload::new(vec![1, 2, 3], 1);
        assert!(payload.timestamp > 0);
    }

    #[test]
    fn not_expired() {
        let payload = TokenPayload::new(vec![1, 2, 3], 1);
        assert!(!payload.is_expired(DEFAULT_TTL_SECONDS));
        assert!(!payload.is_expired_default());
    }

    #[test]
    fn expired_check() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        let payload = TokenPayload {
            solution: vec![1, 2, 3],
            difficulty: 1,
            timestamp: now.saturating_sub(1000),
            seed: 0,
        };
        assert!(payload.is_expired(100));
    }

    #[test]
    fn invalid_bytes_handling() {
        let invalid_bytes = vec![0xFF, 0xFF, 0xFF];
        let result = TokenPayload::from_bytes(&invalid_bytes);
        assert!(result.is_err());
    }
}
