//! Shared utilities for encoding, randomization, and type definitions.

pub mod assets;
pub mod codec;
pub mod error;
pub mod random;

pub use codec::{
    b64_decode_url_safe, b64_encode_std, b64_encode_url_safe, hex_to_rgba, salt_and_encode_b64,
};
pub use random::{
    clamp_to_u8, get_random_bool, get_random_index, get_random_probability, get_random_range,
    get_random_u64,
};

pub type Secret = [u8; 32];

pub const CAPTCHA_TYPE_ROTATE: &str = "rotate";
pub const CAPTCHA_TYPE_SLIDER: &str = "slider";
pub const CAPTCHA_TYPE_PAIR: &str = "pair";
