//! HTML and CSS generation for the image rotation challenge.

use std::fmt::Write;

use crate::config::{Difficulty, Theme};
use crate::engine::NameMapper;

/// Parameters for generating a Rotate challenge.
pub struct RotateConfig<'a> {
    pub difficulty: Difficulty,
    pub images_base64: &'a [&'a str],
    pub initial_rotations: &'a [u16],
    pub token: &'a str,
    pub mapper: &'a mut NameMapper,
    pub theme: &'a Theme,
    pub is_sprite: bool,
}

/// Generates the HTML fragment for a Rotate challenge.
#[must_use]
pub fn generate_rotate_html(config: &mut RotateConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_ROTATE);
    let mut html = String::with_capacity(4096);

    let modal_class = config.mapper.get_or_create("modal");
    let hidden_class = config.mapper.get_or_create("h");
    let stage_class = config.mapper.get_or_create("stage");

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
        for idx in 0..8 {
            let input_id = config.mapper.get_or_create(&format!("r{step}_{idx}"));
            let val_obf = config.mapper.get_or_create(&format!("v{idx}"));
            let checked = if idx == 0 { " checked" } else { "" };

            let _ = write!(
                html,
                r#"<input type="radio" name="{step_field}" value="{val_obf}" id="{input_id}" class="{hidden_class}"{checked}>"#
            );
        }
    }

    for step in 0..steps {
        write_rotate_step_html(&mut html, config, step, steps, &stage_class);
    }

    let _ = write!(html, r"</div>");

    html
}

