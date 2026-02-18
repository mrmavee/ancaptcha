//! Generation of the base HTML structure for the captcha.

use std::fmt::Write;

/// Parameters for generating the initial visual state of the captcha.
pub struct InitialStateConfig<'a> {
    pub error_message: Option<&'a str>,
    pub obfuscated_ids: ObfuscatedIds<'a>,
}

/// Obfuscated identifiers used in the base HTML structure.
pub struct ObfuscatedIds<'a> {
    pub container_id: &'a str,
    pub checkbox_id: &'a str,
    pub completed_id: &'a str,
    pub label_class: &'a str,
    pub error_class: &'a str,
    pub hidden_class: &'a str,
    pub token_name: &'a str,
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

    let _ = write!(html, r#"<div id="{}" class="an-c">"#, ids.container_id);

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
        r#"<span class="cb"></span><span class="ct">I'm not a robot</span><div class="lo"><div class="li"></div>an<span>Captcha</span></div>"#
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
            },
        };

        let html = generate_initial_state(&config);

        assert!(html.contains("I'm not a robot"));
        assert!(html.contains(r#"type="checkbox""#));
        assert!(!html.contains(r#"class="e""#));
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
            },
        };

        let html = generate_initial_state(&config);

        assert!(html.contains(r#"id="x1""#));
        assert!(html.contains(r#"id="y2""#));
        assert!(html.contains(r#"id="c9""#));
        assert!(html.contains(r#"class="z3""#));
        assert!(html.contains(r#"class="h5""#));
    }
}
