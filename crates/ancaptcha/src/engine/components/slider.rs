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

    let ti_class = config.mapper.get_or_create("ti");
    let im_w_class = config.mapper.get_or_create("im-w");
    let st_i_class = config.mapper.get_or_create("st-i");
    let main_img_class = config.mapper.get_or_create("main-img");
    let piece_img_class = config.mapper.get_or_create("piece-img");
    let nx_class = config.mapper.get_or_create("nx");

    for step in 0..steps {
        let step_key = config.mapper.get_or_create(&format!("step{step}"));
        let _ = write!(html, r#"<div class="{stage_class} {step_key}">"#);

        let _ = write!(
            html,
            r#"<div class="{ti_class}">CLICK SLIDER TO ALIGN</div>"#
        );

        let _ = write!(html, r#"<div class="{im_w_class}">"#);
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}"));
        let piece_data_class = config.mapper.get_or_create(&format!("pdat{step}"));

        let _ = write!(
            html,
            r#"<div class="{main_img_class} {img_data_class}"></div>"#
        );

        let target_class = config.mapper.get_or_create("tg");
        let _ = write!(html, r#"<div class="{target_class} {step_key}"></div>"#);

        let _ = write!(
            html,
            r#"<div class="{piece_img_class} {step_key} {piece_data_class}"></div>"#
        );
        let _ = write!(html, r"</div>");

        if steps > 1 {
            let _ = write!(
                html,
                r#"<div class="{st_i_class}">{}/{}</div>"#,
                step + 1,
                steps
            );
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
    let piece_img_class = config.mapper.get_or_create("piece-img");

    let _ = write!(
        css,
        ".{target_class}{{position:absolute !important;width:50px !important;height:50px !important;background:rgba(0,0,0,0.6) !important;box-shadow:0 0 0 1px rgba(0,0,0,0.6),inset 0 0 5px rgba(0,0,0,0.9) !important;z-index:1 !important;}}"
    );
    let _ = write!(
        css,
        ".{piece_img_class}{{position:absolute !important;left:0 !important;width:50px !important;height:50px !important;transition:transform 0.3s !important;box-shadow:0 0 3px rgba(0,0,0,0.7) !important;z-index:2 !important;pointer-events:none !important;user-select:none !important;-webkit-user-drag:none !important;background-size:cover !important;}}"
    );

    let track_class = config.mapper.get_or_create("track");
    let track_bg = crate::common::hex_to_rgba(&text_color, 0.05);
    let _ = write!(
        css,
        ".{track_class}{{height:24px !important;background:{track_bg} !important;border-radius:12px !important;position:relative !important;margin:20px 0 !important;display:flex !important;border:1px solid {border_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{track_class} label{{flex:1 !important;height:100% !important;cursor:pointer !important;}}"
    );
    let _ = write!(
        css,
        ".{track_class} label:hover{{background:rgba(0,0,0,0.1) !important;}}"
    );

    let btn_class = config.mapper.get_or_create("btn");
    let _ = write!(
        css,
        ".{btn_class}{{display:block !important;width:100% !important;padding:10px !important;border:none !important;border-radius:4px !important;background:{accent_color} !important;color:{bg_color} !important;font-weight:bold !important;cursor:pointer !important;text-align:center !important;text-decoration:none !important;box-sizing:border-box !important;transition:background 0.2s,transform 0.1s !important;user-select:none !important;}}"
    );
    let _ = write!(css, ".{btn_class}:hover{{opacity:0.9 !important;}}");
    let _ = write!(
        css,
        ".{btn_class}:active{{transform:scale(0.98) !important;}}"
    );

    let nav_c = config.mapper.get_or_create("nc");
    let _ = write!(
        css,
        ".{nav_c}{{width:100% !important;margin-top:15px !important;}}"
    );

    let stage_class = config.mapper.get_or_create("stage");
    let _ = write!(css, ".{stage_class}{{display:none !important;}}");

    for step in 0..steps {
        let stage_radio = config.mapper.get_or_create(&format!("st{step}"));
        let step_key = config.mapper.get_or_create(&format!("step{step}"));
        let _ = write!(
            css,
            "#{stage_radio}:checked~.{stage_class}.{step_key}{{display:block !important;}}"
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
    let ti_class = config.mapper.get_or_create("ti");
    let st_i_class = config.mapper.get_or_create("st-i");
    let im_w_class = config.mapper.get_or_create("im-w");
    let main_img_class = config.mapper.get_or_create("main-img");

    let shadow_color = crate::common::hex_to_rgba(text_color, 0.2);

    let _ = write!(
        css,
        "#{trigger_id}:checked~.{modal_class}{{display:block !important;}}"
    );

    let completed_id = config.mapper.get_or_create("completed");
    let _ = write!(
        css,
        "#{completed_id}:checked~.{modal_class}{{display:none !important;}}"
    );

    let _ = write!(
        css,
        ".{modal_class}{{display:none !important;position:absolute !important;bottom:calc(100% + 10px) !important;left:50% !important;transform:translateX(-50%) !important;width:240px !important;max-width:90vw !important;background:{bg_color} !important;border-radius:4px !important;box-shadow:0 2px 10px {shadow_color} !important;padding:15px !important;z-index:1000 !important;box-sizing:border-box !important;border:1px solid {border_color} !important;}}"
    );
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
        ".{im_w_class}{{width:200px !important;height:200px !important;margin:0 auto 15px !important;position:relative !important;background:#333 !important;overflow:hidden !important;border:2px solid {accent_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{main_img_class}{{width:100% !important;height:100% !important;display:block !important;pointer-events:none !important;user-select:none !important;-webkit-user-drag:none !important;background-size:cover !important;}}"
    );
}

fn write_slider_images_css(css: &mut String, config: &mut SliderConfig) {
    for (step, img_b64) in config.images_base64.iter().enumerate() {
        let img_data_class = config.mapper.get_or_create(&format!("idat{step}"));
        let _ = write!(
            css,
            ".anc_{} .{img_data_class}{{background-image:url(data:image/jpeg;base64,{img_b64}) !important;}}",
            config.token
        );
    }

    for (step, p_b64) in config.pieces_base64.iter().enumerate() {
        let p_data_class = config.mapper.get_or_create(&format!("pdat{step}"));
        let _ = write!(
            css,
            ".anc_{} .{p_data_class}{{background-image:url(data:image/jpeg;base64,{p_b64}) !important;}}",
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
    let piece_img_class = config.mapper.get_or_create("piece-img");
    let step_key = config.mapper.get_or_create(&format!("step{step}"));

    let _ = write!(
        css,
        ".{target_class}.{step_key}{{left:{target_x}px !important;top:{y_pos}px !important;}}"
    );
    let _ = write!(
        css,
        ".{piece_img_class}.{step_key}{{top:{y_pos}px !important;}}"
    );

    for pos in 0..20 {
        let input_id = config.mapper.get_or_create(&format!("s{step}_{pos}"));
        let offset = pos * 8;
        let _ = write!(
            css,
            "#{input_id}:checked~.{stage_class}.{step_key} .{piece_img_class}{{transform:translateX({offset}px) !important;}}"
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
        assert!(html.contains(r#"type="radio""#));
        assert!(html.contains(&format!(r#"name="{t_name}""#)));
        assert!(html.contains("test_token"));
        assert!(css.contains("data:image/jpeg;base64,test1234"));
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
