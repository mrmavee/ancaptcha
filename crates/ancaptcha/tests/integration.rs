use ancaptcha::crypto::cipher::decrypt;
use ancaptcha::crypto::token::TokenPayload;
use ancaptcha::engine::{
    NameMapper, PairConfig, RotateConfig, SliderConfig, generate_pair_html, generate_rotate_html,
    generate_slider_html,
};
use ancaptcha::verification::unscramble_form;
use ancaptcha::{AnCaptcha, CaptchaStyle, Config, Difficulty, Layout, NoiseIntensity, init_cache};
use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

fn setup_cache() {
    let cache = init_cache();
    cache.warm_up(CaptchaStyle::Rotate);
    cache.warm_up(CaptchaStyle::Slider);
    cache.warm_up(CaptchaStyle::Pair);
}

#[test]
fn config_workflow() {
    let secret = [0u8; 32];
    let config = Config::new(secret);
    assert_eq!(config.difficulty, Difficulty::Medium);
    assert_eq!(config.noise_intensity, NoiseIntensity::Medium);

    let custom_layout = Layout {
        width: "500px".to_string(),
        ..Layout::default()
    };

    let config = Config::new([1u8; 32])
        .with_difficulty(Difficulty::Hard)
        .with_noise_intensity(NoiseIntensity::High)
        .with_layout(custom_layout);

    assert_eq!(config.difficulty, Difficulty::Hard);
    assert_eq!(config.noise_intensity, NoiseIntensity::High);
    assert_eq!(config.layout.width, "500px");
}

#[test]
fn e2e_rotate_flow() {
    setup_cache();
    let secret = [1u8; 32];
    let config = Config::new(secret).with_difficulty(Difficulty::Medium);
    let captcha = AnCaptcha::new(config);

    let bundle = captcha.generate_rotate(None).expect("gen_err");
    let token_bytes = ancaptcha::common::b64_decode_url_safe(&bundle.token).unwrap();
    let plaintext = decrypt(&secret, &token_bytes).unwrap();
    let payload = TokenPayload::from_bytes(&plaintext).unwrap();

    let mut mapper = NameMapper::new(payload.seed);
    mapper.warm_up(payload.difficulty, ancaptcha::common::CAPTCHA_TYPE_ROTATE);

    let mut form = HashMap::new();
    form.insert(mapper.get_or_create("token"), vec![bundle.token]);
    form.insert(
        mapper.get_or_create("st"),
        vec![mapper.get_or_create("st0")],
    );

    for i in 0..payload.difficulty {
        let angle_bytes: [u8; 2] = payload.solution[i as usize * 2..i as usize * 2 + 2]
            .try_into()
            .unwrap();
        let angle = u16::from_le_bytes(angle_bytes);
        let val_idx = angle / 45;
        let val_semantic = format!("v{val_idx}");
        let val_obf = mapper.get_or_create(&val_semantic);
        form.insert(mapper.get_or_create(&format!("s{i}")), vec![val_obf]);
    }

    let (payload_discovered, unscrambled, _kind) = unscramble_form(&secret, &form).unwrap();
    assert_eq!(payload_discovered, payload);

    let res = ancaptcha::RotateSubmission::from_map(&unscrambled)
        .verify(&secret, Difficulty::Medium)
        .unwrap();
    assert!(res);
}

#[test]
fn e2e_slider_flow() {
    setup_cache();
    let secret = [2u8; 32];
    let config = Config::new(secret).with_difficulty(Difficulty::Medium);
    let captcha = AnCaptcha::new(config);

    let bundle = captcha.generate_slider(None).expect("gen_err");
    let token_bytes = ancaptcha::common::b64_decode_url_safe(&bundle.token).unwrap();
    let plaintext = decrypt(&secret, &token_bytes).unwrap();
    let payload = TokenPayload::from_bytes(&plaintext).unwrap();

    let mut mapper = NameMapper::new(payload.seed);
    mapper.warm_up(payload.difficulty, ancaptcha::common::CAPTCHA_TYPE_SLIDER);

    let mut form = HashMap::new();
    form.insert(mapper.get_or_create("token"), vec![bundle.token]);
    form.insert(
        mapper.get_or_create("st"),
        vec![mapper.get_or_create("st0")],
    );

    for i in 0..payload.difficulty {
        let pos = payload.solution[i as usize * 2];
        let val_semantic = format!("p{pos}");
        let val_obf = mapper.get_or_create(&val_semantic);
        form.insert(mapper.get_or_create(&format!("s{i}")), vec![val_obf]);
    }

    let (payload_discovered, unscrambled, _kind) = unscramble_form(&secret, &form).unwrap();
    assert_eq!(payload_discovered, payload);

    let res = ancaptcha::SliderSubmission::from_map(&unscrambled)
        .verify(&secret, Difficulty::Medium)
        .unwrap();
    assert!(res);
}

