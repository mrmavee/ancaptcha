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

    let w = html_escape::encode_text(&layout.width);
    let mw = html_escape::encode_text(&layout.max_width);
    let m = html_escape::encode_text(&layout.margin);
    let ff = html_escape::encode_text(&theme.font_family);

    let _ = write!(
        css,
        "#{id}{{max-width:{mw} !important;width:{w} !important;margin:{m} !important;position:relative !important;font-family:{ff} !important;}}",
        id = ids.container_id
    );

    let _ = write!(
        css,
        "#{id} *,#{id} *::before,#{id} *::after{{box-sizing:border-box !important;text-transform:none !important;text-decoration:none !important;}}",
        id = ids.container_id
    );

    append_trigger_css(&mut css, ids, theme, layout);
    append_brand_css(&mut css, ids, theme);

    if has_error {
        append_error_css(&mut css, ids, theme);
    }

    css
}

fn append_trigger_css(css: &mut String, ids: &ObfuscatedIds, theme: &Theme, layout: &Layout) {
    let h = html_escape::encode_text(&layout.min_height);
    let p = html_escape::encode_text(&layout.padding);
    let cs = html_escape::encode_text(&layout.checkbox_size);
    let bg = html_escape::encode_text(&theme.background_color);
    let bc = html_escape::encode_text(&theme.border_color);
    let tc = html_escape::encode_text(&theme.text_color);
    let ac = html_escape::encode_text(&theme.accent_color);

    let _ = write!(
        css,
        ".{}{{display:flex !important;align-items:center !important;border:1px solid {bc} !important;border-radius:3px !important;padding:{p} !important;background:{bg} !important;cursor:pointer !important;min-height:{h} !important;box-sizing:border-box !important;transition:border-color 0.2s,box-shadow 0.2s !important;width:100% !important;}}",
        ids.label_class
    );
    let _ = write!(
        css,
        ".{}:hover{{border-color:{ac} !important;box-shadow:0 1px 3px {} !important;}}",
        ids.label_class,
        crate::common::hex_to_rgba(&tc, 0.08)
    );
    let _ = write!(
        css,
        "#{}{{position:absolute !important;opacity:0 !important;z-index:-1 !important;pointer-events:none !important;width:0 !important;height:0 !important;}}",
        ids.checkbox_id
    );
    let _ = write!(css, ".{}{{display:none !important;}}", ids.hidden_class);

    let _ = write!(
        css,
        ".{}{{width:{cs} !important;height:{cs} !important;border:2px solid {bc} !important;border-radius:2px !important;background:{bg} !important;display:inline-block !important;flex-shrink:0 !important;margin-right:0.8rem !important;position:relative !important;margin-top:0 !important;margin-bottom:0 !important;margin-left:0 !important;}}",
        ids.checkbox_inner
    );
    let _ = write!(
        css,
        ".{}::after{{content:'' !important;position:absolute !important;top:10% !important;left:35% !important;width:25% !important;height:55% !important;border:solid {ac} !important;border-width:0 3px 3px 0 !important;transform:rotate(45deg) !important;display:none !important;}}",
        ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}:checked+.{} .{}::after{{display:block !important;}}",
        ids.checkbox_id, ids.label_class, ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}:checked~.{} {{border-color:{ac} !important;background:{} !important;}}",
        ids.completed_id,
        ids.label_class,
        crate::common::hex_to_rgba(&ac, 0.05)
    );
    let _ = write!(
        css,
        "#{}:checked~.{} .{} {{background:{ac} !important;border-color:{ac} !important;}}",
        ids.completed_id, ids.label_class, ids.checkbox_inner
    );
}

fn append_brand_css(css: &mut String, ids: &ObfuscatedIds, theme: &Theme) {
    let ff = html_escape::encode_text(&theme.font_family);
    let bg = html_escape::encode_text(&theme.background_color);
    let tc = html_escape::encode_text(&theme.text_color);
    let ac = html_escape::encode_text(&theme.accent_color);

    let _ = write!(
        css,
        ".{}{{flex-grow:1 !important;font-size:0.95rem !important;color:{tc} !important;font-weight:600 !important;line-height:1.2 !important;margin-top:0.1rem !important;}}",
        ids.captcha_text
    );
    let eff = if ff.is_empty() {
        "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif".into()
    } else {
        ff
    };
    let _ = write!(
        css,
        ".{}{{margin-left:auto !important;font-family:{eff} !important;font-size:1.1rem !important;color:{tc} !important;letter-spacing:-0.5px !important;display:flex !important;align-items:center !important;opacity:0.9 !important;white-space:nowrap !important;}}",
        ids.logo_wrapper
    );
    let lsh = format!(
        "inset 0 0 0 3px {ac},inset 0 0 0 5px {bg},inset 0 0 0 7px {}",
        crate::common::hex_to_rgba(&ac, 0.3)
    );
    let _ = write!(
        css,
        ".{}{{width:1.25rem !important;height:1.25rem !important;border-radius:50% !important;margin-right:0.5rem !important;box-shadow:{lsh} !important;flex-shrink:0 !important;opacity:0.7 !important;}}",
        ids.logo_icon
    );
    let _ = write!(
        css,
        ".{} span{{font-weight:800 !important;color:{ac} !important;display:inline !important;}}",
        ids.logo_wrapper
    );
}

fn append_error_css(css: &mut String, ids: &ObfuscatedIds, theme: &Theme) {
    let ec = html_escape::encode_text(&theme.error_color);
    let _ = write!(
        css,
        ".{}{{color:{ec} !important;font-size:14px !important;margin-top:10px !important;text-align:center !important;font-weight:600 !important;}}",
        ids.error_class
    );
    let _ = write!(
        css,
        "#{}{{animation:shake 0.4s cubic-bezier(.36,.07,.19,.97) both !important;}}",
        ids.container_id
    );
    let _ = write!(
        css,
        "@keyframes shake{{10%,90%{{transform:translateX(-1px);}}20%,80%{{transform:translateX(2px);}}30%,50%,70%{{transform:translateX(-4px);}}40%,60%{{transform:translateX(4px);}}}}"
    );
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
