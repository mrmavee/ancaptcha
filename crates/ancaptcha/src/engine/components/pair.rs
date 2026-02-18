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

    for step in 0..steps {
        let _ = write!(html, r#"<div class="{stage_class} ps{step}">"#);

        let _ = write!(html, r#"<div class="ti">FIND THE PAIR</div>"#);

        let _ = write!(html, r#"<div class="{grid_class}">"#);
        for cell in 0..9_usize {
            let input_id = config.mapper.get_or_create(&format!("g{step}_{cell}"));
            let img_data_class = config.mapper.get_or_create(&format!("idat{step}_{cell}"));
            let _ = write!(
                html,
                r#"<label for="{input_id}"><div class="grid-img {img_data_class}"></div></label>"#
            );
        }
        let _ = write!(html, r"</div>");

        if steps > 1 {
            let _ = write!(html, r#"<div class="st-i">{}/{}</div>"#, step + 1, steps);
        }

        if step < steps - 1 {
            let next_stage_id = config.mapper.get_or_create(&format!("pst{}", step + 1));
            let _ = write!(
                html,
                r#"<label for="{next_stage_id}" class="{btn_class} nx">Next</label>"#
            );
        } else {
            let completed_id = config.mapper.get_or_create("completed");
            let _ = write!(
                html,
                r#"<label for="{completed_id}" class="{btn_class} nx">Done</label>"#
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
        "#{trigger_id}:checked~.{modal_class}{{display:block;}}"
    );

    let completed_id = config.mapper.get_or_create("completed");
    let _ = write!(
        css,
        "#{completed_id}:checked~.{modal_class}{{display:none!important;}}"
    );

    let shadow_color = crate::common::hex_to_rgba(&text_color, 0.2);
    let _ = write!(
        css,
        ".{modal_class}{{display:none;position:absolute;bottom:calc(100% + 10px);left:50%;transform:translateX(-50%);width:280px;max-width:90vw;background:{bg_color};border-radius:4px;box-shadow:0 2px 10px {shadow_color};padding:15px;z-index:1000;box-sizing:border-box;border:1px solid {border_color};}}"
    );
    let _ = write!(
        css,
        ".ti{{font-size:16px;font-weight:bold;text-align:center;margin-bottom:15px;color:{text_color};}}"
    );
    let _ = write!(
        css,
        ".st-i{{font-size:14px;font-weight:bold;text-align:center;margin-bottom:15px;color:{text_color};}}"
    );
    let _ = write!(
        css,
        ".{grid_class}{{display:grid;grid-template-columns:repeat(3,1fr);gap:10px;margin-bottom:15px;padding:2px;}}"
    );
    let _ = write!(
        css,
        ".{grid_class} label{{cursor:pointer;border:2px solid transparent;border-radius:4px;overflow:hidden;transition:border-color 0.2s,transform 0.2s;}}"
    );
    let _ = write!(
        css,
        ".{grid_class} label:hover{{border-color:{border_color};}}"
    );
    let _ = write!(
        css,
        ".{grid_class} .grid-img{{width:100%;height:80px;display:block;pointer-events:none;user-select:none;-webkit-user-drag:none;background-size:cover;}}"
    );

    for (idx, img_b64) in config.images_base64.iter().enumerate() {
        let step = idx / 9;
        let cell = idx % 9;
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}_{cell}"));
        let _ = write!(
            css,
            ".anc_{} .{img_data_class}{{background-image:url(data:image/jpeg;base64,{img_b64});}}",
            config.token
        );
    }

    let _ = write!(
        css,
        ".{btn_class}{{display:block;width:100%;padding:10px;border:none;border-radius:4px;background:{accent_color};color:{bg_color};font-weight:bold;cursor:pointer;text-align:center;text-decoration:none;box-sizing:border-box;transition:background 0.2s,transform 0.1s;user-select:none;}}"
    );
    let _ = write!(css, ".{btn_class}:hover{{opacity:0.9;}}");
    let _ = write!(css, ".{btn_class}:active{{transform:scale(0.98);}}");
    let _ = write!(css, ".{stage_class}{{display:none;}}");

    for step in 0..steps {
        let stage_id = config.mapper.get_or_create(&format!("pst{step}"));
        let _ = write!(css, "#{stage_id}:checked~.ps{step}{{display:block;}}");

        for cell in 0..9 {
            let input_id = config.mapper.get_or_create(&format!("g{step}_{cell}"));
            let selection_bg = crate::common::hex_to_rgba(&accent_color, 0.1);
            let _ = write!(
                css,
                "#{input_id}:checked~.ps{step} .{grid_class} label[for=\"{input_id}\"]{{border-color:{accent_color};background:{selection_bg};box-shadow:inset 0 0 0 2px {accent_color};transform:scale(0.92);}}"
            );
        }
    }

    let _ = write!(css, ".nx{{margin-top:10px;}}");

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
