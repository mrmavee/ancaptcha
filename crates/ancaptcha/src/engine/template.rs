//! Coordination of HTML/CSS challenge generation.

use crate::config::{Difficulty, Layout, Theme};
use crate::engine::css::generate_base_css;
use crate::engine::skeleton::{InitialStateConfig, ObfuscatedIds};
use crate::engine::{
    NameMapper, PairConfig, RotateConfig, SliderConfig, generate_initial_state, generate_pair_css,
    generate_pair_html, generate_rotate_css, generate_rotate_html, generate_slider_css,
    generate_slider_html, minify_css, minify_html,
};

/// Per-challenge config for the template renderer.
pub enum CaptchaConfig<'a> {
    /// Rotation challenge.
    Rotate {
        /// Images in base64 (single or sprite).
        images_base64: &'a [&'a str],
        /// Initial rotation per image (degrees).
        initial_rotations: &'a [u16],
        /// True if images form a horizontal sprite.
        is_sprite: bool,
    },
    /// Slider challenge.
    Slider {
        /// Background images with cutout in base64.
        images_base64: &'a [&'a str],
        /// Puzzle pieces in base64.
        pieces_base64: &'a [&'a str],
        /// Correct slot index per step.
        correct_positions: &'a [u8],
    },
    /// Pair matching challenge.
    Pair {
        /// Per-step grid sprite in base64.
        images_base64: &'a [&'a str],
        /// Correct pairs per step.
        correct_pairs: &'a [(u8, u8)],
    },
}

/// Input for `generate_full_captcha`.
pub struct CaptchaRequest<'a> {
    /// Base64 image (FFI passthrough).
    pub image_base64: &'a str,
    /// Encrypted token embedded in the form.
    pub token: &'a str,
    /// Seed for name obfuscation.
    pub seed: u64,
    /// Challenge difficulty.
    pub difficulty: Difficulty,
    /// Active theme.
    pub theme: &'a Theme,
    /// Layout dimensions.
    pub layout: &'a Layout,
    /// Challenge config.
    pub config: CaptchaConfig<'a>,
    /// Error message shown above the form.
    pub error_message: Option<&'a str>,
}

