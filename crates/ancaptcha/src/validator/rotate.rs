//! Logic for validating image rotation challenge solutions.

use crate::common::error::{AnCaptchaError, Result};
use crate::crypto::token::TokenPayload;

use subtle::ConstantTimeEq;

/// Validates a submitted rotation solution against the expected payload angles.
///
/// # Errors
///
/// Fails if the payload solution data is malformed or if submitted values are invalid.
pub fn validate_rotate_solution(payload: &TokenPayload, submitted_values: &[&str]) -> Result<bool> {
    if payload.solution.len() != submitted_values.len() * 2 {
        return Ok(false);
    }

    let value_mapping = [
        ("v0", 0u16),
        ("v1", 45),
        ("v2", 90),
        ("v3", 135),
        ("v4", 180),
        ("v5", 225),
        ("v6", 270),
        ("v7", 315),
    ];

    let mut valid = 1u8;

    for (idx, submitted) in submitted_values.iter().enumerate() {
        let offset = idx * 2;
        let b1 = *payload
            .solution
            .get(offset)
            .ok_or(AnCaptchaError::InvalidToken)?;
        let b2 = *payload
            .solution
            .get(offset + 1)
            .ok_or(AnCaptchaError::InvalidToken)?;
        let expected_angle = u16::from_le_bytes([b1, b2]);

        let submitted_angle = value_mapping
            .iter()
            .find(|(v, _)| *v == *submitted)
            .map(|(_, angle)| *angle)
            .ok_or(AnCaptchaError::InvalidToken)?;

        valid &= expected_angle.ct_eq(&submitted_angle).unwrap_u8();
    }

    Ok(valid == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Difficulty;

    #[test]
    fn correct_solution() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let angles: Vec<u16> = vec![90, 180];
        let solution: Vec<u8> = angles.iter().copied().flat_map(u16::to_le_bytes).collect();
        let payload = TokenPayload {
            solution,
            difficulty: Difficulty::Medium.steps(),
            timestamp: 1000,
            seed: 0,
        };

        let submitted = vec!["v2", "v4"];

        let result = validate_rotate_solution(&payload, &submitted)?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn incorrect_solution() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let angles: Vec<u16> = vec![90, 180];
        let solution: Vec<u8> = angles.iter().copied().flat_map(u16::to_le_bytes).collect();
        let payload = TokenPayload {
            solution,
            difficulty: Difficulty::Medium.steps(),
            timestamp: 1000,
            seed: 0,
        };

        let submitted = vec!["v2", "v3"];

        let result = validate_rotate_solution(&payload, &submitted)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn invalid_range_values() {
        let payload = TokenPayload {
            solution: vec![0, 0],
            difficulty: Difficulty::Easy.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec!["v99"];
        let result = validate_rotate_solution(&payload, &submitted);
        assert!(result.is_err());
    }
}
