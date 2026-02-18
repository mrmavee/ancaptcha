//! Service interface for captcha generation and verification.

use crate::common::error::Result;
use crate::config::Config;
use crate::verification::factory::{
    CaptchaBundle, generate_pair_bundle, generate_rotate_bundle, generate_slider_bundle,
};
use crate::verification::validator::{verify_pair, verify_rotate, verify_slider};

/// Entry point for captcha operations.
pub struct AnCaptcha {
    config: Config,
}

impl AnCaptcha {
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// # Errors
    ///
    /// Fails if the asset cache is uninitialized or if token encryption fails.
    pub fn generate_rotate(&self, error_message: Option<&str>) -> Result<CaptchaBundle> {
        generate_rotate_bundle(
            &self.config.secret,
            self.config.difficulty,
            &self.config.theme,
            &self.config.layout,
            error_message,
        )
    }

    /// # Errors
    ///
    /// Fails if the asset cache is uninitialized or if token encryption fails.
    pub fn generate_slider(&self, error_message: Option<&str>) -> Result<CaptchaBundle> {
        generate_slider_bundle(
            &self.config.secret,
            self.config.difficulty,
            &self.config.theme,
            &self.config.layout,
            error_message,
        )
    }

    /// # Errors
    ///
    /// Fails if the asset cache is uninitialized or if token encryption fails.
    pub fn generate_pair(&self, error_message: Option<&str>) -> Result<CaptchaBundle> {
        generate_pair_bundle(
            &self.config.secret,
            self.config.difficulty,
            &self.config.theme,
            &self.config.layout,
            error_message,
        )
    }

    /// # Errors
    ///
    /// Fails if the token is invalid, malformed, or has expired.
    pub fn verify_rotate(&self, token: &str, submitted_values: &[&str]) -> Result<bool> {
        verify_rotate(&self.config.secret, token, submitted_values)
    }

    /// # Errors
    ///
    /// Fails if the token is invalid, malformed, or has expired.
    pub fn verify_slider(&self, token: &str, submitted_values: &[&str]) -> Result<bool> {
        verify_slider(&self.config.secret, token, submitted_values)
    }

    /// # Errors
    ///
    /// Fails if the token is invalid, malformed, or has expired.
    pub fn verify_pair(&self, token: &str, submitted_values: &[Vec<&str>]) -> Result<bool> {
        verify_pair(&self.config.secret, token, submitted_values)
    }

