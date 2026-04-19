//! anCaptcha FFI: C-compatible bridge for the anCaptcha engine.
//!
//! C-ABI for integrating stateless human verification using
//! interactive CSS into foreign languages. Built for Tor Browser's
//! "Safest" mode where JavaScript is restricted.
//!
//! (c) 2026 Maverick. Licensed under Apache License, Version 2.0.

#![warn(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::fn_to_numeric_cast_any,
    clippy::multiple_unsafe_ops_per_block
)]
#![deny(improper_ctypes)]
#![deny(improper_ctypes_definitions)]

mod core;

pub use core::config::{
    anCaptcha_set_config, anCaptcha_set_layout, anCaptcha_set_theme, anCaptcha_warm_up,
};
pub use core::error::{
    anCaptcha_free_string, anCaptcha_last_error_length, anCaptcha_last_error_message,
};
pub use core::generate::{
    anCaptcha_generate_pair, anCaptcha_generate_rotate, anCaptcha_generate_slider,
};
pub use core::utils::Status;
pub use core::verify::{
    anCaptcha_verify_auto, anCaptcha_verify_pair, anCaptcha_verify_rotate, anCaptcha_verify_slider,
};
