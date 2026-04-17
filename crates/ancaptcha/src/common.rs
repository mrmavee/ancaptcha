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

/// 32-byte key for ChaCha20-Poly1305 token encryption.
pub type Secret = [u8; 32];

/// Identifier for the rotation challenge type.
pub const CAPTCHA_TYPE_ROTATE: &str = "rotate";
/// Identifier for the slider challenge type.
pub const CAPTCHA_TYPE_SLIDER: &str = "slider";
/// Identifier for the pair identification challenge type.
pub const CAPTCHA_TYPE_PAIR: &str = "pair";
