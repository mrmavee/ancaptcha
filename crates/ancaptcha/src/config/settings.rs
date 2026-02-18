//! Configuration models for captcha behavior and appearance.

/// Captcha challenge styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptchaStyle {
    Rotate,
    Slider,
    Pair,
}

/// Challenge difficulty levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl Difficulty {
    #[must_use]
    pub const fn steps(&self) -> u8 {
        match self {
            Self::Easy => 1,
            Self::Medium => 2,
            Self::Hard => 3,
        }
    }
}

/// Image distortion intensity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NoiseIntensity {
    Low,
    #[default]
    Medium,
    High,
}

impl NoiseIntensity {
    #[must_use]
    pub const fn jitter_amount(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 1,
            Self::High => 2,
        }
    }

    #[must_use]
    pub const fn color_shift_amount(&self) -> u8 {
        match self {
            Self::Low => 0,
            Self::Medium => 5,
            Self::High => 10,
        }
    }

    #[must_use]
    pub const fn salt_pepper_probability(&self) -> f32 {
        match self {
            Self::Low => 0.0,
            Self::Medium => 0.001,
            Self::High => 0.002,
        }
    }
}

/// Interface visual theme settings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    pub background_color: String,
    pub border_color: String,
    pub text_color: String,
    pub accent_color: String,
    pub error_color: String,
    pub font_family: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background_color: "#f9f9f9".to_string(),
            border_color: "#d3d3d3".to_string(),
            text_color: "#333333".to_string(),
            accent_color: "#14B8A6".to_string(),
            error_color: "#dc3545".to_string(),
            font_family:
                "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif"
                    .to_string(),
        }
    }
}

/// Structural layout dimensions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Layout {
    pub width: String,
    pub max_width: String,
    pub margin: String,
    pub min_height: String,
    pub padding: String,
    pub checkbox_size: String,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width: "100%".to_string(),
            max_width: "400px".to_string(),
            margin: "20px auto".to_string(),
            min_height: "3.5rem".to_string(),
            padding: "0.5rem 0.9rem".to_string(),
            checkbox_size: "1.2rem".to_string(),
        }
    }
}

use crate::common::Secret;

/// Global engine configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// 32-byte secret key for token encryption.
    /// In distributed environments, this key must be identical across all nodes
    /// to ensure stateless validation consistency.
    pub secret: Secret,
    pub difficulty: Difficulty,
    pub noise_intensity: NoiseIntensity,
    pub theme: Theme,
    pub layout: Layout,
}

impl Config {
    #[must_use]
    pub fn new(secret: Secret) -> Self {
        Self {
            secret,
            difficulty: Difficulty::Medium,
            noise_intensity: NoiseIntensity::Medium,
            theme: Theme::default(),
            layout: Layout::default(),
        }
    }

    #[must_use]
    pub const fn with_difficulty(mut self, difficulty: Difficulty) -> Self {
        self.difficulty = difficulty;
        self
    }

    #[must_use]
    pub const fn with_noise_intensity(mut self, noise_intensity: NoiseIntensity) -> Self {
        self.noise_intensity = noise_intensity;
        self
    }

    #[must_use]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    #[must_use]
    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_difficulty_check() {
        assert_eq!(Difficulty::default(), Difficulty::Medium);
    }

    #[test]
    fn difficulty_step_mapping() {
        assert_eq!(Difficulty::Easy.steps(), 1);
        assert_eq!(Difficulty::Medium.steps(), 2);
        assert_eq!(Difficulty::Hard.steps(), 3);
    }

    #[test]
    fn noise_intensity_mapping() {
        assert_eq!(NoiseIntensity::Low.jitter_amount(), 0);
        assert_eq!(NoiseIntensity::Medium.jitter_amount(), 1);
    }
}
