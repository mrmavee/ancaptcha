//! Generation of the structural and theme-based CSS rules.

use crate::config::{Layout, Theme};
use crate::engine::skeleton::ObfuscatedIds;
use std::fmt::Write;

/// Generates the base CSS for the captcha container and trigger element.
#[must_use]
pub fn generate_base_css(
    has_error: bool,
    ids: &ObfuscatedIds,
    theme: &Theme,
    layout: &Layout,
) -> String {
    let mut css = String::with_capacity(1024);

    let width = html_escape::encode_text(&layout.width);
    let max_width = html_escape::encode_text(&layout.max_width);
    let margin = html_escape::encode_text(&layout.margin);
    let min_height = html_escape::encode_text(&layout.min_height);
    let padding = html_escape::encode_text(&layout.padding);
    let checkbox_size = html_escape::encode_text(&layout.checkbox_size);
    let font_family = html_escape::encode_text(&theme.font_family);
    let bg_color = html_escape::encode_text(&theme.background_color);
    let border_color = html_escape::encode_text(&theme.border_color);
    let text_color = html_escape::encode_text(&theme.text_color);
    let accent_color = html_escape::encode_text(&theme.accent_color);

    let _ = write!(
        css,
        "#{}{{max-width:{};width:{};margin:{};position:relative;font-family:{};}}",
        ids.container_id, max_width, width, margin, font_family
    );

    let _ = write!(
        css,
        ".{}{{display:flex;align-items:center;border:1px solid {border_color};border-radius:3px;padding:{padding};background:{bg_color};cursor:pointer;min-height:{min_height};box-sizing:border-box;transition:border-color 0.2s,box-shadow 0.2s;width:100%;}}",
        ids.label_class
    );
    let _ = write!(
        css,
        ".{}:hover{{border-color:{accent_color};box-shadow:0 1px 3px {};}}",
        ids.label_class,
        crate::common::hex_to_rgba(&text_color, 0.08)
    );

    let _ = write!(
        css,
        "#{}{{position:absolute;opacity:0;z-index:-1;pointer-events:none;}}",
        ids.checkbox_id
    );

    let _ = write!(css, ".{}{{display:none;}}", ids.hidden_class);

    let _ = write!(
        css,
        ".cb{{width:{checkbox_size};height:{checkbox_size};border:2px solid {border_color};border-radius:2px;background:{bg_color};display:inline-block;flex-shrink:0;margin-right:0.8rem;position:relative;}}"
    );
    let _ = write!(
        css,
        ".cb::after{{content:'';position:absolute;top:10%;left:35%;width:25%;height:55%;border:solid {accent_color};border-width:0 3px 3px 0;transform:rotate(45deg);display:none;}}"
    );
    let _ = write!(
        css,
        "#{}:checked+.{} .cb::after{{display:block;}}",
        ids.checkbox_id, ids.label_class
    );

    let _ = write!(
        css,
        "#{}:checked~.{} {{border-color:{accent_color};background:{};}}",
        ids.completed_id,
        ids.label_class,
        crate::common::hex_to_rgba(&accent_color, 0.05)
    );
    let _ = write!(
        css,
        "#{}:checked~.{} .cb {{background:{accent_color};border-color:{accent_color};}}",
        ids.completed_id, ids.label_class
    );

    let _ = write!(
        css,
        ".ct{{flex-grow:1;font-size:0.95rem;color:{text_color};font-weight:600;line-height:1.2;margin-top:0.1rem;}}"
    );

    let final_font = if font_family.is_empty() {
        "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif".to_string()
    } else {
        font_family.into_owned()
    };

    let _ = write!(
        css,
        ".lo{{margin-left:auto;font-family:{final_font};font-size:1.1rem;color:{text_color};letter-spacing:-0.5px;display:flex;align-items:center;opacity:0.9;white-space:nowrap;}}"
    );

    let li_shadow = format!(
        "inset 0 0 0 3px {accent_color},inset 0 0 0 5px {bg_color},inset 0 0 0 7px {}",
        crate::common::hex_to_rgba(&accent_color, 0.3)
    );
    let _ = write!(
        css,
        ".li{{width:1.25rem;height:1.25rem;border-radius:50%;margin-right:0.5rem;box-shadow:{li_shadow};flex-shrink:0;opacity:0.7;}}"
    );

    let _ = write!(css, ".lo span{{font-weight:800;color:{accent_color};}}");

    if has_error {
        let error_color = html_escape::encode_text(&theme.error_color);
        let _ = write!(
            css,
            ".{}{{color:{error_color};font-size:14px;margin-top:10px;text-align:center;font-weight:600;}}",
            ids.error_class
        );
        let _ = write!(
            css,
            "#{}{{animation:shake 0.4s cubic-bezier(.36,.07,.19,.97) both;}}",
            ids.container_id
        );
        let _ = write!(
            css,
            "@keyframes shake{{10%,90%{{transform:translateX(-1px);}}20%,80%{{transform:translateX(2px);}}30%,50%,70%{{transform:translateX(-4px);}}40%,60%{{transform:translateX(4px);}}}}"
        );
    }

    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_css_structure() {
        let ids = ObfuscatedIds::default();
        let theme = Theme::default();
        let layout = Layout::default();
        let css = generate_base_css(false, &ids, &theme, &layout);
        assert!(css.contains(&format!("#{}", ids.container_id)));
        assert!(css.contains(&format!("#{}", ids.checkbox_id)));
        assert!(css.contains(&format!(".{}", ids.label_class)));
        assert!(!css.contains("shake"));
    }

    #[test]
    fn base_css_error_state() {
        let ids = ObfuscatedIds::default();
        let theme = Theme::default();
        let layout = Layout::default();
        let css = generate_base_css(true, &ids, &theme, &layout);
        assert!(css.contains("shake"));
        assert!(css.contains("@keyframes"));
    }
}
