//! Logic for validating matching pair identification solutions.

use crate::common::error::{AnCaptchaError, Result};
use crate::crypto::token::TokenPayload;

use subtle::ConstantTimeEq;

/// Validates a submitted pair identification solution against the expected payload cells.
///
/// # Errors
///
/// Fails if the payload solution data is malformed or if submitted values are invalid.
pub fn validate_pair_solution(
    payload: &TokenPayload,
    submitted_values: &[Vec<&str>],
) -> Result<bool> {
    let steps = payload.difficulty as usize;
    if submitted_values.len() < steps {
        return Ok(false);
    }

    let value_mapping = [
        ("c0", 0u8),
        ("c1", 1),
        ("c2", 2),
        ("c3", 3),
        ("c4", 4),
        ("c5", 5),
        ("c6", 6),
        ("c7", 7),
        ("c8", 8),
    ];

    let mut valid = 1u8;

    for step_idx in 0..steps {
        let step_selections = submitted_values
            .get(step_idx)
            .ok_or(AnCaptchaError::InvalidToken)?;
        if step_selections.len() != 2 {
            return Ok(false);
        }

        let pair_offset = step_idx * 2;
        let expected_cell1 = payload
            .solution
            .get(pair_offset)
            .copied()
            .ok_or(AnCaptchaError::InvalidToken)?;
        let expected_cell2 = payload
            .solution
            .get(pair_offset + 1)
            .copied()
            .ok_or(AnCaptchaError::InvalidToken)?;

        let submitted_cells: Vec<u8> = step_selections
            .iter()
            .filter_map(|v| {
                value_mapping
                    .iter()
                    .find(|(k, _)| *k == *v)
                    .map(|(_, cell)| *cell)
            })
            .collect();

        if submitted_cells.len() != 2 {
            return Ok(false);
        }

        let s0 = *submitted_cells
            .first()
            .ok_or(AnCaptchaError::InvalidToken)?;
        let s1 = *submitted_cells.get(1).ok_or(AnCaptchaError::InvalidToken)?;

        let match_direct = expected_cell1.ct_eq(&s0) & expected_cell2.ct_eq(&s1);
        let match_crossed = expected_cell1.ct_eq(&s1) & expected_cell2.ct_eq(&s0);
        let step_valid = match_direct | match_crossed;

        valid &= step_valid.unwrap_u8();
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
            solution: vec![0, 2, 4, 6],
            difficulty: Difficulty::Medium.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec![vec!["c0", "c2"], vec!["c4", "c6"]];
        let result = validate_pair_solution(&payload, &submitted)?;
        assert!(result);
        Ok(())
    }

    #[test]
    fn incorrect_solution() -> Result<()> {
        let payload = TokenPayload {
            solution: vec![0, 2],
            difficulty: Difficulty::Easy.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec![vec!["c0", "c1"]];
        let result = validate_pair_solution(&payload, &submitted)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn wrong_selection_count() -> Result<()> {
        let payload = TokenPayload {
            solution: vec![0, 2],
            difficulty: Difficulty::Easy.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec![vec!["c0", "c1", "c2"]];
        let result = validate_pair_solution(&payload, &submitted)?;
        assert!(!result);
        Ok(())
    }

    #[test]
    fn invalid_range() -> Result<()> {
        let payload = TokenPayload {
            solution: vec![0, 2],
            difficulty: Difficulty::Easy.steps(),
            timestamp: 1000,
            seed: 0,
        };
        let submitted = vec![vec!["c99", "c0"]];
        let result = validate_pair_solution(&payload, &submitted)?;
        assert!(!result);
        Ok(())
    }
}
