//! HTML and CSS generation for the Pair identification challenge.

use std::fmt::Write;

use crate::config::{Difficulty, Theme};
use crate::engine::NameMapper;

/// Parameters for generating a Pair challenge.
pub struct PairConfig<'a> {
    pub difficulty: Difficulty,
    pub images_base64: &'a [&'a str],
    pub correct_pairs: &'a [(u8, u8)],
    pub token: &'a str,
    pub mapper: &'a mut NameMapper,
    pub theme: &'a Theme,
}

/// Generates the HTML fragment for a Pair challenge.
#[must_use]
pub fn generate_pair_html(config: &mut PairConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_PAIR);
    let mut html = String::with_capacity(4096);

    let modal_class = config.mapper.get_or_create("modal");
    let hidden_class = config.mapper.get_or_create("h");
    let grid_class = config.mapper.get_or_create("grid");
    let btn_class = config.mapper.get_or_create("btn");
    let stage_class = config.mapper.get_or_create("stage");

    let _ = write!(html, r#"<div class="{modal_class} anc_{}">"#, config.token);

    let token_name = config.mapper.get_or_create("token");
    let _ = write!(
        html,
        r#"<input type="hidden" name="{token_name}" value="{}">"#,
        config.token
    );

    let stage_field = config.mapper.get_or_create("pst");
    for step in 0..steps {
        let stage_id = config.mapper.get_or_create(&format!("pst{step}"));
        let val_obf = config.mapper.get_or_create(&format!("pst_v{step}"));
        let checked = if step == 0 { " checked" } else { "" };
        let _ = write!(
            html,
            r#"<input type="radio" name="{stage_field}" value="{val_obf}" id="{stage_id}" class="{hidden_class}"{checked}>"#
        );
    }

    for step in 0..steps {
        for cell in 0..9 {
            let input_id = config.mapper.get_or_create(&format!("g{step}_{cell}"));
            let name_obf = config.mapper.get_or_create(&format!("s{step}_{cell}"));
            let val_obf = config.mapper.get_or_create(&format!("c{cell}"));
            let _ = write!(
                html,
                r#"<input type="checkbox" name="{name_obf}" value="{val_obf}" id="{input_id}" class="{hidden_class}">"#
            );
        }
    }

    let ti_class = config.mapper.get_or_create("ti");
    let st_i_class = config.mapper.get_or_create("st-i");
    let grid_img_class = config.mapper.get_or_create("grid-img");
    let nx_class = config.mapper.get_or_create("nx");

    for step in 0..steps {
        let step_key = config.mapper.get_or_create(&format!("pstep{step}"));
        let _ = write!(html, r#"<div class="{stage_class} {step_key}">"#);

        let _ = write!(html, r#"<div class="{ti_class}">FIND THE PAIR</div>"#);

        let _ = write!(html, r#"<div class="{grid_class}">"#);
        for cell in 0..9_usize {
            let input_id = config.mapper.get_or_create(&format!("g{step}_{cell}"));
            let img_data_class = config.mapper.get_or_create(&format!("idat{step}_{cell}"));
            let _ = write!(
                html,
                r#"<label for="{input_id}"><div class="{grid_img_class} {img_data_class}"></div></label>"#
            );
        }
        let _ = write!(html, r"</div>");

        if steps > 1 {
            let _ = write!(
                html,
                r#"<div class="{st_i_class}">{}/{}</div>"#,
                step + 1,
                steps
            );
        }

        if step < steps - 1 {
            let next_stage_id = config.mapper.get_or_create(&format!("pst{}", step + 1));
            let _ = write!(
                html,
                r#"<label for="{next_stage_id}" class="{btn_class} {nx_class}">Next</label>"#
            );
        } else {
            let completed_id = config.mapper.get_or_create("completed");
            let _ = write!(
                html,
                r#"<label for="{completed_id}" class="{btn_class} {nx_class}">Done</label>"#
            );
        }
        let _ = write!(html, r"</div>");
    }

    let _ = write!(html, r"</div>");

    html
}

/// Generates the CSS rules for a Pair challenge.
#[must_use]
pub fn generate_pair_css(config: &mut PairConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_PAIR);
    let mut css = String::with_capacity(4096);

    let bg_color = html_escape::encode_text(&config.theme.background_color);
    let border_color = html_escape::encode_text(&config.theme.border_color);
    let text_color = html_escape::encode_text(&config.theme.text_color);
    let accent_color = html_escape::encode_text(&config.theme.accent_color);

    let modal_class = config.mapper.get_or_create("modal");
    let trigger_id = config.mapper.get_or_create("checkbox");

    let grid_class = config.mapper.get_or_create("grid");
    let btn_class = config.mapper.get_or_create("btn");
    let stage_class = config.mapper.get_or_create("stage");

    let _ = write!(
        css,
        "#{trigger_id}:checked~.{modal_class}{{display:block !important;}}"
    );

    let completed_id = config.mapper.get_or_create("completed");
    let _ = write!(
        css,
        "#{completed_id}:checked~.{modal_class}{{display:none!important;}}"
    );

    let shadow_color = crate::common::hex_to_rgba(&text_color, 0.2);
    let _ = write!(
        css,
        ".{modal_class}{{display:none !important;position:absolute !important;bottom:calc(100% + 10px) !important;left:50% !important;transform:translateX(-50%) !important;width:280px !important;max-width:90vw !important;background:{bg_color} !important;border-radius:4px !important;box-shadow:0 2px 10px {shadow_color} !important;padding:15px !important;z-index:1000 !important;box-sizing:border-box !important;border:1px solid {border_color} !important;}}"
    );

    let ti_class = config.mapper.get_or_create("ti");
    let st_i_class = config.mapper.get_or_create("st-i");
    let grid_img_class = config.mapper.get_or_create("grid-img");
    let nx_class = config.mapper.get_or_create("nx");

    let _ = write!(
        css,
        ".{ti_class}{{font-size:16px !important;font-weight:bold !important;text-align:center !important;margin-bottom:15px !important;color:{text_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{st_i_class}{{font-size:14px !important;font-weight:bold !important;text-align:center !important;margin-bottom:15px !important;color:{text_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{grid_class}{{display:grid !important;grid-template-columns:repeat(3,1fr) !important;gap:10px !important;margin-bottom:15px !important;padding:2px !important;}}"
    );
    let _ = write!(
        css,
        ".{grid_class} label{{cursor:pointer !important;border:2px solid transparent !important;border-radius:4px !important;overflow:hidden !important;transition:border-color 0.2s,transform 0.2s !important;}}"
    );
    let _ = write!(
        css,
        ".{grid_class} label:hover{{border-color:{border_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{grid_class} .{grid_img_class}{{width:100% !important;height:80px !important;display:block !important;pointer-events:none !important;user-select:none !important;-webkit-user-drag:none !important;background-size:cover !important;}}"
    );

    for (idx, img_b64) in config.images_base64.iter().enumerate() {
        let step = idx / 9;
        let cell = idx % 9;
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}_{cell}"));
        let _ = write!(
            css,
            ".anc_{} .{img_data_class}{{background-image:url(data:image/jpeg;base64,{img_b64}) !important;}}",
            config.token
        );
    }

    let _ = write!(
        css,
        ".{btn_class}{{display:block !important;width:100% !important;padding:10px !important;border:none !important;border-radius:4px !important;background:{accent_color} !important;color:{bg_color} !important;font-weight:bold !important;cursor:pointer !important;text-align:center !important;text-decoration:none !important;box-sizing:border-box !important;transition:background 0.2s,transform 0.1s !important;user-select:none !important;}}"
    );
    let _ = write!(css, ".{btn_class}:hover{{opacity:0.9 !important;}}");
    let _ = write!(
        css,
        ".{btn_class}:active{{transform:scale(0.98) !important;}}"
    );
    let _ = write!(css, ".{stage_class}{{display:none !important;}}");

    for step in 0..steps {
        let stage_id = config.mapper.get_or_create(&format!("pst{step}"));
        let step_key = config.mapper.get_or_create(&format!("pstep{step}"));
        let _ = write!(
            css,
            "#{stage_id}:checked~.{step_key}{{display:block !important;}}"
        );

        for cell in 0..9 {
            let input_id = config.mapper.get_or_create(&format!("g{step}_{cell}"));
            let selection_bg = crate::common::hex_to_rgba(&accent_color, 0.1);
            let _ = write!(
                css,
                "#{input_id}:checked~.{step_key} .{grid_class} label[for=\"{input_id}\"]{{border-color:{accent_color} !important;background:{selection_bg} !important;box-shadow:inset 0 0 0 2px {accent_color} !important;transform:scale(0.92) !important;}}"
            );
        }
    }

    let _ = write!(css, ".{nx_class}{{margin-top:10px !important;}}");

    css
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_generation() {
        let mut mapper = NameMapper::new(0);
        let theme = Theme::default();
        let images = vec![
            "img1", "img2", "img1", "img3", "img4", "img5", "img6", "img7", "img8",
        ];
        let mut config = PairConfig {
            difficulty: Difficulty::Medium,
            images_base64: &images,
            correct_pairs: &[(0, 2), (1, 5)],
            token: "test_token",
            mapper: &mut mapper,
            theme: &theme,
        };

        let html = generate_pair_html(&mut config);
        let css = generate_pair_css(&mut config);
        let t_name = mapper.get_or_create("token");
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(&format!(r#"name="{t_name}""#)));
        assert!(html.contains("test_token"));
        assert!(css.contains("data:image/jpeg;base64,img1"));
    }

    #[test]
    fn css_generation() {
        let mut mapper = NameMapper::new(0);
        let theme = Theme::default();
        let images = vec!["img1"];
        let mut config = PairConfig {
            difficulty: Difficulty::Easy,
            images_base64: &images,
            correct_pairs: &[(0, 2)],
            token: "t",
            mapper: &mut mapper,
            theme: &theme,
        };

        let css = generate_pair_css(&mut config);
        assert!(css.contains("display:grid"));
        assert!(css.contains("grid-template-columns"));
    }
}
