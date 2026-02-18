//! C-compatible submission verification interfaces.

use crate::core::config::get_secret;
use crate::core::utils::{Status, catch_panic, set_error};
use libc::{c_char, size_t};
use std::collections::HashMap;
use std::ffi::CStr;
use tracing::error;

#[unsafe(no_mangle)]
/// Verifies a Rotate captcha submission.
///
/// # Safety
///
/// `token` and `values` must be valid null-terminated C strings.
pub unsafe extern "C" fn anCaptcha_verify_rotate(
    token: *const c_char,
    values: *const *const c_char,
    values_len: size_t,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        if token.is_null() || (values_len > 0 && values.is_null()) {
            set_error("Invalid arguments");
            return Status::ErrInvalidToken as i32;
        }

        let token_str = unsafe { CStr::from_ptr(token) }.to_string_lossy();
        let mut submitted = Vec::with_capacity(values_len);
        for i in 0..values_len {
            let val_ptr = unsafe { values.add(i) };
            let val_ptr = unsafe { *val_ptr };
            if val_ptr.is_null() {
                continue;
            }
            submitted.push(unsafe { CStr::from_ptr(val_ptr) }.to_string_lossy());
        }

        let refs: Vec<&str> = submitted.iter().map(AsRef::as_ref).collect();

        match ancaptcha::verify_rotate(secret, &token_str, &refs) {
            Ok(true) => Status::Ok as i32,
            Ok(false) => {
                set_error("Incorrect answer");
                Status::VerificationFailed as i32
            }
            Err(e) => {
                error!("Rotate verification error: {e:?}");
                set_error("Invalid or expired token");
                Status::ErrInvalidToken as i32
            }
        }
    })
}

#[unsafe(no_mangle)]
/// Verifies a Slider captcha submission.
///
/// # Safety
///
/// `token` and `values` must be valid null-terminated C strings.
pub unsafe extern "C" fn anCaptcha_verify_slider(
    token: *const c_char,
    values: *const *const c_char,
    values_len: size_t,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        if token.is_null() || (values_len > 0 && values.is_null()) {
            set_error("Invalid arguments");
            return Status::ErrInvalidToken as i32;
        }

        let token_str = unsafe { CStr::from_ptr(token) }.to_string_lossy();
        let mut submitted = Vec::with_capacity(values_len);
        for i in 0..values_len {
            let val_ptr = unsafe { values.add(i) };
            let val_ptr = unsafe { *val_ptr };
            if val_ptr.is_null() {
                continue;
            }
            submitted.push(unsafe { CStr::from_ptr(val_ptr) }.to_string_lossy());
        }

        let refs: Vec<&str> = submitted.iter().map(AsRef::as_ref).collect();

        match ancaptcha::verify_slider(secret, &token_str, &refs) {
            Ok(true) => Status::Ok as i32,
            Ok(false) => {
                set_error("Incorrect answer");
                Status::VerificationFailed as i32
            }
            Err(e) => {
                error!("Slider verification error: {e:?}");
                set_error("Invalid or expired token");
                Status::ErrInvalidToken as i32
            }
        }
    })
}

#[unsafe(no_mangle)]
/// Verifies a Find the Pair captcha submission.
///
/// # Safety
///
/// `token` and `values` must be valid null-terminated C strings.
pub unsafe extern "C" fn anCaptcha_verify_pair(
    token: *const c_char,
    values: *const *const *const c_char,
    stage_lengths: *const size_t,
    stages_count: size_t,
) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        if token.is_null() || (stages_count > 0 && (values.is_null() || stage_lengths.is_null())) {
            set_error("Invalid arguments");
            return Status::ErrInvalidToken as i32;
        }

        let token_str = unsafe { CStr::from_ptr(token) }.to_string_lossy();
        let mut submitted_stages = Vec::with_capacity(stages_count);
        for i in 0..stages_count {
            let stage_ptr = unsafe { values.add(i) };
            let stage_ptr = unsafe { *stage_ptr };
            let stage_len = unsafe { stage_lengths.add(i) };
            let stage_len = unsafe { *stage_len };
            if stage_ptr.is_null() {
                continue;
            }

            let mut stage_values = Vec::with_capacity(stage_len);
            for j in 0..stage_len {
                let val_ptr = unsafe { stage_ptr.add(j) };
                let val_ptr = unsafe { *val_ptr };
                if val_ptr.is_null() {
                    continue;
                }
                stage_values.push(
                    unsafe { CStr::from_ptr(val_ptr) }
                        .to_string_lossy()
                        .into_owned(),
                );
            }
            submitted_stages.push(stage_values);
        }

        let submitted_refs: Vec<Vec<&str>> = submitted_stages
            .iter()
            .map(|stage| stage.iter().map(String::as_str).collect())
            .collect();

        match ancaptcha::verify_pair(secret, &token_str, &submitted_refs) {
            Ok(true) => Status::Ok as i32,
            Ok(false) => {
                set_error("Incorrect answer");
                Status::VerificationFailed as i32
            }
            Err(e) => {
                error!("Pair verification error: {e:?}");
                set_error("Invalid or expired token");
                Status::ErrInvalidToken as i32
            }
        }
    })
}

fn parse_form(data: &str) -> HashMap<String, Vec<String>> {
    let mut form: HashMap<String, Vec<String>> = HashMap::new();
    for pair in data.split('&') {
        if let Some((k, v)) = pair.split_once('=') {
            let k_raw = k.replace('+', " ");
            let v_raw = v.replace('+', " ");
            let k_dec = percent_encoding::percent_decode_str(&k_raw)
                .decode_utf8_lossy()
                .into_owned();
            let v_dec = percent_encoding::percent_decode_str(&v_raw)
                .decode_utf8_lossy()
                .into_owned();
            form.entry(k_dec).or_default().push(v_dec);
        }
    }
    form
}

#[unsafe(no_mangle)]
/// Automatically detects and verifies a captcha submission from URL-encoded form data.
///
/// # Safety
///
/// `form_data_urlencoded` must be a valid null-terminated C string.
pub unsafe extern "C" fn anCaptcha_verify_auto(form_data_urlencoded: *const c_char) -> i32 {
    catch_panic!({
        let Some(secret) = get_secret() else {
            set_error("Configuration missing");
            return Status::ErrCrypto as i32;
        };

        if form_data_urlencoded.is_null() {
            set_error("Invalid arguments");
            return Status::ErrInvalidToken as i32;
        }

        let form_str = unsafe { CStr::from_ptr(form_data_urlencoded) }.to_string_lossy();
        let form = parse_form(&form_str);

        let ancaptcha = ancaptcha::AnCaptcha::new(ancaptcha::Config::new(*secret));
        match ancaptcha.verify_request(&form) {
            Ok(true) => Status::Ok as i32,
            Ok(false) => {
                set_error("Incorrect answer");
                Status::VerificationFailed as i32
            }
            Err(e) => {
                error!("Auto verification error: {e:?}");
                set_error("Invalid or expired token");
                Status::ErrInvalidToken as i32
            }
        }
    })
}
