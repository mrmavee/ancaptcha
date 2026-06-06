#![no_main]

use ancaptcha_ffi::{
    anCaptcha_free_string, anCaptcha_generate_pair, anCaptcha_generate_rotate,
    anCaptcha_generate_slider, anCaptcha_last_error_length, anCaptcha_last_error_message,
    anCaptcha_set_config, anCaptcha_set_layout, anCaptcha_set_size, anCaptcha_set_theme,
    anCaptcha_verify_auto, anCaptcha_verify_pair, anCaptcha_verify_rotate, anCaptcha_verify_slider,
    anCaptcha_warm_up,
};
use libfuzzer_sys::fuzz_target;
use std::ptr;
use std::sync::OnceLock;

static INIT: OnceLock<()> = OnceLock::new();

fuzz_target!(|data: &[u8]| {
    INIT.get_or_init(|| {
        let secret = [0u8; 32];
        unsafe {
            let _ = anCaptcha_set_config(secret.as_ptr(), 1);
        }
    });

    let mut raw_buffer = data.to_vec();
    raw_buffer.push(0);

    let c_ptr = raw_buffer.as_ptr().cast::<i8>();

    unsafe {
        let _ = anCaptcha_verify_auto(c_ptr);

        let c_ptr_arr = [c_ptr];
        let _ = anCaptcha_verify_rotate(c_ptr, c_ptr_arr.as_ptr(), 1);
        let _ = anCaptcha_verify_slider(c_ptr, c_ptr_arr.as_ptr(), 1);

        let c_ptr_arr_arr = [c_ptr_arr.as_ptr()];
        let stages = [1usize];
        let _ = anCaptcha_verify_pair(c_ptr, c_ptr_arr_arr.as_ptr(), stages.as_ptr(), 1);

        let _ = anCaptcha_set_theme(c_ptr, ptr::null(), c_ptr, ptr::null(), c_ptr, ptr::null());
        let _ = anCaptcha_set_layout(ptr::null(), c_ptr, ptr::null(), c_ptr, ptr::null(), c_ptr);
        let _ = anCaptcha_set_size(c_ptr);

        let mut token_ptr: *mut i8 = ptr::null_mut();
        let mut html_ptr: *mut i8 = ptr::null_mut();
        let mut css_ptr: *mut i8 = ptr::null_mut();

        if anCaptcha_generate_rotate(
            1,
            c_ptr,
            &raw mut token_ptr,
            &raw mut html_ptr,
            &raw mut css_ptr,
        ) == 0
        {
            anCaptcha_free_string(token_ptr);
            anCaptcha_free_string(html_ptr);
            anCaptcha_free_string(css_ptr);
        }
        if anCaptcha_generate_slider(
            1,
            c_ptr,
            &raw mut token_ptr,
            &raw mut html_ptr,
            &raw mut css_ptr,
        ) == 0
        {
            anCaptcha_free_string(token_ptr);
            anCaptcha_free_string(html_ptr);
            anCaptcha_free_string(css_ptr);
        }
        if anCaptcha_generate_pair(
            1,
            c_ptr,
            &raw mut token_ptr,
            &raw mut html_ptr,
            &raw mut css_ptr,
        ) == 0
        {
            anCaptcha_free_string(token_ptr);
            anCaptcha_free_string(html_ptr);
            anCaptcha_free_string(css_ptr);
        }

        let mut err_buf = [0i8; 256];
        let _ = anCaptcha_last_error_message(err_buf.as_mut_ptr(), err_buf.len());
        let _ = anCaptcha_last_error_length();
        let _ = anCaptcha_warm_up(i32::from(raw_buffer[0]));
    }
});