/// Orchestrates the generation of a complete captcha challenge (HTML and CSS).
///
/// The returned HTML and CSS are tightly coupled through obfuscated identifiers.
/// They must be rendered together in the same response to ensure visual and
/// functional integrity of the challenge.
#[must_use]
pub fn generate_full_captcha(request: &mut CaptchaRequest) -> (String, String) {
    let mut mapper = NameMapper::new(request.seed);

    let container_id = mapper.get_or_create("container");
    let checkbox_id = mapper.get_or_create("checkbox");
    let completed_id = mapper.get_or_create("completed");
    let label_class = mapper.get_or_create("label");
    let error_class = mapper.get_or_create("error");
    let hidden_class = mapper.get_or_create("h");
    let token_name = mapper.get_or_create("token");
    let checkbox_inner = mapper.get_or_create("cb");
    let captcha_text = mapper.get_or_create("ct");
    let logo_wrapper = mapper.get_or_create("lo");
    let logo_icon = mapper.get_or_create("li");

    let ids = ObfuscatedIds {
        container_id: &container_id,
        checkbox_id: &checkbox_id,
        completed_id: &completed_id,
        label_class: &label_class,
        error_class: &error_class,
        hidden_class: &hidden_class,
        token_name: &token_name,
        checkbox_inner: &checkbox_inner,
        captcha_text: &captcha_text,
        logo_wrapper: &logo_wrapper,
        logo_icon: &logo_icon,
    };

    let initial_config = InitialStateConfig {
        error_message: request.error_message,
        obfuscated_ids: ids,
    };

    let initial_html = generate_initial_state(&initial_config);
    let base_css = generate_base_css(
        request.error_message.is_some(),
        &initial_config.obfuscated_ids,
        request.theme,
        request.layout,
    );

    let (type_html, type_css) = match &request.config {
        CaptchaConfig::Rotate {
            images_base64,
            initial_rotations,
            is_sprite,
        } => {
            let mut config = RotateConfig {
                difficulty: request.difficulty,
                images_base64,
                initial_rotations,
                token: request.token,
                mapper: &mut mapper,
                theme: request.theme,
                is_sprite: *is_sprite,
            };
            (
                generate_rotate_html(&mut config),
                generate_rotate_css(&mut config),
            )
        }
        CaptchaConfig::Slider {
            images_base64,
            pieces_base64,
            correct_positions,
        } => {
            let mut config = SliderConfig {
                difficulty: request.difficulty,
                images_base64,
                pieces_base64,
                correct_positions,
                token: request.token,
                mapper: &mut mapper,
                theme: request.theme,
            };
            (
                generate_slider_html(&mut config),
                generate_slider_css(&mut config),
            )
        }
        CaptchaConfig::Pair {
            images_base64,
            correct_pairs,
        } => {
            let mut config = PairConfig {
                difficulty: request.difficulty,
                images_base64,
                correct_pairs,
                token: request.token,
                mapper: &mut mapper,
                theme: request.theme,
            };
            (
                generate_pair_html(&mut config),
                generate_pair_css(&mut config),
            )
        }
    };

    let full_html = format!("{initial_html}{type_html}</div>");
    let full_css = format!("{base_css}{type_css}");

    let minified_html = minify_html(&full_html);
    let minified_css = minify_css(&full_css);

    let combined_html = format!("<style>{minified_css}</style>{minified_html}");

    (combined_html, String::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_rotate_generation() {
        let theme = Theme::default();
        let layout = Layout::default();
        let mut request = CaptchaRequest {
            image_base64: "img",
            token: "tok",
            seed: 0,
            difficulty: Difficulty::Medium,
            theme: &theme,
            layout: &layout,
            config: CaptchaConfig::Rotate {
                images_base64: &["a", "b"],
                initial_rotations: &[0, 1],
                is_sprite: false,
            },
            error_message: None,
        };

        let (html, css) = generate_full_captcha(&mut request);
        assert!(css.is_empty());
        assert!(html.contains("<style>"));
        assert!(html.contains("data:image/jpeg;base64,a"));
        assert!(html.contains("data:image/jpeg;base64,b"));
    }

    #[test]
    fn full_rotate_sprite() {
        let theme = Theme::default();
        let layout = Layout::default();
        let mut request = CaptchaRequest {
            image_base64: "img",
            token: "tok",
            seed: 0,
            difficulty: Difficulty::Medium,
            theme: &theme,
            layout: &layout,
            config: CaptchaConfig::Rotate {
                images_base64: &["sprite_data"],
                initial_rotations: &[0, 1],
                is_sprite: true,
            },
            error_message: None,
        };

        let (html, css) = generate_full_captcha(&mut request);
        assert!(css.is_empty());
        assert!(html.contains("<style>"));
        assert!(html.contains("data:image/jpeg;base64,sprite_data"));
        assert!(html.contains("background-position"));
    }

    #[test]
    fn full_slider_generation() {
        let theme = Theme::default();
        let layout = Layout::default();
        let mut request = CaptchaRequest {
            image_base64: "img",
            token: "tok",
            seed: 0,
            difficulty: Difficulty::Easy,
            theme: &theme,
            layout: &layout,
            config: CaptchaConfig::Slider {
                images_base64: &["main"],
                pieces_base64: &["piece"],
                correct_positions: &[5, 140],
            },
            error_message: None,
        };

        let (html, css) = generate_full_captcha(&mut request);
        assert!(css.is_empty());
        assert!(html.contains("<style>"));
        assert!(html.contains("data:image/jpeg;base64,main"));
        assert!(html.contains("data:image/jpeg;base64,piece"));
        assert!(html.contains("transform:translateX("));
    }

    #[test]
    fn full_pair_generation() {
        let theme = Theme::default();
        let layout = Layout::default();
        let mut request = CaptchaRequest {
            image_base64: "img",
            token: "tok",
            seed: 0,
            difficulty: Difficulty::Easy,
            theme: &theme,
            layout: &layout,
            config: CaptchaConfig::Pair {
                images_base64: &["sprite1"],
                correct_pairs: &[(0, 1)],
            },
            error_message: None,
        };

        let (html, css) = generate_full_captcha(&mut request);
        assert!(css.is_empty());
        assert!(html.contains("<style>"));
        assert!(html.contains("data:image/jpeg;base64,sprite1"));
        assert!(html.contains("display:grid"));
    }
}
