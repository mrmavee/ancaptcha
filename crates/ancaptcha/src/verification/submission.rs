//! Verification submission types and deserialization logic.

use crate::common::Secret;
use crate::common::error::Result;
use crate::config::Difficulty;
use crate::verification::{verify_pair, verify_rotate, verify_slider};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

/// Submitted data for a Rotate captcha challenge.
#[derive(Debug)]
pub struct RotateSubmission {
    /// Encrypted challenge token.
    pub token: String,
    /// Obfuscated step 0 value.
    pub s0: Option<String>,
    /// Obfuscated step 1 value.
    pub s1: Option<String>,
    /// Obfuscated step 2 value.
    pub s2: Option<String>,
}

impl<'de> Deserialize<'de> for RotateSubmission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs = Vec::<(String, String)>::deserialize(deserializer)?;
        let mut form_map: HashMap<String, Vec<String>> = HashMap::new();
        for (k, v) in pairs {
            form_map.entry(k).or_default().push(v);
        }
        Ok(Self::from_map(&form_map))
    }
}

impl RotateSubmission {
    /// Constructs a submission instance from a hash map of unscrambled form values.
    #[must_use]
    pub fn from_map(map: &HashMap<String, Vec<String>>) -> Self {
        let mut token = String::new();
        let (mut s0, mut s1, mut s2) = (None, None, None);

        if let Some(v) = map.get("_t").and_then(|vals| vals.first()) {
            token.clone_from(v);
        }
        if let Some(v) = map.get("_s0").and_then(|vals| vals.first()) {
            s0 = Some(v.clone());
        }
        if let Some(v) = map.get("_s1").and_then(|vals| vals.first()) {
            s1 = Some(v.clone());
        }
        if let Some(v) = map.get("_s2").and_then(|vals| vals.first()) {
            s2 = Some(v.clone());
        }

        Self { token, s0, s1, s2 }
    }

    /// Validates the rotation submission.
    ///
    /// # Errors
    ///
    /// Fails if the token is invalid or if the verification process encounters an error.
    pub fn verify(&self, secret: &Secret, _diff: Difficulty) -> Result<bool> {
        let vals: Vec<&str> = [self.s0.as_deref(), self.s1.as_deref(), self.s2.as_deref()]
            .iter()
            .flatten()
            .copied()
            .collect();
        verify_rotate(secret, &self.token, &vals)
    }
}

/// Submitted data for a Slider captcha challenge.
#[derive(Debug)]
pub struct SliderSubmission {
    /// Encrypted challenge token.
    pub token: String,
    /// Obfuscated step 0 value.
    pub s0: Option<String>,
    /// Obfuscated step 1 value.
    pub s1: Option<String>,
    /// Obfuscated step 2 value.
    pub s2: Option<String>,
}

impl<'de> Deserialize<'de> for SliderSubmission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs = Vec::<(String, String)>::deserialize(deserializer)?;
        let mut form_map: HashMap<String, Vec<String>> = HashMap::new();
        for (k, v) in pairs {
            form_map.entry(k).or_default().push(v);
        }
        Ok(Self::from_map(&form_map))
    }
}

impl SliderSubmission {
    /// Constructs a submission instance from a hash map of unscrambled form values.
    #[must_use]
    pub fn from_map(map: &HashMap<String, Vec<String>>) -> Self {
        let mut token = String::new();
        let (mut s0, mut s1, mut s2) = (None, None, None);

        if let Some(v) = map.get("_t").and_then(|vals| vals.first()) {
            token.clone_from(v);
        }
        if let Some(v) = map.get("_s0").and_then(|vals| vals.first()) {
            s0 = Some(v.clone());
        }
        if let Some(v) = map.get("_s1").and_then(|vals| vals.first()) {
            s1 = Some(v.clone());
        }
        if let Some(v) = map.get("_s2").and_then(|vals| vals.first()) {
            s2 = Some(v.clone());
        }

        Self { token, s0, s1, s2 }
    }

    /// Validates the slider submission.
    ///
    /// # Errors
    ///
    /// Fails if the token is invalid or if the verification process encounters an error.
    pub fn verify(&self, secret: &Secret, _diff: Difficulty) -> Result<bool> {
        let vals: Vec<&str> = [self.s0.as_deref(), self.s1.as_deref(), self.s2.as_deref()]
            .iter()
            .flatten()
            .copied()
            .collect();
        verify_slider(secret, &self.token, &vals)
    }
}

/// Submitted data for a Pair identification challenge.
#[derive(Debug)]
pub struct PairSubmission {
    /// Encrypted challenge token.
    pub token: String,
    /// Selected cell identifiers for step 0.
    pub s0: Vec<String>,
    /// Selected cell identifiers for step 1.
    pub s1: Vec<String>,
    /// Selected cell identifiers for step 2.
    pub s2: Vec<String>,
}

impl<'de> Deserialize<'de> for PairSubmission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let pairs = Vec::<(String, String)>::deserialize(deserializer)?;
        let mut form_map: HashMap<String, Vec<String>> = HashMap::new();
        for (k, v) in pairs {
            form_map.entry(k).or_default().push(v);
        }
        Ok(Self::from_map(&form_map))
    }
}