fn write_rotate_step_html(
    html: &mut String,
    config: &mut RotateConfig,
    step: u8,
    steps: u8,
    stage_class: &str,
) {
    let btn_class = config.mapper.get_or_create("btn");
    let wrapper_class = config.mapper.get_or_create("ir-w");
    let ti_class = config.mapper.get_or_create("ti");
    let im_w_class = config.mapper.get_or_create("im-w");
    let st_i_class = config.mapper.get_or_create("st-i");
    let rt_class = config.mapper.get_or_create("rt");
    let nx_class = config.mapper.get_or_create("nx");

    let step_key = config.mapper.get_or_create(&format!("step{step}"));
    let _ = write!(html, r#"<div class="{stage_class} {step_key}">"#);
    let _ = write!(html, r#"<div class="{ti_class}">ROTATE THE PIC</div>"#);

    let _ = write!(html, r#"<div class="{im_w_class}">"#);
    let initial_rot_class = config.mapper.get_or_create(&format!("ir{step}"));
    let _ = write!(html, r#"<div class="{wrapper_class} {initial_rot_class}">"#);

    let img_id_obf = config.mapper.get_or_create(&format!("img{step}"));
    let img_data_class = config.mapper.get_or_create(&format!("id{step}"));
    let extra_class = if config.is_sprite {
        format!(" {}", config.mapper.get_or_create("ispr"))
    } else {
        String::new()
    };
    let _ = write!(
        html,
        r#"<div id="{img_id_obf}" class="{img_data_class}{extra_class}"></div>"#
    );
    let _ = write!(html, r"</div></div>");

    if steps > 1 {
        let _ = write!(
            html,
            r#"<div class="{st_i_class}">{}/{}</div>"#,
            step + 1,
            steps
        );
    }

    let controls_class = config.mapper.get_or_create("ctrl");
    let _ = write!(html, r#"<div class="{controls_class}">"#);
    let rot_c = config.mapper.get_or_create("rc");
    let _ = write!(html, r#"<div class="{rot_c}">"#);
    for idx in 0..8 {
        let next_idx = (idx + 1) % 8;
        let next_id = config.mapper.get_or_create(&format!("r{step}_{next_idx}"));
        let label_class = config.mapper.get_or_create(&format!("l{step}_{idx}"));
        let _ = write!(
            html,
            r#"<label for="{next_id}" class="{label_class} {btn_class} {rt_class}">Rotate</label>"#
        );
    }
    let _ = write!(html, "</div>");

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
    let _ = write!(html, "</div>");
    let _ = write!(html, r"</div></div>");
}

/// Generates the CSS rules for a Rotate challenge.
#[must_use]
pub fn generate_rotate_css(config: &mut RotateConfig) -> String {
    let steps = config.difficulty.steps();
    config
        .mapper
        .warm_up(steps, crate::common::CAPTCHA_TYPE_ROTATE);
    let mut css = String::with_capacity(8192);

    let bg_color = html_escape::encode_text(&config.theme.background_color);
    let border_color = html_escape::encode_text(&config.theme.border_color);
    let text_color = html_escape::encode_text(&config.theme.text_color);
    let accent_color = html_escape::encode_text(&config.theme.accent_color);

    let modal_class = config.mapper.get_or_create("modal");
    let trigger_id = config.mapper.get_or_create("checkbox");

    let btn_class = config.mapper.get_or_create("btn");
    let stage_class = config.mapper.get_or_create("stage");
    let controls_class = config.mapper.get_or_create("ctrl");
    let wrapper_class = config.mapper.get_or_create("ir-w");

    let _ = write!(
        css,
        "#{trigger_id}:checked~.{modal_class}{{display:block !important;}}"
    );

    let completed_id = config.mapper.get_or_create("completed");
    let _ = write!(
        css,
        "#{completed_id}:checked~.{modal_class}{{display:none !important;}}"
    );

    let shadow_color = crate::common::hex_to_rgba(&text_color, 0.2);
    let _ = write!(
        css,
        ".{modal_class}{{display:none !important;position:absolute !important;bottom:calc(100% + 10px) !important;left:50% !important;transform:translateX(-50%) !important;width:280px !important;max-width:90vw !important;background:{bg_color} !important;border-radius:4px !important;box-shadow:0 2px 10px {shadow_color} !important;padding:15px !important;z-index:1000 !important;box-sizing:border-box !important;border:1px solid {border_color} !important;}}"
    );

    let ti_class = config.mapper.get_or_create("ti");
    let im_w_class = config.mapper.get_or_create("im-w");
    let st_i_class = config.mapper.get_or_create("st-i");
    let rt_class = config.mapper.get_or_create("rt");
    let nx_class = config.mapper.get_or_create("nx");

    let _ = write!(
        css,
        ".{ti_class}{{font-size:16px !important;font-weight:bold !important;text-align:center !important;margin-bottom:15px !important;color:{text_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{im_w_class}{{width:120px !important;height:120px !important;margin:0 auto 10px !important;border-radius:50% !important;overflow:hidden !important;border:3px solid {accent_color} !important;position:relative !important;}}"
    );
    let _ = write!(
        css,
        ".{st_i_class}{{font-size:14px !important;font-weight:bold !important;text-align:center !important;margin-bottom:15px !important;color:{text_color} !important;}}"
    );
    let _ = write!(
        css,
        ".{wrapper_class}{{width:100% !important;height:100% !important;transition:transform 0.3s !important;}}"
    );

    if config.is_sprite && steps > 1 {
        write_rotate_sprite_css(&mut css, config);
    } else {
        write_rotate_individual_css(&mut css, config);
    }

    write_rotate_controls_css(
        &mut css,
        config.theme,
        &controls_class,
        &btn_class,
        &nx_class,
    );

    let _ = write!(css, ".{stage_class}{{display:none !important;}}");
    let _ = write!(
        css,
        ".{controls_class} .{rt_class}{{display:none !important;}}"
    );

    for step in 0..steps {
        write_rotate_step_css(&mut css, config, step, &stage_class);
    }

    css
}

fn write_rotate_individual_css(css: &mut String, config: &mut RotateConfig) {
    for (step, img_b64) in config.images_base64.iter().enumerate() {
        let img_data_class = config.mapper.get_or_create(&format!("id{step}"));
        let _ = write!(
            css,
            ".anc_{} .{img_data_class}{{width:100% !important;height:100% !important;background-image:url(data:image/jpeg;base64,{img_b64}) !important;background-size:cover !important;transition:transform 0.3s !important;pointer-events:none !important;user-select:none !important;-webkit-user-drag:none !important;}}",
            config.token
        );
    }
}

fn write_rotate_sprite_css(css: &mut String, config: &mut RotateConfig) {
    let steps = config.difficulty.steps();
    let sprite_b64 = config.images_base64.first().copied().unwrap_or("");
    let cell_w = 120;
    let sprite_class = config.mapper.get_or_create("ispr");

    let _ = write!(
        css,
        ".anc_{} .{sprite_class}{{width:100% !important;height:100% !important;background-image:url(data:image/jpeg;base64,{sprite_b64}) !important;background-size:{}px 100% !important;transition:transform 0.3s !important;pointer-events:none !important;user-select:none !important;-webkit-user-drag:none !important;}}",
        config.token,
        i32::from(steps) * cell_w
    );

    for step in 0..steps {
        let img_data_class = config.mapper.get_or_create(&format!("id{step}"));
        let x_off = i32::from(step) * -cell_w;
        let _ = write!(
            css,
            ".{img_data_class}{{background-position:{x_off}px 0 !important;}}"
        );
    }
}

fn write_rotate_controls_css(
    css: &mut String,
    theme: &Theme,
    controls_class: &str,
    btn_class: &str,
    nx_class: &str,
) {
    let bg = html_escape::encode_text(&theme.background_color);
    let bc = html_escape::encode_text(&theme.border_color);
    let tc = html_escape::encode_text(&theme.text_color);
    let ac = html_escape::encode_text(&theme.accent_color);

    let _ = write!(
        css,
        ".{controls_class}{{display:flex !important;flex-direction:column !important;gap:10px !important;width:100% !important;}}"
    );
    let _ = write!(
        css,
        ".{btn_class}{{display:block !important;width:100% !important;padding:10px !important;border:none !important;border-radius:4px !important;background:{bg} !important;color:{tc} !important;border:1px solid {bc} !important;font-weight:bold !important;cursor:pointer !important;text-align:center !important;box-sizing:border-box !important;text-decoration:none !important;transition:background 0.2s,transform 0.1s !important;user-select:none !important;}}"
    );
    let _ = write!(css, ".{btn_class}:hover{{opacity:0.8 !important;}}");
    let _ = write!(
        css,
        ".{btn_class}:active{{transform:scale(0.98) !important;}}"
    );
    let _ = write!(
        css,
        ".{nx_class}{{background:{ac} !important;color:{bg} !important;border:none !important;}}"
    );
    let _ = write!(css, ".{nx_class}:hover{{opacity:0.9 !important;}}");
    let _ = write!(
        css,
        ".{nx_class}:active{{transform:scale(0.98) !important;}}"
    );
}

fn write_rotate_step_css(css: &mut String, config: &mut RotateConfig, step: u8, stage_class: &str) {
    let stage_radio = config.mapper.get_or_create(&format!("st{step}"));
    let step_key = config.mapper.get_or_create(&format!("step{step}"));
    let _ = write!(
        css,
        "#{stage_radio}:checked~.{stage_class}.{step_key}{{display:block !important;}}"
    );

    let initial_rot = config.initial_rotations.get(step as usize).unwrap_or(&0);
    let initial_rot_class = config.mapper.get_or_create(&format!("ir{step}"));
    let _ = write!(
        css,
        ".{initial_rot_class}{{transform:rotate({initial_rot}deg) !important;}}"
    );

    let rt_class = config.mapper.get_or_create("rt");
    let im_w_class = config.mapper.get_or_create("im-w");

    for idx in 0..8 {
        let angle = idx * 45;
        let input_id = config.mapper.get_or_create(&format!("r{step}_{idx}"));
        let label_class = config.mapper.get_or_create(&format!("l{step}_{idx}"));
        let img_id_obf = config.mapper.get_or_create(&format!("img{step}"));

        let _ = write!(
            css,
            "#{input_id}:checked~.{stage_class}.{step_key} .{im_w_class} div#{img_id_obf}{{transform:rotate({angle}deg) !important;}}"
        );
        let _ = write!(
            css,
            "#{input_id}:checked~.{stage_class}.{step_key} .{rt_class}.{label_class} {{display:block !important;}}"
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
        let images = vec!["img1", "img2"];
        let mut config = RotateConfig {
            difficulty: Difficulty::Medium,
            images_base64: &images,
            initial_rotations: &[90, 180],
            token: "test_token",
            mapper: &mut mapper,
            theme: &theme,
            is_sprite: false,
        };

        let html = generate_rotate_html(&mut config);
        let css = generate_rotate_css(&mut config);
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
        let mut config = RotateConfig {
            difficulty: Difficulty::Easy,
            images_base64: &images,
            initial_rotations: &[45],
            token: "t",
            mapper: &mut mapper,
            theme: &theme,
            is_sprite: false,
        };

        let css = generate_rotate_css(&mut config);
        assert!(css.contains("transform:rotate(45deg)"));
        assert!(css.contains("display:block"));
    }

    #[test]
    fn sprite_css() {
        let mut mapper = NameMapper::new(0);
        let theme = Theme::default();
        let images = vec!["sprite_data"];
        let mut config = RotateConfig {
            difficulty: Difficulty::Medium,
            images_base64: &images,
            initial_rotations: &[90, 180],
            token: "t",
            mapper: &mut mapper,
            theme: &theme,
            is_sprite: true,
        };

        let css = generate_rotate_css(&mut config);
        assert!(css.contains("background-position"));
        assert!(css.contains("sprite_data"));
    }
}