#[test]
fn e2e_pair_flow() {
    setup_cache();
    let secret = [3u8; 32];
    let config = Config::new(secret).with_difficulty(Difficulty::Medium);
    let captcha = AnCaptcha::new(config);

    let bundle = captcha.generate_pair(None).expect("gen_err");
    let token_bytes = ancaptcha::common::b64_decode_url_safe(&bundle.token).unwrap();
    let plaintext = decrypt(&secret, &token_bytes).unwrap();
    let payload = TokenPayload::from_bytes(&plaintext).unwrap();

    let mut mapper = NameMapper::new(payload.seed);
    mapper.warm_up(payload.difficulty, ancaptcha::common::CAPTCHA_TYPE_PAIR);

    let mut form = HashMap::new();
    form.insert(mapper.get_or_create("token"), vec![bundle.token]);
    form.insert(
        mapper.get_or_create("pst"),
        vec![mapper.get_or_create("pst0")],
    );

    for i in 0..payload.difficulty {
        let a_idx = payload.solution[i as usize * 2];
        let b_idx = payload.solution[i as usize * 2 + 1];
        form.insert(
            mapper.get_or_create(&format!("s{i}_{a_idx}")),
            vec![mapper.get_or_create(&format!("c{a_idx}"))],
        );
        form.insert(
            mapper.get_or_create(&format!("s{i}_{b_idx}")),
            vec![mapper.get_or_create(&format!("c{b_idx}"))],
        );
    }

    let (payload_discovered, unscrambled, _kind) = unscramble_form(&secret, &form).unwrap();
    assert_eq!(payload_discovered, payload);

    let res = ancaptcha::PairSubmission::from_map(&unscrambled)
        .verify(&secret, Difficulty::Medium)
        .unwrap();
    assert!(res);
}

#[test]
fn asset_integrity_checks() {
    setup_cache();
    let secret = [4u8; 32];
    let config = Config::new(secret);
    let captcha = AnCaptcha::new(config);

    let bundle1 = captcha.generate_rotate(None).unwrap();
    let bundle2 = captcha.generate_rotate(None).unwrap();

    assert_ne!(bundle1.html, bundle2.html);
    assert_eq!(bundle1.css, "");
    assert!(bundle1.html.contains("<style>"));
    assert!(bundle1.html.contains("data:image/jpeg;base64,"));
}

#[test]
fn engine_literals() {
    let mut mapper = NameMapper::new(0);
    let theme = ancaptcha::Theme::default();

    let mut rot_cfg = RotateConfig {
        difficulty: Difficulty::Easy,
        images_base64: &["img"],
        initial_rotations: &[0],
        token: "tok",
        mapper: &mut mapper,
        theme: &theme,
        is_sprite: false,
    };
    assert!(generate_rotate_html(&mut rot_cfg).contains("value="));

    let mut sli_cfg = SliderConfig {
        difficulty: Difficulty::Easy,
        images_base64: &["img"],
        pieces_base64: &["piece"],
        correct_positions: &[0],
        token: "tok",
        mapper: &mut mapper,
        theme: &theme,
    };
    assert!(generate_slider_html(&mut sli_cfg).contains("value="));

    let mut pair_cfg = PairConfig {
        difficulty: Difficulty::Easy,
        images_base64: &["img"],
        correct_pairs: &[(0, 1)],
        token: "tok",
        mapper: &mut mapper,
        theme: &theme,
    };
    assert!(generate_pair_html(&mut pair_cfg).contains("value="));
}

#[test]
fn high_concurrency_stress() {
    setup_cache();
    let config = Config::new([5u8; 32]);
    let ancaptcha = Arc::new(AnCaptcha::new(config));

    let mut handles = Vec::new();
    for _ in 0..20 {
        let ac = Arc::clone(&ancaptcha);
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                ac.generate_rotate(None).expect("err");
                ac.generate_slider(None).expect("err");
                ac.generate_pair(None).expect("err");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("thread_panic");
    }
}

#[test]
fn token_error_handling() {
    setup_cache();
    let secret = [6u8; 32];
    let captcha = AnCaptcha::new(Config::new(secret));

    let mut form_bad = HashMap::new();
    form_bad.insert("token".to_string(), vec!["!!!".to_string()]);
    assert!(captcha.verify_request(&form_bad).is_err());

    let bundle = captcha.generate_rotate(None).expect("ERR");
    let mut form_empty = HashMap::new();
    form_empty.insert("token".to_string(), vec![bundle.token]);
    assert!(captcha.verify_request(&form_empty).is_err());
}
