//! Base64 encoding/decoding and color conversion utilities.

use crate::common::error::Result;
use base64::{
    Engine,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

pub fn b64_encode_url_safe<T: AsRef<[u8]>>(input: T) -> String {
    URL_SAFE_NO_PAD.encode(input)
}

/// # Errors
///
/// Fails if the input string is not valid base64.
pub fn b64_decode_url_safe<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    Ok(URL_SAFE_NO_PAD.decode(input)?)
}

pub fn b64_encode_std<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}

#[must_use]
pub fn salt_and_encode_b64(mut input: Vec<u8>) -> String {
    let count = crate::common::random::get_random_range(1..5);
    for _ in 0..count {
        input.push(crate::common::random::get_random_range(0..255));
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
