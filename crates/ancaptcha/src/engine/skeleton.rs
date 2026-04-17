//! Generation of the base HTML structure for the captcha.

use std::fmt::Write;

/// Parameters for generating the initial visual state of the captcha.
pub struct InitialStateConfig<'a> {
    /// Error message shown above the checkbox.
    pub error_message: Option<&'a str>,
    /// Obfuscated IDs for this render.
    pub obfuscated_ids: ObfuscatedIds<'a>,
}

/// Obfuscated identifiers used in the base HTML structure.
pub struct ObfuscatedIds<'a> {
    /// Outer container ID.
    pub container_id: &'a str,
    /// Trigger checkbox ID.
    pub checkbox_id: &'a str,
    /// Completed state checkbox ID.
    pub completed_id: &'a str,
    /// Label CSS class.
    pub label_class: &'a str,
    /// Error CSS class.
    pub error_class: &'a str,
    /// Hidden elements CSS class.
    pub hidden_class: &'a str,
    /// Token form field name.
    pub token_name: &'a str,
    /// Checkbox inner CSS class.
    pub checkbox_inner: &'a str,
    /// "I'm not a robot" text CSS class.
    pub captcha_text: &'a str,
    /// Logo wrapper CSS class.
    pub logo_wrapper: &'a str,
    /// Logo icon CSS class.
    pub logo_icon: &'a str,
}

impl Default for ObfuscatedIds<'_> {
    fn default() -> Self {
        Self {
            container_id: "a",
            checkbox_id: "checkbox",
            completed_id: "completed",
            label_class: "c",
            error_class: "e",
            hidden_class: "h",
            token_name: "_t",
            checkbox_inner: "cb",
            captcha_text: "ct",
            logo_wrapper: "lo",
            logo_icon: "li",
        }
    }
}

/// Generates the initial HTML skeleton including the trigger checkbox and labels.
#[must_use]
pub fn generate_initial_state(config: &InitialStateConfig) -> String {
    let mut html = String::with_capacity(1024);
    let ids = &config.obfuscated_ids;

    if let Some(error) = config.error_message {
        let escaped_error = html_escape::encode_text(error);
        let _ = write!(
            html,
            r#"<div class="{}">{escaped_error}</div>"#,
            ids.error_class
        );
    }

    let _ = write!(html, r#"<div id="{}">"#, ids.container_id);

    let _ = write!(
        html,
        r#"<input type="checkbox" id="{}" class="{}">"#,
        ids.checkbox_id, ids.hidden_class
    );

    let _ = write!(
        html,
        r#"<input type="checkbox" id="{}" class="{}">"#,
        ids.completed_id, ids.hidden_class
    );

    let _ = write!(
        html,
        r#"<label for="{}" class="{}">"#,
        ids.checkbox_id, ids.label_class
    );

    let _ = write!(
        html,
        r#"<span class="{}"></span><span class="{}">I'm not a robot</span><div class="{}"><div class="{}"></div>an<span>Captcha</span></div>"#,
        ids.checkbox_inner, ids.captcha_text, ids.logo_wrapper, ids.logo_icon
    );

    let _ = write!(html, r"</label>");

    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state_default() {
        let config = InitialStateConfig {
            error_message: None,
            obfuscated_ids: ObfuscatedIds {
                container_id: "a",
                checkbox_id: "checkbox",
                completed_id: "completed",
                label_class: "c",
                error_class: "e",
                hidden_class: "h",
                token_name: "_t",
                checkbox_inner: "cb",
                captcha_text: "ct",
                logo_wrapper: "lo",
                logo_icon: "li",
            },
        };

        let html = generate_initial_state(&config);

        assert!(html.contains("I'm not a robot"));
        assert!(html.contains(r#"type="checkbox""#));
        assert!(html.contains(r#"class="h""#));
    }

    #[test]
    fn initial_state_error() -> Result<(), &'static str> {
        let config = InitialStateConfig {
            error_message: Some("Verification failed. Try again."),
            obfuscated_ids: ObfuscatedIds {
                container_id: "a",
                checkbox_id: "checkbox",
                completed_id: "completed",
                label_class: "c",
                error_class: "e",
                hidden_class: "h",
                token_name: "_t",
                checkbox_inner: "cb",
                captcha_text: "ct",
                logo_wrapper: "lo",
                logo_icon: "li",
            },
        };

        let html = generate_initial_state(&config);

        assert!(html.contains("I'm not a robot"));
        assert!(html.contains("Verification failed. Try again."));
        assert!(html.contains(r#"class="e""#));

        let error_pos = html.find(r#"class="e""#).ok_or("Error class not found")?;
        let container_pos = html.find(r#"id="a""#).ok_or("Container ID not found")?;
        assert!(error_pos < container_pos);
        Ok(())
    }
    #[test]
    fn ids_obfuscation() {
        let config = InitialStateConfig {
            error_message: None,
            obfuscated_ids: ObfuscatedIds {
                container_id: "x1",
                checkbox_id: "y2",
                completed_id: "c9",
                label_class: "z3",
                error_class: "e4",
                hidden_class: "h5",
                token_name: "t6",
                checkbox_inner: "cb7",
                captcha_text: "ct8",
                logo_wrapper: "lo9",
                logo_icon: "li0",
            },
        };

        let html = generate_initial_state(&config);

        assert!(html.contains(r#"id="x1""#));
        assert!(html.contains(r#"id="y2""#));
        assert!(html.contains(r#"id="c9""#));
        assert!(html.contains(r#"class="z3""#));
        assert!(html.contains(r#"class="h5""#));
        assert!(html.contains(r#"class="cb7""#));
        assert!(html.contains(r#"class="ct8""#));
        assert!(html.contains(r#"class="lo9""#));
        assert!(html.contains(r#"class="li0""#));
    }
}
