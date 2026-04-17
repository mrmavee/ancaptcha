//! Submission handling and verification coordination.

pub mod factory;
pub mod service;
pub mod submission;
pub mod validator;

pub use factory::{
    CaptchaBundle, generate_pair_bundle, generate_rotate_bundle, generate_slider_bundle,
};
pub use service::AnCaptcha;
pub use submission::{PairSubmission, RotateSubmission, SliderSubmission};
pub use validator::{unscramble_form, verify_pair, verify_rotate, verify_slider};
