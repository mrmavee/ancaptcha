//! C-compatible captcha bundle generation interfaces.

use crate::core::config::{get_layout, get_secret, get_theme};
use crate::core::error::anCaptcha_free_string;
use crate::core::utils::{Status, catch_panic, set_error, to_c_string};
use libc::c_char;
use std::ffi::CStr;
use tracing::error;

const fn map_difficulty(val: i32) -> ancaptcha::Difficulty {
    match val {
        0 => ancaptcha::Difficulty::Easy,
        1 => ancaptcha::Difficulty::Medium,
        _ => ancaptcha::Difficulty::Hard,
    }
}

#[unsafe(no_mangle)]
/// Generates a Rotate style captcha bundle.
///
/// # Safety
///
/// `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
pub unsafe extern "C" fn anCaptcha_generate_rotate(
    difficulty: i32,
    error_msg: *const c_char,
    token_out: *mut *mut c_char,
    html_out: *mut *mut c_char,
    css_out: *mut *mut c_char,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        let err_str = if error_msg.is_null() {
            None
        } else {
            let c_str = unsafe { CStr::from_ptr(error_msg) };
            Some(c_str.to_string_lossy())
        };

        let diff = map_difficulty(difficulty);
        let theme = get_theme();
        let layout = get_layout();

        match ancaptcha::generate_rotate_bundle(secret, diff, &theme, &layout, err_str.as_deref()) {
            Ok(bundle) => {
                let t_ptr = to_c_string(bundle.token);
                let h_ptr = to_c_string(bundle.html);
                let c_ptr = to_c_string(bundle.css);

                if t_ptr.is_null() || h_ptr.is_null() || c_ptr.is_null() {
                    unsafe { anCaptcha_free_string(t_ptr) };
                    unsafe { anCaptcha_free_string(h_ptr) };
                    unsafe { anCaptcha_free_string(c_ptr) };
                    return Status::ErrMemory as i32;
                }

                if !token_out.is_null() {
                    unsafe { *token_out = t_ptr };
                }
                if !html_out.is_null() {
                    unsafe { *html_out = h_ptr };
                }
                if !css_out.is_null() {
                    unsafe { *css_out = c_ptr };
                }
                Status::Ok as i32
            }
            Err(e) => {
                error!("Rotate generation error: {e:?}");
                set_error("Failed to generate captcha");
                Status::ErrMemory as i32
            }
        }
    })
}

#[unsafe(no_mangle)]
/// Generates a Slider style captcha bundle.
///
/// # Safety
///
/// `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
pub unsafe extern "C" fn anCaptcha_generate_slider(
    difficulty: i32,
    error_msg: *const c_char,
    token_out: *mut *mut c_char,
    html_out: *mut *mut c_char,
    css_out: *mut *mut c_char,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        let err_str = if error_msg.is_null() {
            None
        } else {
            let c_str = unsafe { CStr::from_ptr(error_msg) };
            Some(c_str.to_string_lossy())
        };

        let diff = map_difficulty(difficulty);
        let theme = get_theme();
        let layout = get_layout();

        match ancaptcha::generate_slider_bundle(secret, diff, &theme, &layout, err_str.as_deref()) {
            Ok(bundle) => {
                let t_ptr = to_c_string(bundle.token);
                let h_ptr = to_c_string(bundle.html);
                let c_ptr = to_c_string(bundle.css);

                if t_ptr.is_null() || h_ptr.is_null() || c_ptr.is_null() {
                    unsafe { anCaptcha_free_string(t_ptr) };
                    unsafe { anCaptcha_free_string(h_ptr) };
                    unsafe { anCaptcha_free_string(c_ptr) };
                    return Status::ErrMemory as i32;
                }

                if !token_out.is_null() {
                    unsafe { *token_out = t_ptr };
                }
                if !html_out.is_null() {
                    unsafe { *html_out = h_ptr };
                }
                if !css_out.is_null() {
                    unsafe { *css_out = c_ptr };
                }
                Status::Ok as i32
            }
            Err(e) => {
                error!("Slider generation error: {e:?}");
                set_error("Failed to generate captcha");
                Status::ErrMemory as i32
            }
        }
    })
}

#[unsafe(no_mangle)]
/// Generates a Find the Pair style captcha bundle.
///
/// # Safety
///
/// `token_out`, `html_out`, and `css_out` must be valid pointers to string pointers.
pub unsafe extern "C" fn anCaptcha_generate_pair(
    difficulty: i32,
    error_msg: *const c_char,
    token_out: *mut *mut c_char,
    html_out: *mut *mut c_char,
    css_out: *mut *mut c_char,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        let err_str = if error_msg.is_null() {
            None
        } else {
            let c_str = unsafe { CStr::from_ptr(error_msg) };
            Some(c_str.to_string_lossy())
        };

        let diff = map_difficulty(difficulty);
        let theme = get_theme();
        let layout = get_layout();

        match ancaptcha::generate_pair_bundle(secret, diff, &theme, &layout, err_str.as_deref()) {
            Ok(bundle) => {
                let t_ptr = to_c_string(bundle.token);
                let h_ptr = to_c_string(bundle.html);
                let c_ptr = to_c_string(bundle.css);

                if t_ptr.is_null() || h_ptr.is_null() || c_ptr.is_null() {
                    unsafe { anCaptcha_free_string(t_ptr) };
                    unsafe { anCaptcha_free_string(h_ptr) };
                    unsafe { anCaptcha_free_string(c_ptr) };
                    return Status::ErrMemory as i32;
                }

                if !token_out.is_null() {
                    unsafe { *token_out = t_ptr };
                }
                if !html_out.is_null() {
                    unsafe { *html_out = h_ptr };
                }
                if !css_out.is_null() {
                    unsafe { *css_out = c_ptr };
                }
                Status::Ok as i32
            }
            Err(e) => {
                error!("Pair generation error: {e:?}");
                set_error("Failed to generate captcha");
                Status::ErrMemory as i32
            }
        }
    })
}
