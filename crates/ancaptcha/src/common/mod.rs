//! Shared utilities for encoding, randomization, and type definitions.

pub mod assets;
pub mod error;

use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};
use error::Result;

/// 32-byte array for cryptographic operations.
pub type Secret = [u8; 32];

/// Encodes binary data into a URL-safe base64 string without padding.
pub fn b64_encode_url_safe<T: AsRef<[u8]>>(input: T) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

/// Decodes a URL-safe base64 string.
///
/// # Errors
///
/// Fails if the input string is not valid base64.
pub fn b64_decode_url_safe<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    Ok(URL_SAFE_NO_PAD.decode(input)?)
}

/// Encodes binary data into a standard base64 string.
pub fn b64_encode_std<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}

/// Clamps a 16-bit integer to the 0-255 range.
#[must_use]
pub fn clamp_to_u8(val: i16) -> u8 {
    u8::try_from(val.clamp(0, 255)).unwrap_or(0)
}

/// Returns a random value within the specified range.
pub fn get_random_range<T, R>(range: R) -> T
where
    T: rand::distr::uniform::SampleUniform,
    R: rand::distr::uniform::SampleRange<T>,
{
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random_range(range)
}

/// Returns a random index within the specified length.
#[must_use]
pub fn get_random_index(len: usize) -> usize {
    get_random_range(0..len)
}

/// Returns a random boolean value.
#[must_use]
pub fn get_random_bool() -> bool {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Returns a random float between 0.0 and 1.0.
#[must_use]
pub fn get_random_probability() -> f32 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Returns a random 64-bit unsigned integer.
#[must_use]
pub fn get_random_u64() -> u64 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Appends random salt bytes to the input and encodes it to standard base64.
#[must_use]
pub fn salt_and_encode_b64(mut input: Vec<u8>) -> String {
    let count = get_random_range(1..5);
    for _ in 0..count {
        input.push(get_random_range(0..255));
    }
    b64_encode_std(input)
}

/// Converts a hex color string to an RGBA functional notation.
///
/// Supports #RRGGBB and RRGGBB formats. Falls back to original string on failure.
#[must_use]
pub fn hex_to_rgba(hex: &str, alpha: f32) -> String {
    let raw = hex.trim_start_matches('#');
    if raw.len() != 6 {
        return hex.to_string();
    }

    let r = u8::from_str_radix(&raw[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&raw[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&raw[4..6], 16).unwrap_or(0);

    format!("rgba({r}, {g}, {b}, {alpha})")
}

pub const CAPTCHA_TYPE_ROTATE: &str = "rotate";
pub const CAPTCHA_TYPE_SLIDER: &str = "slider";
pub const CAPTCHA_TYPE_PAIR: &str = "pair";
