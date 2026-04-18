//! Submission handling and verification coordination.

pub mod factory;
pub mod resolver;
pub mod service;
pub mod submission;

pub use factory::{
    CaptchaBundle, generate_pair_bundle, generate_rotate_bundle, generate_slider_bundle,
};
pub use resolver::{unscramble_form, verify_pair, verify_rotate, verify_slider};
pub use service::AnCaptcha;
pub use submission::{PairSubmission, RotateSubmission, SliderSubmission};
