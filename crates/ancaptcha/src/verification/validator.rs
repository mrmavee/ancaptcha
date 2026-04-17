//! Form data processing and challenge response validation.

use crate::common::Secret;
use crate::common::error::{AnCaptchaError, Result};
use crate::crypto::token::TokenPayload;
use std::collections::HashMap;

fn decode_token_payload(secret: &Secret, token: &str) -> Result<TokenPayload> {
    let token_bytes = crate::common::b64_decode_url_safe(token)?;
    let plaintext = crate::crypto::cipher::decrypt(secret, &token_bytes)?;
    let payload = TokenPayload::from_bytes(&plaintext)?;

    if payload.is_expired_default() {
        return Err(AnCaptchaError::ExpiredToken);
    }

    Ok(payload)
}

fn prepare_verification(
    secret: &Secret,
    token: &str,
    captcha_type: &str,
) -> Result<(TokenPayload, crate::engine::NameMapper)> {
    let payload = decode_token_payload(secret, token)?;
    let mut mapper = crate::engine::NameMapper::new(payload.seed);
    mapper.warm_up(payload.difficulty, captcha_type);
    Ok((payload, mapper))
}

fn unscramble_slice(mapper: &crate::engine::NameMapper, values: &[&str]) -> Vec<String> {
    values
        .iter()
        .map(|v| mapper.get_semantic(v).unwrap_or_else(|| (*v).to_string()))
        .collect()
}

fn unscramble_vec_slice(
    mapper: &crate::engine::NameMapper,
    values: &[Vec<&str>],
) -> Vec<Vec<String>> {
    values
        .iter()
        .map(|step| unscramble_slice(mapper, step))
        .collect()
}

/// Verifies a Rotate captcha submission.
///
/// # Errors
///
/// Fails if the token is invalid, expired, or if internal verification fails.
pub fn verify_rotate(secret: &Secret, token: &str, submitted_values: &[&str]) -> Result<bool> {
    let (payload, mapper) =
        prepare_verification(secret, token, crate::common::CAPTCHA_TYPE_ROTATE)?;

    let unscrambled = unscramble_slice(&mapper, submitted_values);
    let refs: Vec<&str> = unscrambled.iter().map(String::as_str).collect();

    crate::validator::rotate::validate_rotate_solution(&payload, &refs)
}

/// Verifies a Slider captcha submission.
///
/// # Errors
///
/// Fails if the token is invalid, expired, or if internal verification fails.
pub fn verify_slider(secret: &Secret, token: &str, submitted_values: &[&str]) -> Result<bool> {
    let (payload, mapper) =
        prepare_verification(secret, token, crate::common::CAPTCHA_TYPE_SLIDER)?;

    let unscrambled = unscramble_slice(&mapper, submitted_values);
    let refs: Vec<&str> = unscrambled.iter().map(String::as_str).collect();

    crate::validator::slider::validate_slider_solution(&payload, &refs)
}

/// Verifies a Find the Pair captcha submission.
///
/// # Errors
///
/// Fails if the token is invalid, expired, or if internal verification fails.
pub fn verify_pair(secret: &Secret, token: &str, submitted_values: &[Vec<&str>]) -> Result<bool> {
    let (payload, mapper) = prepare_verification(secret, token, crate::common::CAPTCHA_TYPE_PAIR)?;

    let unscrambled = unscramble_vec_slice(&mapper, submitted_values);
    let refs: Vec<Vec<&str>> = unscrambled
        .iter()
        .map(|step| step.iter().map(String::as_str).collect())
        .collect();

    crate::validator::pair::validate_pair_solution(&payload, &refs)
}

fn find_token_in_form<S: std::hash::BuildHasher>(
    secret: &Secret,
    form: &HashMap<String, Vec<String>, S>,
) -> Result<(String, String, TokenPayload)> {
    form.iter()
        .find_map(|(key, vals)| {
            vals.iter().find_map(|val| {
                let token_bytes = crate::common::b64_decode_url_safe(val).ok()?;
                let plaintext = crate::crypto::cipher::decrypt(secret, &token_bytes).ok()?;
                let payload = TokenPayload::from_bytes(&plaintext).ok()?;
                Some((key.clone(), val.clone(), payload))
            })
        })
        .ok_or(AnCaptchaError::InvalidToken)
}

