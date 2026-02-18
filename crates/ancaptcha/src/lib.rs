//! anCaptcha: A No-JS, stateless captcha engine implemented in Rust for cross-language integration.
//!
//! Originally designed for the darknet, specifically for Tor hidden services, to provide human 
//! verification without requiring JavaScript. It uses authenticated encryption (ChaCha20-Poly1305) 
//! to store challenge state in tokens, eliminating the need for server-side sessions.
//!
//! (c) 2026 Maverick. Licensed under Apache License, Version 2.0.
#![forbid(unsafe_code)]
#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

pub mod common;
pub mod config;
pub mod crypto;
pub mod engine;
pub mod storage;
pub mod styles;
pub mod verification;

pub use common::assets;
pub use common::error::{AnCaptchaError, Result};
pub use config::{CaptchaStyle, Config, Difficulty, Layout, NoiseIntensity, Theme};
pub use crypto::token::DEFAULT_TTL_SECONDS;
pub use engine::{CaptchaRequest, generate_full_captcha};
pub use storage::{AssetCache, get_cache, init_cache, init_with_intensity};
pub use verification::{
    AnCaptcha, CaptchaBundle, PairSubmission, RotateSubmission, SliderSubmission,
    generate_pair_bundle, generate_rotate_bundle, generate_slider_bundle, verify_pair,
    verify_rotate, verify_slider,
};
