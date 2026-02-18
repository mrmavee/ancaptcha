//! Challenge-specific validation and solution checking logic.

pub mod pair;
pub mod rotate;
pub mod slider;

pub use pair::validate_pair_solution;
pub use rotate::validate_rotate_solution;
pub use slider::validate_slider_solution;
