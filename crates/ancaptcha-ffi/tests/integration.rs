use ancaptcha_ffi::{
    Status, anCaptcha_free_string, anCaptcha_generate_pair, anCaptcha_generate_rotate,
    anCaptcha_generate_slider, anCaptcha_last_error_length, anCaptcha_last_error_message,
    anCaptcha_set_config, anCaptcha_set_layout, anCaptcha_set_theme, anCaptcha_verify_auto,
    anCaptcha_verify_pair, anCaptcha_verify_rotate, anCaptcha_verify_slider, anCaptcha_warm_up,
};
use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::ptr;

const TEST_SECRET: [u8; 32] = [13u8; 32];

fn setup_ffi() {
    unsafe {
        let _ = anCaptcha_set_config(TEST_SECRET.as_ptr(), 1);
        let _ = anCaptcha_warm_up(0);
        let _ = anCaptcha_warm_up(1);
        let _ = anCaptcha_warm_up(2);
    }
}

#[test]
fn ffi_config_flow() {
    unsafe {
        assert_eq!(
            anCaptcha_set_config(TEST_SECRET.as_ptr(), 1),
            Status::Ok as i32
        );

        let bg = CString::new("#123456").unwrap();
        assert_eq!(
            anCaptcha_set_theme(
                bg.as_ptr(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null()
            ),
            Status::Ok as i32
        );

        let w = CString::new("300px").unwrap();
        assert_eq!(
            anCaptcha_set_layout(
                w.as_ptr(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null(),
                ptr::null()
            ),
            Status::Ok as i32
        );
    }
}

#[test]
fn ffi_e2e_rotate() {
    setup_ffi();
    unsafe {
        let mut ptr_t: *mut c_char = ptr::null_mut();
        let mut ptr_h: *mut c_char = ptr::null_mut();
        let mut ptr_c: *mut c_char = ptr::null_mut();

        assert_eq!(
            anCaptcha_generate_rotate(
                0,
                ptr::null(),
                &raw mut ptr_t,
                &raw mut ptr_h,
                &raw mut ptr_c
            ),
            Status::Ok as i32
        );

        let cow_h = CStr::from_ptr(ptr_h).to_string_lossy();
        let cow_c = CStr::from_ptr(ptr_c).to_string_lossy();
        assert!(cow_h.contains("<style>"));
        assert_eq!(cow_c, "");

        let cow_t = CStr::from_ptr(ptr_t).to_string_lossy();
        let token_bytes = ancaptcha::common::b64_decode_url_safe(cow_t.as_ref()).unwrap();
        let plaintext = ancaptcha::crypto::cipher::decrypt(&TEST_SECRET, &token_bytes).unwrap();
        let payload = ancaptcha::crypto::token::TokenPayload::from_bytes(&plaintext).unwrap();

        let angle_bytes: [u8; 2] = payload.solution[0..2].try_into().unwrap();
        let val_idx = u16::from_le_bytes(angle_bytes) / 45;
        let semantic = format!("v{val_idx}");
        let c_val = CString::new(semantic).unwrap();
        let vals = [c_val.as_ptr()];

        assert_eq!(
            anCaptcha_verify_rotate(ptr_t, vals.as_ptr(), 1),
            Status::Ok as i32
        );

        anCaptcha_free_string(ptr_t);
        anCaptcha_free_string(ptr_h);
        anCaptcha_free_string(ptr_c);
    }
}

#[test]
fn ffi_e2e_slider() {
    setup_ffi();
    unsafe {
        let mut ptr_t: *mut c_char = ptr::null_mut();
        let mut ptr_h: *mut c_char = ptr::null_mut();
        let mut ptr_c: *mut c_char = ptr::null_mut();

        assert_eq!(
            anCaptcha_generate_slider(
                0,
                ptr::null(),
                &raw mut ptr_t,
                &raw mut ptr_h,
                &raw mut ptr_c
            ),
            Status::Ok as i32
        );

        let cow_h = CStr::from_ptr(ptr_h).to_string_lossy();
        let cow_c = CStr::from_ptr(ptr_c).to_string_lossy();
        assert!(cow_h.contains("<style>"));
        assert_eq!(cow_c, "");

        let cow_t = CStr::from_ptr(ptr_t).to_string_lossy();
        let token_bytes = ancaptcha::common::b64_decode_url_safe(cow_t.as_ref()).unwrap();
        let plaintext = ancaptcha::crypto::cipher::decrypt(&TEST_SECRET, &token_bytes).unwrap();
        let payload = ancaptcha::crypto::token::TokenPayload::from_bytes(&plaintext).unwrap();

        let pos = payload.solution[0];
        let semantic = format!("p{pos}");
        let c_val = CString::new(semantic).unwrap();
        let vals = [c_val.as_ptr()];

        assert_eq!(
            anCaptcha_verify_slider(ptr_t, vals.as_ptr(), 1),
            Status::Ok as i32
        );

        anCaptcha_free_string(ptr_t);
        anCaptcha_free_string(ptr_h);
        anCaptcha_free_string(ptr_c);
    }
}

#[test]
fn ffi_e2e_pair() {
    setup_ffi();
    unsafe {
        let mut ptr_t: *mut c_char = ptr::null_mut();
        let mut ptr_h: *mut c_char = ptr::null_mut();
        let mut ptr_c: *mut c_char = ptr::null_mut();

        assert_eq!(
            anCaptcha_generate_pair(
                0,
                ptr::null(),
                &raw mut ptr_t,
                &raw mut ptr_h,
                &raw mut ptr_c
            ),
            Status::Ok as i32
        );

        let cow_h = CStr::from_ptr(ptr_h).to_string_lossy();
        let cow_c = CStr::from_ptr(ptr_c).to_string_lossy();
        assert!(cow_h.contains("<style>"));
        assert_eq!(cow_c, "");

        let cow_t = CStr::from_ptr(ptr_t).to_string_lossy();
        let token_bytes = ancaptcha::common::b64_decode_url_safe(cow_t.as_ref()).unwrap();
        let plaintext = ancaptcha::crypto::cipher::decrypt(&TEST_SECRET, &token_bytes).unwrap();
        let payload = ancaptcha::crypto::token::TokenPayload::from_bytes(&plaintext).unwrap();

        let a = payload.solution[0];
        let b = payload.solution[1];
        let sem_a = CString::new(format!("c{a}")).unwrap();
        let sem_b = CString::new(format!("c{b}")).unwrap();
        let stage_vals = [sem_a.as_ptr(), sem_b.as_ptr()];
        let stages = [stage_vals.as_ptr()];
        let lens = [2 as size_t];

        assert_eq!(
            anCaptcha_verify_pair(ptr_t, stages.as_ptr(), lens.as_ptr(), 1),
            Status::Ok as i32
        );

        anCaptcha_free_string(ptr_t);
        anCaptcha_free_string(ptr_h);
        anCaptcha_free_string(ptr_c);
    }
}

#[test]
fn ffi_auto_verification() {
    setup_ffi();
    unsafe {
        let mut ptr_t: *mut c_char = ptr::null_mut();
        let mut ptr_h: *mut c_char = ptr::null_mut();
        let mut ptr_c: *mut c_char = ptr::null_mut();

        let _ = anCaptcha_generate_rotate(
            0,
            ptr::null(),
            &raw mut ptr_t,
            &raw mut ptr_h,
            &raw mut ptr_c,
        );

        let cow_t = CStr::from_ptr(ptr_t).to_string_lossy();
        let str_t = cow_t.as_ref();
        let token_bytes = ancaptcha::common::b64_decode_url_safe(str_t).unwrap();
        let plaintext = ancaptcha::crypto::cipher::decrypt(&TEST_SECRET, &token_bytes).unwrap();
        let payload = ancaptcha::crypto::token::TokenPayload::from_bytes(&plaintext).unwrap();

        let mut mapper = ancaptcha::engine::NameMapper::new(payload.seed);
        mapper.warm_up(payload.difficulty, "rotate");

        let angle_bytes: [u8; 2] = payload.solution[0..2].try_into().unwrap();
        let val_sem = format!("v{}", u16::from_le_bytes(angle_bytes) / 45);

        let form = format!(
            "{}={}&{}={}&{}={}",
            mapper.get_or_create("token"),
            str_t,
            mapper.get_or_create("st"),
            mapper.get_or_create("st0"),
            mapper.get_or_create("s0"),
            mapper.get_or_create(&val_sem)
        );
        let c_form = CString::new(form).unwrap();

        assert_eq!(anCaptcha_verify_auto(c_form.as_ptr()), Status::Ok as i32);

        anCaptcha_free_string(ptr_t);
        anCaptcha_free_string(ptr_h);
        anCaptcha_free_string(ptr_c);
    }
}

#[test]
fn ffi_error_reporting() {
    unsafe {
        let _ = anCaptcha_set_config(ptr::null(), 1);
        let len = anCaptcha_last_error_length();
        assert!(len > 0);

        let mut buf = vec![0; len + 1];
        let _ = anCaptcha_last_error_message(buf.as_mut_ptr().cast(), len + 1);
        let msg = CStr::from_ptr(buf.as_ptr().cast()).to_string_lossy();
        assert_eq!(msg, "Secret key is null");
    }
}
