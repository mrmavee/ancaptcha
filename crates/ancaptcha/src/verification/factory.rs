//! Automated construction of multi-modal captcha challenge bundles.

use crate::common::Secret;
use crate::common::error::Result;
use crate::config::{Difficulty, Layout, Theme};
use crate::engine::template::{CaptchaConfig, CaptchaRequest};
use crate::storage::get_cache;

/// Artifacts required to render and validate a captcha challenge.
pub struct CaptchaBundle {
    /// Encrypted URL-safe base64 token.
    pub token: String,
    /// Challenge HTML fragment.
    pub html: String,
    /// Challenge CSS.
    pub css: String,
}

/// Generates a Rotate style captcha bundle.
///
/// # Errors
///
/// Fails if the asset cache is missing or if token generation fails.
pub fn generate_rotate_bundle(
    secret: &Secret,
    difficulty: Difficulty,
    theme: &Theme,
    layout: &Layout,
    error_message: Option<&str>,
) -> Result<CaptchaBundle> {
    let cache = get_cache().ok_or(crate::AnCaptchaError::InvalidToken)?;
    let image_names = cache.all_image_names();
    let steps = difficulty.steps() as usize;

    let mut raw_bins: Vec<Vec<u8>> = Vec::with_capacity(steps);
    let mut initial_rotations = Vec::with_capacity(steps);
    let mut correct_angles = Vec::with_capacity(steps);

    let valid_angles = [45u16, 90, 135, 180, 225, 270, 315];

    let mut selected_indices = Vec::with_capacity(steps);
    while selected_indices.len() < steps {
        let idx = crate::common::get_random_range(0..image_names.len());
        if !selected_indices.contains(&idx) {
            selected_indices.push(idx);
        }
    }

    for name_idx in selected_indices {
        let name = image_names
            .get(name_idx)
            .cloned()
            .unwrap_or_else(|| "1".to_string());
        let bin = cache.get_image(&name).unwrap_or_default();
        raw_bins.push(bin);

        let angle_idx = crate::common::get_random_range(0..valid_angles.len());
        let init = valid_angles.get(angle_idx).copied().unwrap_or(0);
        initial_rotations.push(init);
        correct_angles.push((360 - init) % 360);
    }

    let solution: Vec<u8> = correct_angles
        .iter()
        .copied()
        .flat_map(u16::to_le_bytes)
        .collect();

    let payload = crate::crypto::token::TokenPayload::new(solution, difficulty.steps());
    let plaintext = payload.to_bytes()?;
    let encrypted = crate::crypto::cipher::encrypt(secret, &plaintext)?;
    let token = crate::common::b64_encode_url_safe(&encrypted);

    let (images_owned, is_sprite) = if steps > 1 {
        let refs: Vec<&[u8]> = raw_bins.iter().map(Vec::as_slice).collect();
        let sprite = crate::engine::stitch_horizontal(&refs, 200, 200)
            .ok_or(crate::AnCaptchaError::InvalidToken)?;
        (vec![crate::common::salt_and_encode_b64(sprite)], true)
    } else {
        let imgs: Vec<String> = raw_bins
            .into_iter()
            .map(crate::common::salt_and_encode_b64)
            .collect();
        (imgs, false)
    };

    let images_refs: Vec<&str> = images_owned.iter().map(AsRef::as_ref).collect();
    let mut request = CaptchaRequest {
        image_base64: "",
        token: &token,
        seed: payload.seed,
        difficulty,
        theme,
        layout,
        config: CaptchaConfig::Rotate {
            images_base64: &images_refs,
            initial_rotations: &initial_rotations,
            is_sprite,
        },
        error_message,
    };

    let (html, css) = crate::engine::template::generate_full_captcha(&mut request);
    Ok(CaptchaBundle { token, html, css })
}