fn attempt_unscramble_type<S: std::hash::BuildHasher>(
    payload: &TokenPayload,
    form: &HashMap<String, Vec<String>, S>,
    captcha_type: &str,
    token_key: &str,
    token_val: &str,
) -> Option<HashMap<String, Vec<String>>> {
    let mut mapper = crate::engine::NameMapper::new(payload.seed);
    mapper.warm_up(payload.difficulty, captcha_type);

    let token_field_obf = mapper.get_or_create("token");
    if token_field_obf != token_key {
        return None;
    }

    let stage_field_semantic = if captcha_type == crate::common::CAPTCHA_TYPE_PAIR {
        "pst"
    } else {
        "st"
    };
    let stage_field_obf = mapper.get_or_create(stage_field_semantic);
    if !form.contains_key(&stage_field_obf) {
        return None;
    }

    if captcha_type == crate::common::CAPTCHA_TYPE_SLIDER {
        let s0_obf = mapper.get_or_create("s0");
        let vals = form.get(&s0_obf)?;
        if vals
            .first()
            .and_then(|v| mapper.get_semantic(v))
            .is_some_and(|s| !s.starts_with('p'))
        {
            return None;
        }
    }

    if captcha_type == crate::common::CAPTCHA_TYPE_ROTATE {
        let s0_obf = mapper.get_or_create("s0");
        let vals = form.get(&s0_obf)?;
        if vals
            .first()
            .and_then(|v| mapper.get_semantic(v))
            .is_some_and(|s| !s.starts_with('v'))
        {
            return None;
        }
    }

    if captcha_type == crate::common::CAPTCHA_TYPE_PAIR {
        let mut found_step = false;
        for j in 0..9 {
            let cell_obf = mapper.get_or_create(&format!("s0_{j}"));
            if form.contains_key(&cell_obf) {
                found_step = true;
                break;
            }
        }
        if !found_step {
            return None;
        }
    }

    let mut current_unscrambled = HashMap::new();
    current_unscrambled.insert("_t".to_string(), vec![token_val.to_string()]);

    if let Some(vals) = form.get(&stage_field_obf) {
        let mut semantic_vals = Vec::new();
        for v in vals {
            semantic_vals.push(mapper.get_semantic(v).unwrap_or_else(|| v.clone()));
        }
        current_unscrambled.insert(format!("_{stage_field_semantic}"), semantic_vals);
    }

    for i in 0..payload.difficulty {
        let f_semantic = format!("s{i}");
        let f_obf = mapper.get_or_create(&f_semantic);
        if let Some(vals) = form.get(&f_obf) {
            let mut semantic_vals = Vec::new();
            for v in vals {
                semantic_vals.push(mapper.get_semantic(v).unwrap_or_else(|| v.clone()));
            }
            current_unscrambled.insert(format!("_{f_semantic}"), semantic_vals);
        }

        if captcha_type == crate::common::CAPTCHA_TYPE_PAIR {
            for j in 0..9 {
                let pair_f_semantic = format!("s{i}_{j}");
                let pair_f_obf = mapper.get_or_create(&pair_f_semantic);
                if let Some(vals) = form.get(&pair_f_obf) {
                    let mut semantic_vals = Vec::new();
                    for v in vals {
                        semantic_vals.push(mapper.get_semantic(v).unwrap_or_else(|| v.clone()));
                    }
                    current_unscrambled.insert(format!("_{pair_f_semantic}"), semantic_vals);
                }
            }
        }
    }
    Some(current_unscrambled)
}

/// Reverses identifier obfuscation from form data and identifies the captcha type.
///
/// # Errors
///
/// Fails if no valid token is found or if decryption fails.
pub fn unscramble_form<S: std::hash::BuildHasher>(
    secret: &Secret,
    form: &HashMap<String, Vec<String>, S>,
) -> Result<(TokenPayload, HashMap<String, Vec<String>>, u8)> {
    let (token_key, token_val, payload) = find_token_in_form(secret, form)?;

    let captcha_types = [
        (crate::common::CAPTCHA_TYPE_SLIDER, 1u8),
        (crate::common::CAPTCHA_TYPE_PAIR, 2u8),
        (crate::common::CAPTCHA_TYPE_ROTATE, 3u8),
    ];

    let mut best_unscrambled = HashMap::new();
    let mut detected_type = 0;

    for (ct, ct_idx) in captcha_types {
        if let Some(unscrambled) =
            attempt_unscramble_type(&payload, form, ct, &token_key, &token_val)
        {
            best_unscrambled = unscrambled;
            detected_type = ct_idx;
            break;
        }
    }

    Ok((payload, best_unscrambled, detected_type))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Difficulty;

    #[test]
    fn token_discovery() -> Result<()> {
        let secret = [7u8; 32];
        let payload = TokenPayload::new(vec![1, 2], 1);
        let token_bytes = payload.to_bytes()?;
        let encrypted = crate::crypto::cipher::encrypt(&secret, &token_bytes)?;
        let token_b64 = crate::common::b64_encode_url_safe(encrypted);

        let mut form = HashMap::new();
        form.insert("random_key".to_string(), vec!["random_val".to_string()]);
        form.insert("tok".to_string(), vec![token_b64.clone()]);

        let (key, val, discovered_payload) = find_token_in_form(&secret, &form)?;
        assert_eq!(key, "tok");
        assert_eq!(val, token_b64);
        assert_eq!(discovered_payload.seed, payload.seed);
        Ok(())
    }

    #[test]
    fn rotate_unscrambling() -> Result<()> {
        let secret = [8u8; 32];
        let difficulty = Difficulty::Easy;
        let payload = TokenPayload {
            solution: vec![0, 0],
            difficulty: difficulty.steps(),
            timestamp: 2_000_000_000,
            seed: 123,
        };
        let token_bytes = payload.to_bytes()?;
        let encrypted = crate::crypto::cipher::encrypt(&secret, &token_bytes)?;
        let token_b64 = crate::common::b64_encode_url_safe(encrypted);

        let mut mapper = crate::engine::NameMapper::new(payload.seed);
        mapper.warm_up(payload.difficulty, crate::common::CAPTCHA_TYPE_ROTATE);

        let mut form = HashMap::new();
        form.insert(mapper.get_or_create("token"), vec![token_b64]);
        form.insert(
            mapper.get_or_create("st"),
            vec![mapper.get_or_create("st0")],
        );
        form.insert(mapper.get_or_create("s0"), vec![mapper.get_or_create("v2")]);

        let (discovered_payload, unscrambled, kind) = unscramble_form(&secret, &form)?;
        assert_eq!(discovered_payload.seed, 123);
        assert_eq!(kind, 3);

        let s0_vals = unscrambled.get("_s0").ok_or(AnCaptchaError::InvalidToken)?;
        assert_eq!(s0_vals.first().ok_or(AnCaptchaError::InvalidToken)?, "v2");
        Ok(())
    }
}
