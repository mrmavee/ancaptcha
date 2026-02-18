//! Minification utilities for generated HTML and CSS content.

/// Reduces HTML string size by removing whitespace and newlines.
#[must_use]
pub fn minify_html(html: &str) -> String {
    html.replace('\n', "")
        .replace("> ", ">")
        .replace(" <", "<")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .replace("> <", "><")
}

/// Reduces CSS string size by removing whitespace and newlines.
#[must_use]
pub fn minify_css(css: &str) -> String {
    css.replace('\n', "")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
        .replace(" {", "{")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("} ", "}")
        .replace("; ", ";")
        .replace(": ", ":")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_minification() {
        let html = r#"<div class="test">
            <span>  I'm not a robot  </span>
        </div>"#;
        let minified = minify_html(html);
        assert!(minified.contains("I'm not a robot"));
        assert!(!minified.contains('\n'));
        assert!(!minified.contains("  "));
        assert!(!minified.contains("> <"));
    }

    #[test]
    fn css_minification() {
        let css = ".test {
            color: #333333;
            margin: 0   10px;
            border: 1px solid #d3d3d3;
        }";
        let minified = minify_css(css);
        assert!(minified.contains("0 10px"));
        assert!(minified.contains("#333333"));
        assert!(minified.contains("1px solid #d3d3d3"));
        assert!(!minified.contains('\n'));
        assert!(!minified.contains("  "));
    }
}