/// Generates a Slider style captcha bundle.
///
/// # Errors
///
/// Fails if the asset cache is missing or if token generation fails.
pub fn generate_slider_bundle(
    secret: &Secret,
    difficulty: Difficulty,
    theme: &Theme,
    layout: &Layout,
    error_message: Option<&str>,
) -> Result<CaptchaBundle> {
    let cache = get_cache().ok_or(crate::AnCaptchaError::InvalidToken)?;
    let image_names = cache.all_image_names();
    let steps = difficulty.steps() as usize;

    let mut selected_indices = Vec::with_capacity(steps);
    while selected_indices.len() < steps {
        let idx = crate::common::get_random_range(0..image_names.len());
        if !selected_indices.contains(&idx) {
            selected_indices.push(idx);
        }
    }

    let mut main_bins: Vec<Vec<u8>> = Vec::with_capacity(steps);
    let mut piece_bins: Vec<Vec<u8>> = Vec::with_capacity(steps);
    let mut solution = Vec::with_capacity(steps * 2);

    for idx in selected_indices {
        let name = image_names
            .get(idx)
            .cloned()
            .unwrap_or_else(|| "1".to_string());
        let (x_idx, y_pos, m, p) = cache
            .get_slider_pair(&name)
            .ok_or(crate::AnCaptchaError::InvalidToken)?;
        main_bins.push(m);
        piece_bins.push(p);
        solution.push(x_idx);
        solution.push(y_pos);
    }

    let payload = crate::crypto::token::TokenPayload::new(solution.clone(), difficulty.steps());
    let plaintext = payload.to_bytes()?;
    let encrypted = crate::crypto::cipher::encrypt(secret, &plaintext)?;
    let token = crate::common::b64_encode_url_safe(&encrypted);

    let main_refs: Vec<&[u8]> = main_bins.iter().map(Vec::as_slice).collect();
    let pc_refs: Vec<&[u8]> = piece_bins.iter().map(Vec::as_slice).collect();

    let main_sprite = crate::engine::stitch_vertical(&main_refs, 200, 200)
        .ok_or(crate::AnCaptchaError::InvalidToken)?;
    let piece_sprite = crate::engine::stitch_vertical(&pc_refs, 50, 50)
        .ok_or(crate::AnCaptchaError::InvalidToken)?;

    let main_b64 = crate::common::salt_and_encode_b64(main_sprite);
    let piece_b64 = crate::common::salt_and_encode_b64(piece_sprite);

    let images_owned = [main_b64];
    let pieces_owned = [piece_b64];
    let images_refs: Vec<&str> = images_owned.iter().map(AsRef::as_ref).collect();
    let pieces_refs: Vec<&str> = pieces_owned.iter().map(AsRef::as_ref).collect();

    let mut request = CaptchaRequest {
        image_base64: "",
        token: &token,
        seed: payload.seed,
        difficulty,
        theme,
        layout,
        config: CaptchaConfig::Slider {
            images_base64: &images_refs,
            pieces_base64: &pieces_refs,
            correct_positions: &solution,
        },
        error_message,
    };

    let (html, css) = crate::engine::template::generate_full_captcha(&mut request);

    Ok(CaptchaBundle { token, html, css })
}

/// Generates a Find the Pair style captcha bundle.
///
/// # Errors
///
/// Fails if the asset cache is missing or if token generation fails.
pub fn generate_pair_bundle(
    secret: &Secret,
    difficulty: Difficulty,
    theme: &Theme,
    layout: &Layout,
    error_message: Option<&str>,
) -> Result<CaptchaBundle> {
    use rand::seq::SliceRandom;
    let cache = get_cache().ok_or(crate::AnCaptchaError::InvalidToken)?;
    let image_names = cache.all_image_names();
    let steps = difficulty.steps() as usize;
    let mut rng = rand::rng();

    let mut pairs = Vec::with_capacity(steps);
    let mut step_sprites: Vec<String> = Vec::with_capacity(steps);

    for _ in 0..steps {
        let a = crate::common::get_random_range(0..9u8);
        let mut b = crate::common::get_random_range(0..9u8);
        while a == b {
            b = crate::common::get_random_range(0..9u8);
        }
        pairs.push((a, b));

        let mut unique_binaries = Vec::with_capacity(8);
        let mut shuffled_names = image_names.to_vec();
        shuffled_names.shuffle(&mut rng);

        for name in shuffled_names {
            if let Some(img) = cache
                .get_pair_image(&name)
                .filter(|img| !unique_binaries.contains(img))
            {
                unique_binaries.push(img);
            }
            if unique_binaries.len() >= 8 {
                break;
            }
        }

        while unique_binaries.len() < 8 {
            if let Some(img) = unique_binaries.first().cloned() {
                unique_binaries.push(img);
            } else {
                unique_binaries.push(Vec::new());
            }
        }

        let mut grid: Vec<Vec<u8>> = vec![Vec::new(); 9];
        let target = unique_binaries.first().cloned().unwrap_or_default();
        if let Some(cell) = grid.get_mut(a as usize) {
            cell.clone_from(&target);
        }
        if let Some(cell) = grid.get_mut(b as usize) {
            *cell = target;
        }

        let mut dist_idx = 1;
        for (i, cell) in grid.iter_mut().enumerate() {
            if i != a as usize && i != b as usize {
                if let Some(dist) = unique_binaries.get(dist_idx) {
                    cell.clone_from(dist);
                } else if let Some(first) = unique_binaries.first() {
                    cell.clone_from(first);
                }
                dist_idx += 1;
            }
        }

        let grid_refs: Vec<&[u8]> = grid.iter().map(Vec::as_slice).collect();
        let sprite = crate::engine::stitch_grid(&grid_refs, 100, 3)
            .ok_or(crate::AnCaptchaError::InvalidToken)?;
        step_sprites.push(crate::common::salt_and_encode_b64(sprite));
    }

    let solution: Vec<u8> = pairs.iter().flat_map(|(a, b)| vec![*a, *b]).collect();

    let payload = crate::crypto::token::TokenPayload::new(solution, difficulty.steps());
    let plaintext = payload.to_bytes()?;
    let encrypted = crate::crypto::cipher::encrypt(secret, &plaintext)?;
    let token = crate::common::b64_encode_url_safe(&encrypted);

    let images_refs: Vec<&str> = step_sprites.iter().map(AsRef::as_ref).collect();

    let mut request = CaptchaRequest {
        image_base64: "",
        token: &token,
        seed: payload.seed,
        difficulty,
        theme,
        layout,
        config: CaptchaConfig::Pair {
            images_base64: &images_refs,
            correct_pairs: &pairs,
        },
        error_message,
    };

    let (html, css) = crate::engine::template::generate_full_captcha(&mut request);

    Ok(CaptchaBundle { token, html, css })
}
