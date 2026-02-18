//! HTML and CSS generation for the puzzle slider challenge.

use std::fmt::Write;

use crate::config::{Difficulty, Theme};
use crate::engine::NameMapper;

/// Parameters for generating a Slider challenge.
pub struct SliderConfig<'a> {
    pub difficulty: Difficulty,
    pub images_base64: &'a [&'a str],
    pub pieces_base64: &'a [&'a str],
    pub correct_positions: &'a [u8],
    pub token: &'a str,
    pub mapper: &'a mut NameMapper,
    pub theme: &'a Theme,
}

/// Generates the HTML fragment for a Slider challenge.
#[must_use]
pub fn generate_slider_html(config: &mut SliderConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_SLIDER);
    let mut html = String::with_capacity(4096);

    let modal_class = config.mapper.get_or_create("modal");
    let hidden_class = config.mapper.get_or_create("h");
    let btn_class = config.mapper.get_or_create("btn");
    let stage_class = config.mapper.get_or_create("stage");
    let track_class = config.mapper.get_or_create("track");

    let _ = write!(html, r#"<div class="{modal_class} anc_{}">"#, config.token);

    let token_name = config.mapper.get_or_create("token");
    let _ = write!(
        html,
        r#"<input type="hidden" name="{token_name}" value="{}">"#,
        config.token
    );

    let stage_field = config.mapper.get_or_create("st");
    for step in 0..steps {
        let stage_id = config.mapper.get_or_create(&format!("st{step}"));
        let val_obf = config.mapper.get_or_create(&format!("st_v{step}"));
        let checked = if step == 0 { " checked" } else { "" };
        let _ = write!(
            html,
            r#"<input type="radio" name="{stage_field}" value="{val_obf}" id="{stage_id}" class="{hidden_class}"{checked}>"#
        );
    }

    for step in 0..steps {
        let step_field = config.mapper.get_or_create(&format!("s{step}"));
        for pos in 0..20 {
            let input_id = config.mapper.get_or_create(&format!("s{step}_{pos}"));
            let val_obf = config.mapper.get_or_create(&format!("p{pos}"));
            let checked = if pos == 0 { " checked" } else { "" };
            let _ = write!(
                html,
                r#"<input type="radio" name="{step_field}" value="{val_obf}" id="{input_id}" class="{hidden_class}"{checked}>"#
            );
        }
    }

    for step in 0..steps {
        let _ = write!(html, r#"<div class="{stage_class} s{step}">"#);

        let _ = write!(html, r#"<div class="ti">CLICK SLIDER TO ALIGN</div>"#);

        let _ = write!(html, r#"<div class="im-w">"#);
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}"));
        let piece_data_class = config.mapper.get_or_create(&format!("pdat{step}"));

        let _ = write!(html, r#"<div class="main-img {img_data_class}"></div>"#);

        let target_class = config.mapper.get_or_create("tg");
        let _ = write!(html, r#"<div class="{target_class} s{step}"></div>"#);

        let _ = write!(
            html,
            r#"<div class="piece-img s{step} {piece_data_class}"></div>"#
        );
        let _ = write!(html, r"</div>");

        if steps > 1 {
            let _ = write!(html, r#"<div class="st-i">{}/{}</div>"#, step + 1, steps);
        }

        let _ = write!(html, r#"<div class="{track_class}">"#);
        for pos in 0..20 {
            let input_id = config.mapper.get_or_create(&format!("s{step}_{pos}"));
            let _ = write!(html, r#"<label for="{input_id}"></label>"#);
        }
        let _ = write!(html, r"</div>");

        let nav_c = config.mapper.get_or_create("nc");
        let _ = write!(html, r#"<div class="{nav_c}">"#);
        if step < steps - 1 {
            let next_stage_id = config.mapper.get_or_create(&format!("st{}", step + 1));
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
        let _ = write!(html, r"</div>");
    }

    let _ = write!(html, r"</div>");

    html
}

/// Generates the CSS rules for a Slider challenge.
#[must_use]
pub fn generate_slider_css(config: &mut SliderConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_SLIDER);
    let mut css = String::with_capacity(8192);

    let bg_color = html_escape::encode_text(&config.theme.background_color);
    let border_color = html_escape::encode_text(&config.theme.border_color);
    let text_color = html_escape::encode_text(&config.theme.text_color);
    let accent_color = html_escape::encode_text(&config.theme.accent_color);

    write_slider_base_css(
        &mut css,
        config,
        &bg_color,
        &border_color,
        &text_color,
        &accent_color,
    );

    write_slider_images_css(&mut css, config);

    let target_class = config.mapper.get_or_create("tg");
    let _ = write!(
        css,
        ".{target_class}{{position:absolute;width:50px;height:50px;background:rgba(0,0,0,0.4);border-radius:2px;box-shadow:inset 0 0 8px #fff,0 0 5px rgba(255,255,255,0.5);z-index:1;}}"
    );
    let _ = write!(
        css,
        ".piece-img{{position:absolute;left:0;width:50px;height:50px;transition:transform 0.3s;filter:drop-shadow(0 0 3px #fff) drop-shadow(0 0 5px rgba(255,255,255,0.5));z-index:2;pointer-events:none;user-select:none;-webkit-user-drag:none;background-size:cover;}}"
    );

    let track_class = config.mapper.get_or_create("track");
    let track_bg = crate::common::hex_to_rgba(&text_color, 0.05);
    let _ = write!(
        css,
        ".{track_class}{{height:24px;background:{track_bg};border-radius:12px;position:relative;margin:20px 0;display:flex;border:1px solid {border_color};}}"
    );
    let _ = write!(
        css,
        ".{track_class} label{{flex:1;height:100%;cursor:pointer;}}"
    );
    let _ = write!(
        css,
        ".{track_class} label:hover{{background:rgba(0,0,0,0.1);}}"
    );

    let btn_class = config.mapper.get_or_create("btn");
    let _ = write!(
        css,
        ".{btn_class}{{display:block;width:100%;padding:10px;border:none;border-radius:4px;background:{accent_color};color:{bg_color};font-weight:bold;cursor:pointer;text-align:center;text-decoration:none;box-sizing:border-box;transition:background 0.2s,transform 0.1s;user-select:none;}}"
    );
    let _ = write!(css, ".{btn_class}:hover{{opacity:0.9;}}");
    let _ = write!(css, ".{btn_class}:active{{transform:scale(0.98);}}");

    let nav_c = config.mapper.get_or_create("nc");
    let _ = write!(css, ".{nav_c}{{width:100%;margin-top:15px;}}");

    let stage_class = config.mapper.get_or_create("stage");
    let _ = write!(css, ".{stage_class}{{display:none;}}");

    for step in 0..steps {
        let stage_radio = config.mapper.get_or_create(&format!("st{step}"));
        let _ = write!(
            css,
            "#{stage_radio}:checked~.{stage_class}.s{step}{{display:block;}}"
        );

        write_step_css(&mut css, config, step, &stage_class);
    }

    css
}

fn write_slider_base_css(
    css: &mut String,
    config: &mut SliderConfig,
    bg_color: &str,
    border_color: &str,
    text_color: &str,
    accent_color: &str,
) {
    let modal_class = config.mapper.get_or_create("modal");
    let trigger_id = config.mapper.get_or_create("checkbox");

    let shadow_color = crate::common::hex_to_rgba(text_color, 0.2);

    let _ = write!(
        css,
        "#{trigger_id}:checked~.{modal_class}{{display:block;}}"
    );

    let completed_id = config.mapper.get_or_create("completed");
    let _ = write!(
        css,
        "#{completed_id}:checked~.{modal_class}{{display:none!important;}}"
    );

    let _ = write!(
        css,
        ".{modal_class}{{display:none;position:absolute;bottom:calc(100% + 10px);left:50%;transform:translateX(-50%);width:240px;max-width:90vw;background:{bg_color};border-radius:4px;box-shadow:0 2px 10px {shadow_color};padding:15px;z-index:1000;box-sizing:border-box;border:1px solid {border_color};}}"
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
        ".im-w{{width:200px;height:200px;margin:0 auto 15px;position:relative;background:#333;overflow:hidden;border:2px solid {accent_color};}}"
    );
    let _ = write!(
        css,
        ".main-img{{width:100%;height:100%;display:block;pointer-events:none;user-select:none;-webkit-user-drag:none;background-size:cover;}}"
    );
}

fn write_slider_images_css(css: &mut String, config: &mut SliderConfig) {
    for (step, img_b64) in config.images_base64.iter().enumerate() {
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}"));
        let _ = write!(
            css,
            ".anc_{} .{img_data_class}{{background-image:url(data:image/jpeg;base64,{img_b64});}}",
            config.token
        );
    }

    for (step, p_b64) in config.pieces_base64.iter().enumerate() {
        let p_data_class = config.mapper.get_or_create(&format!("pdat{step}"));
        let _ = write!(
            css,
            ".anc_{} .{p_data_class}{{background-image:url(data:image/jpeg;base64,{p_b64});}}",
            config.token
        );
    }
}

fn write_step_css(css: &mut String, config: &mut SliderConfig, step: u8, stage_class: &str) {
    let x_idx = config
        .correct_positions
        .get(step as usize * 2)
        .copied()
        .unwrap_or(0);
    let y_pos = config
        .correct_positions
        .get(step as usize * 2 + 1)
        .copied()
        .unwrap_or(140);
    let target_x = x_idx * 8;

    let target_class = config.mapper.get_or_create("tg");
    let _ = write!(
        css,
        ".{target_class}.s{step}{{left:{target_x}px;top:{y_pos}px;}}"
    );
    let _ = write!(css, ".piece-img.s{step}{{top:{y_pos}px;}}");

    for pos in 0..20 {
        let input_id = config.mapper.get_or_create(&format!("s{step}_{pos}"));
        let offset = pos * 8;
        let _ = write!(
            css,
            "#{input_id}:checked~.{stage_class}.s{step} .piece-img{{transform:translateX({offset}px);}}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn html_generation() {
        let mut mapper = NameMapper::new(0);
        let theme = Theme::default();
        let mut config = SliderConfig {
            difficulty: Difficulty::Medium,
            images_base64: &["test1234"],
            pieces_base64: &["piece1234"],
            correct_positions: &[5, 140, 7, 140],
            token: "test_token",
            mapper: &mut mapper,
            theme: &theme,
        };

        let html = generate_slider_html(&mut config);
        let css = generate_slider_css(&mut config);
        let t_name = mapper.get_or_create("token");
        assert!(css.contains("data:image/jpeg;base64,test1234"));
        assert!(css.contains("data:image/jpeg;base64,piece1234"));
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(&format!(r#"name="{t_name}""#)));
        assert!(html.contains("test_token"));
    }

    #[test]
    fn css_generation() {
        let mut mapper = NameMapper::new(0);
        let theme = Theme::default();
        let mut config = SliderConfig {
            difficulty: Difficulty::Easy,
            images_base64: &["test"],
            pieces_base64: &["piece"],
            correct_positions: &[3, 140],
            token: "t",
            mapper: &mut mapper,
            theme: &theme,
        };

        let css = generate_slider_css(&mut config);
        assert!(css.contains("transform:translateX("));
        assert!(css.contains("display:block"));
    }
}
