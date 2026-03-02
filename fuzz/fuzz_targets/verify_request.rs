#![no_main]

use ancaptcha::{AnCaptcha, Config, Difficulty, NoiseIntensity};
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;
use std::sync::LazyLock;

static CAPTCHA: LazyLock<AnCaptcha> = LazyLock::new(|| {
    let secret = [0u8; 32];
    let config = Config::new(secret)
        .with_difficulty(Difficulty::Hard)
        .with_noise_intensity(NoiseIntensity::High);
    AnCaptcha::new(config)
});

#[derive(Arbitrary, Debug)]
pub struct FormInput {
    pub map: HashMap<String, Vec<String>>,
}

fuzz_target!(|input: FormInput| {
    let _ = CAPTCHA.verify_request(&input.map);
});
