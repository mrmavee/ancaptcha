//! Visual asset distortion and HTML/CSS generation engine.

pub mod components;
pub mod css;
pub mod minify;
pub mod noise;
pub mod obfuscator;
pub mod processor;
pub mod skeleton;
pub mod template;

pub use crate::config::NoiseIntensity;
pub use components::{
    PairConfig, RotateConfig, SliderConfig, generate_pair_css, generate_pair_html,
    generate_rotate_css, generate_rotate_html, generate_slider_css, generate_slider_html,
};
pub use minify::{minify_css, minify_html};
pub use noise::{apply_color_shift, apply_full_noise, apply_pixel_jitter, apply_salt_pepper};
pub use obfuscator::NameMapper;
pub use processor::{create_slider_cutout, stitch_grid, stitch_horizontal, stitch_vertical};
pub use skeleton::generate_initial_state;
pub use template::{CaptchaRequest, generate_full_captcha};
