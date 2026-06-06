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
    size: &crate::config::Size,
) -> String {
    let mut css = String::with_capacity(1024);

    let w = html_escape::encode_text(&layout.width);
    let mw = html_escape::encode_text(&layout.max_width);
    let m = html_escape::encode_text(&layout.margin);
    let ff = html_escape::encode_text(&theme.font_family);

    let base_fs = size.base_font_size();
    let _ = write!(
        css,
        "#{id}{{font-size:{base_fs}px !important;max-width:{mw} !important;width:{w} !important;margin:{m} !important;position:relative !important;font-family:{ff} !important;}}",
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
        ".{}{{display:flex !important;align-items:center !important;border:0.0625em solid {bc} !important;border-radius:0.1875em !important;padding:{p} !important;background:{bg} !important;cursor:pointer !important;min-height:{h} !important;box-sizing:border-box !important;transition:border-color 0.2s,box-shadow 0.2s !important;width:100% !important;}}",
        ids.label_class
    );
    let _ = write!(
        css,
        ".{}:hover{{border-color:{ac} !important;box-shadow:0 0.0625em 0.1875em {} !important;}}",
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
        ".{}{{width:{cs} !important;height:{cs} !important;border:0.125em solid {bc} !important;border-radius:0.125em !important;background:{bg} !important;display:inline-block !important;flex-shrink:0 !important;margin-right:0.8em !important;position:relative !important;margin-top:0 !important;margin-bottom:0 !important;margin-left:0 !important;}}",
        ids.checkbox_inner
    );
    let _ = write!(css, "@keyframes spin{{to{{transform:rotate(360deg);}}}}");
    let _ = write!(
        css,
        ".{}::before{{content:'' !important;position:absolute !important;top:0 !important;left:0 !important;right:0 !important;bottom:0 !important;margin:auto !important;width:70% !important;height:70% !important;border:0.125em solid {bc} !important;border-top-color:{ac} !important;border-radius:50% !important;animation:spin 0.8s linear infinite !important;display:none !important;}}",
        ids.checkbox_inner
    );
    let _ = write!(
        css,
        ".{}::after{{content:'' !important;position:absolute !important;top:45% !important;left:50% !important;width:30% !important;height:60% !important;border:solid {bg} !important;border-width:0 0.1875em 0.1875em 0 !important;transform:translate(-50%,-50%) rotate(45deg) !important;display:none !important;}}",
        ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}:checked~.{} .{}::before{{display:block !important;}}",
        ids.checkbox_id, ids.label_class, ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}:checked~.{} .{}::before{{display:none !important;}}",
        ids.completed_id, ids.label_class, ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}:checked~.{} .{}::after{{display:block !important;}}",
        ids.completed_id, ids.label_class, ids.checkbox_inner
    );
    let tint = crate::common::hex_to_rgba(&ac, 0.05);
    let _ = write!(
        css,
        "#{}:checked~.{} {{border-color:{ac} !important;background-color:{bg} !important;background-image:linear-gradient({tint},{tint}) !important;}}",
        ids.completed_id, ids.label_class
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
        ".{}{{flex-grow:1 !important;font-size:0.95em !important;color:{tc} !important;font-weight:600 !important;line-height:1.2 !important;margin-top:0.1em !important;}}",
        ids.captcha_text
    );
    let eff = if ff.is_empty() {
        "-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Helvetica,Arial,sans-serif".into()
    } else {
        ff
    };
    let _ = write!(
        css,
        ".{}{{margin-left:auto !important;font-family:{eff} !important;font-size:1.1em !important;color:{tc} !important;letter-spacing:-0.03em !important;display:flex !important;align-items:center !important;opacity:0.9 !important;white-space:nowrap !important;}}",
        ids.logo_wrapper
    );
    let lsh = format!(
        "inset 0 0 0 0.1875em {ac},inset 0 0 0 0.3125em {bg},inset 0 0 0 0.4375em {}",
        crate::common::hex_to_rgba(&ac, 0.3)
    );
    let _ = write!(
        css,
        ".{}{{width:1.25em !important;height:1.25em !important;border-radius:50% !important;margin-right:0.5em !important;box-shadow:{lsh} !important;flex-shrink:0 !important;opacity:0.7 !important;}}",
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
        ".{}{{color:{ec} !important;font-size:0.875em !important;margin-top:0.625em !important;text-align:center !important;font-weight:600 !important;}}",
        ids.error_class
    );
    let _ = write!(
        css,
        ".{} .{} {{border-color:{ec} !important;}}",
        ids.label_class, ids.checkbox_inner
    );
    let _ = write!(
        css,
        "#{}{{animation:shake 0.4s cubic-bezier(.36,.07,.19,.97) both !important;}}",
        ids.container_id
    );
    let _ = write!(
        css,
        "@keyframes shake{{10%,90%{{transform:translateX(-0.0625em);}}20%,80%{{transform:translateX(0.125em);}}30%,50%,70%{{transform:translateX(-0.25em);}}40%,60%{{transform:translateX(0.25em);}}}}"
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
        let size = crate::config::Size::default();
        let css = generate_base_css(false, &ids, &theme, &layout, &size);
        assert!(css.contains(&format!("#{}", ids.container_id)));
        assert!(css.contains(&format!("#{}", ids.checkbox_id)));
        assert!(css.contains(&format!(".{}", ids.label_class)));
    }

    #[test]
    fn base_css_error_state() {
        let ids = ObfuscatedIds::default();
        let theme = Theme::default();
        let layout = Layout::default();
        let size = crate::config::Size::default();
        let css = generate_base_css(true, &ids, &theme, &layout, &size);
        assert!(css.contains("shake"));
        assert!(css.contains("@keyframes"));
    }
}