    /// # Errors
    ///
    /// Fails if the token cannot be found in the form data or if decryption fails.
    pub fn verify_request<S: std::hash::BuildHasher>(
        &self,
        form_data: &std::collections::HashMap<String, Vec<String>, S>,
    ) -> Result<bool> {
        let (payload, unscrambled, captcha_type) =
            crate::verification::validator::unscramble_form(&self.config.secret, form_data)?;

        let difficulty = match payload.difficulty {
            1 => crate::Difficulty::Easy,
            2 => crate::Difficulty::Medium,
            _ => crate::Difficulty::Hard,
        };

        match captcha_type {
            1 => {
                let sub = crate::verification::submission::SliderSubmission::from_map(&unscrambled);
                sub.verify(&self.config.secret, difficulty)
            }
            2 => {
                let sub = crate::verification::submission::PairSubmission::from_map(&unscrambled);
                sub.verify(&self.config.secret, difficulty)
            }
            _ => {
                let sub = crate::verification::submission::RotateSubmission::from_map(&unscrambled);
                sub.verify(&self.config.secret, difficulty)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AnCaptchaError, init_cache};
    use std::collections::HashMap;

    fn setup_test_cache() {
        let cache = init_cache();
        cache.warm_up(crate::CaptchaStyle::Rotate);
        cache.warm_up(crate::CaptchaStyle::Slider);
        cache.warm_up(crate::CaptchaStyle::Pair);
    }

    #[test]
    fn bundle_generation() -> Result<()> {
        setup_test_cache();
        let config = Config::new([0u8; 32]);
        let ac = AnCaptcha::new(config);

        let rotate = ac.generate_rotate(None)?;
        assert!(!rotate.token.is_empty());
        assert!(rotate.html.contains("ROTATE THE PIC"));

        let slider = ac.generate_slider(None)?;
        assert!(!slider.token.is_empty());
        assert!(slider.html.contains("CLICK SLIDER TO ALIGN"));

        let pair = ac.generate_pair(None)?;
        assert!(!pair.token.is_empty());
        assert!(pair.html.contains("FIND THE PAIR"));
        Ok(())
    }

    #[test]
    fn auto_request_verification() -> Result<()> {
        setup_test_cache();
        let secret = [9u8; 32];
        let ac = AnCaptcha::new(Config::new(secret).with_difficulty(crate::Difficulty::Easy));

        let bundle = ac.generate_rotate(None)?;
        let t_bytes = crate::common::b64_decode_url_safe(&bundle.token)?;
        let plaintext = crate::crypto::cipher::decrypt(&secret, &t_bytes)?;
        let payload = crate::crypto::token::TokenPayload::from_bytes(&plaintext)?;

        let mut mapper = crate::engine::NameMapper::new(payload.seed);
        mapper.warm_up(payload.difficulty, "rotate");

        let mut form = HashMap::new();
        form.insert(mapper.get_or_create("token"), vec![bundle.token]);
        form.insert(
            mapper.get_or_create("st"),
            vec![mapper.get_or_create("st0")],
        );

        let sol = payload
            .solution
            .get(0..2)
            .ok_or(AnCaptchaError::InvalidToken)?;
        let val_idx =
            u16::from_le_bytes(sol.try_into().map_err(|_| AnCaptchaError::InvalidToken)?) / 45;
        let v_obf = mapper.get_or_create(&format!("v{val_idx}"));
        form.insert(mapper.get_or_create("s0"), vec![v_obf]);

        let (discovered_payload, _, _) =
            crate::verification::validator::unscramble_form(&secret, &form)?;
        assert_eq!(discovered_payload.seed, payload.seed);
        assert!(ac.verify_request(&form)?);
        Ok(())
    }

    #[test]
    fn request_verification_failures() -> Result<()> {
        setup_test_cache();
        let secret = [11u8; 32];
        let ac = AnCaptcha::new(Config::new(secret));

        let mut form = HashMap::new();
        form.insert("token".to_string(), vec!["invalid".to_string()]);
        assert!(ac.verify_request(&form).is_err());

        let bundle = ac.generate_rotate(None)?;
        let mut form_wrong = HashMap::new();
        form_wrong.insert("token".to_string(), vec![bundle.token]);
        assert!(ac.verify_request(&form_wrong).is_err());

        Ok(())
    }

    #[test]
    fn explicit_verifications() -> Result<()> {
        setup_test_cache();
        let secret = [10u8; 32];
        let config = Config::new(secret).with_difficulty(crate::Difficulty::Easy);
        let ac = AnCaptcha::new(config);

        let rotate = ac.generate_rotate(None)?;
        let t_rot = crate::common::b64_decode_url_safe(&rotate.token)?;
        let dec_rot = crate::crypto::cipher::decrypt(&secret, &t_rot)?;
        let p_rot = crate::crypto::token::TokenPayload::from_bytes(&dec_rot)?;
        let sol_rot = p_rot
            .solution
            .get(0..2)
            .ok_or(AnCaptchaError::InvalidToken)?;
        let ang_rot = u16::from_le_bytes(
            sol_rot
                .try_into()
                .map_err(|_| AnCaptchaError::InvalidToken)?,
        );
        let correct_v = format!("v{}", ang_rot / 45);
        assert!(ac.verify_rotate(&rotate.token, &[&correct_v])?);

        let slider = ac.generate_slider(None)?;
        let t_sli = crate::common::b64_decode_url_safe(&slider.token)?;
        let dec_sli = crate::crypto::cipher::decrypt(&secret, &t_sli)?;
        let p_sli = crate::crypto::token::TokenPayload::from_bytes(&dec_sli)?;
        let p_idx = *p_sli.solution.first().ok_or(AnCaptchaError::InvalidToken)?;
        let correct_p = format!("p{p_idx}");
        assert!(ac.verify_slider(&slider.token, &[&correct_p])?);

        let pair = ac.generate_pair(None)?;
        let t_pa = crate::common::b64_decode_url_safe(&pair.token)?;
        let dec_pa = crate::crypto::cipher::decrypt(&secret, &t_pa)?;
        let p_pa = crate::crypto::token::TokenPayload::from_bytes(&dec_pa)?;
        let c1 = *p_pa.solution.first().ok_or(AnCaptchaError::InvalidToken)?;
        let c2 = *p_pa.solution.get(1).ok_or(AnCaptchaError::InvalidToken)?;
        let val_pair = [format!("c{c1}"), format!("c{c2}")];
        let val_refs: Vec<&str> = val_pair.iter().map(String::as_str).collect();
        assert!(ac.verify_pair(&pair.token, &[val_refs])?);

        Ok(())
    }
}
