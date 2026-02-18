//! Style-specific challenge component generation.

pub mod pair;
pub mod rotate;
pub mod slider;

pub use pair::{PairConfig, generate_pair_css, generate_pair_html};
pub use rotate::{RotateConfig, generate_rotate_css, generate_rotate_html};
pub use slider::{SliderConfig, generate_slider_css, generate_slider_html};