impl PairSubmission {
    /// Constructs a submission instance from a hash map of unscrambled form values.
    #[must_use]
    pub fn from_map(map: &HashMap<String, Vec<String>>) -> Self {
        let mut token = String::new();
        let (mut s0, mut s1, mut s2) = (Vec::new(), Vec::new(), Vec::new());

        if let Some(v) = map.get("_t").and_then(|vals| vals.first()) {
            token.clone_from(v);
        }

        for (k, v) in map {
            if k.starts_with("_s0_") {
                s0.extend(v.clone());
            } else if k.starts_with("_s1_") {
                s1.extend(v.clone());
            } else if k.starts_with("_s2_") {
                s2.extend(v.clone());
            }
        }

        Self { token, s0, s1, s2 }
    }

    /// Validates the pair identification submission.
    ///
    /// # Errors
    ///
    /// Fails if the token is invalid or if the verification process encounters an error.
    pub fn verify(&self, secret: &Secret, diff: Difficulty) -> Result<bool> {
        let steps = diff.steps() as usize;
        let mut stages = Vec::with_capacity(steps);

        if steps > 0 {
            stages.push(self.s0.iter().map(String::as_str).collect::<Vec<_>>());
        }
        if steps > 1 {
            stages.push(self.s1.iter().map(String::as_str).collect::<Vec<_>>());
        }
        if steps > 2 {
            stages.push(self.s2.iter().map(String::as_str).collect::<Vec<_>>());
        }

        let refs: Vec<Vec<&str>> = stages.iter().map(|s| s.as_slice().to_vec()).collect();

        verify_pair(secret, &self.token, &refs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_submission_map() {
        let mut map = HashMap::new();
        map.insert("_t".to_string(), vec!["token".to_string()]);
        map.insert("_s0".to_string(), vec!["v1".to_string()]);
        map.insert("_s1".to_string(), vec!["v2".to_string()]);

        let sub = RotateSubmission::from_map(&map);
        assert_eq!(sub.token, "token");
        assert_eq!(sub.s0, Some("v1".to_string()));
        assert_eq!(sub.s1, Some("v2".to_string()));
        assert_eq!(sub.s2, None);
    }

    #[test]
    fn slider_submission_map() {
        let mut map = HashMap::new();
        map.insert("_t".to_string(), vec!["t".to_string()]);
        map.insert("_s0".to_string(), vec!["p5".to_string()]);

        let sub = SliderSubmission::from_map(&map);
        assert_eq!(sub.token, "t");
        assert_eq!(sub.s0, Some("p5".to_string()));
    }

    #[test]
    fn pair_submission_map() {
        let mut map = HashMap::new();
        map.insert("_t".to_string(), vec!["t".to_string()]);
        map.insert("_s0_0".to_string(), vec!["c0".to_string()]);
        map.insert("_s0_1".to_string(), vec!["c1".to_string()]);

        let sub = PairSubmission::from_map(&map);
        assert_eq!(sub.token, "t");
        assert!(sub.s0.contains(&"c0".to_string()));
        assert!(sub.s0.contains(&"c1".to_string()));
    }

    #[test]
    fn submission_missing_fields() {
        let map = HashMap::new();
        let sub = RotateSubmission::from_map(&map);
        assert!(sub.token.is_empty());
        assert!(sub.s0.is_none());

        let slider = SliderSubmission::from_map(&map);
        assert!(slider.token.is_empty());

        let pair = PairSubmission::from_map(&map);
        assert!(pair.s0.is_empty());
    }

    #[test]
    fn submission_verification_dispatch() -> Result<()> {
        let secret = [0u8; 32];
        let payload = crate::crypto::token::TokenPayload::new(vec![0, 0], 1);
        let bytes = payload.to_bytes()?;
        let encrypted = crate::crypto::cipher::encrypt(&secret, &bytes)?;
        let token = crate::common::b64_encode_url_safe(encrypted);

        let sub = RotateSubmission {
            token: token.clone(),
            s0: Some("v0".to_string()),
            s1: None,
            s2: None,
        };
        assert!(sub.verify(&secret, Difficulty::Easy)?);

        let slider = SliderSubmission {
            token: token.clone(),
            s0: Some("p0".to_string()),
            s1: None,
            s2: None,
        };
        assert!(slider.verify(&secret, Difficulty::Easy)?);

        let pair = PairSubmission {
            token,
            s0: vec!["c0".to_string(), "c0".to_string()],
            s1: Vec::new(),
            s2: Vec::new(),
        };
        assert!(pair.verify(&secret, Difficulty::Easy)?);
        Ok(())
    }

    #[test]
    fn submission_verification_failures() -> Result<()> {
        let secret = [1u8; 32];
        let payload = crate::crypto::token::TokenPayload::new(vec![0, 0], 1);
        let bytes = payload.to_bytes()?;
        let encrypted = crate::crypto::cipher::encrypt(&secret, &bytes)?;
        let token = crate::common::b64_encode_url_safe(encrypted);

        let sub = RotateSubmission {
            token: token.clone(),
            s0: Some("v1".to_string()),
            s1: None,
            s2: None,
        };
        assert!(!sub.verify(&secret, Difficulty::Easy)?);

        let slider = SliderSubmission {
            token: token.clone(),
            s0: Some("p1".to_string()),
            s1: None,
            s2: None,
        };
        assert!(!slider.verify(&secret, Difficulty::Easy)?);

        let pair = PairSubmission {
            token,
            s0: vec!["c1".to_string(), "c1".to_string()],
            s1: Vec::new(),
            s2: Vec::new(),
        };
        assert!(!pair.verify(&secret, Difficulty::Easy)?);
        Ok(())
    }
}
