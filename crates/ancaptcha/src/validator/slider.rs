//! Logic for validating puzzle slider challenge solutions.

use crate::common::error::{AnCaptchaError, Result};
use crate::crypto::token::TokenPayload;

use subtle::ConstantTimeEq;

/// Validates a submitted slider solution against the expected payload positions.
///
/// # Errors
///
/// Fails if the payload solution data is malformed or if submitted values are invalid.
pub fn validate_slider_solution(payload: &TokenPayload, submitted_values: &[&str]) -> Result<bool> {
    if payload.solution.len() != submitted_values.len() * 2 {
        return Ok(false);
    }

    let mut valid = 1u8;

    for (idx, submitted) in submitted_values.iter().enumerate() {
        let expected_pos = payload
            .solution
            .get(idx * 2)
            .copied()
            .ok_or(AnCaptchaError::InvalidToken)?;

        let submitted_pos = submitted
            .strip_prefix("p")
            .and_then(|s| s.parse::<u8>().ok())
            .filter(|&v| v < 20)
            .ok_or(AnCaptchaError::InvalidToken)?;

        valid &= expected_pos.ct_eq(&submitted_pos).unwrap_u8();
    }

    Ok(valid == 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Difficulty;

    #[test]
    fn correct_solution() -> Result<()> {
        let payload = TokenPayload {
            solution: vec![5, 140, 7, 140],
            difficulty: Difficulty::Medium.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec!["p5", "p7"];
        let result = validate_slider_solution(&payload, &submitted)?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn incorrect_solution() -> Result<()> {
        let payload = TokenPayload {
            solution: vec![5, 140, 7, 140],
            difficulty: Difficulty::Medium.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec!["p5", "p3"];
        let result = validate_slider_solution(&payload, &submitted)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn invalid_range() {
        let payload = TokenPayload {
            solution: vec![5, 140],
            difficulty: Difficulty::Easy.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec!["p99"];
        let result = validate_slider_solution(&payload, &submitted);
        assert!(result.is_err());
    }
}
