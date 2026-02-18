//! C-compatible configuration and theme management.

use std::ffi::CStr;
use std::ptr;
use std::sync::{LazyLock, OnceLock, RwLock};

use crate::core::utils::{Status, catch_panic, set_error};
use libc::c_char;

static SECRET: OnceLock<[u8; 32]> = OnceLock::new();
static THEME: LazyLock<RwLock<ancaptcha::Theme>> =
    LazyLock::new(|| RwLock::new(ancaptcha::Theme::default()));
static LAYOUT: LazyLock<RwLock<ancaptcha::Layout>> =
    LazyLock::new(|| RwLock::new(ancaptcha::Layout::default()));

/// Returns the globally configured secret key.
pub fn get_secret() -> Option<&'static [u8; 32]> {
    SECRET.get()
}

/// Returns the current visual theme settings.
pub fn get_theme() -> ancaptcha::Theme {
    THEME
        .read()
        .map_or_else(|_| ancaptcha::Theme::default(), |l| l.clone())
}

/// Returns the current structural layout settings.
pub fn get_layout() -> ancaptcha::Layout {
    LAYOUT
        .read()
        .map_or_else(|_| ancaptcha::Layout::default(), |l| l.clone())
}

#[unsafe(no_mangle)]
/// Initializes the library with a secret key and noise intensity.
///
/// # Safety
///
/// `secret` must be a valid pointer to a 32-byte array.
pub unsafe extern "C" fn anCaptcha_set_config(secret: *const u8, noise_intensity: i32) -> i32 {
    if secret.is_null() {
        set_error("Secret key is null");
        return Status::ErrCrypto as i32;
    }

    catch_panic!({
        let mut key = [0u8; 32];
        unsafe {
            ptr::copy_nonoverlapping(secret, key.as_mut_ptr(), 32);
        }

        let n_intensity = match noise_intensity {
            0 => ancaptcha::NoiseIntensity::Low,
            1 => ancaptcha::NoiseIntensity::Medium,
            _ => ancaptcha::NoiseIntensity::High,
        };

        ancaptcha::init_with_intensity(n_intensity);

        if let Some(existing) = SECRET.get() {
            if existing == &key {
                return Status::Ok as i32;
            }
            set_error("AnCaptcha already initialized with a different secret");
            return Status::ErrCrypto as i32;
        }

        let _ = SECRET.set(key);
        Status::Ok as i32
    })
}

#[unsafe(no_mangle)]
/// Configures the visual theme for the captcha interface.
///
/// # Safety
///
/// All pointers must be valid null-terminated C strings or null to keep current values.
pub unsafe extern "C" fn anCaptcha_set_theme(
    background_color: *const c_char,
    border_color: *const c_char,
    text_color: *const c_char,
    accent_color: *const c_char,
    error_color: *const c_char,
    font_family: *const c_char,
) -> i32 {
    catch_panic!({
        let mut theme = ancaptcha::Theme::default();

        macro_rules! set_str {
            ($field:ident, $ptr:ident) => {
                if !$ptr.is_null() {
                    let s = unsafe { CStr::from_ptr($ptr) }.to_string_lossy();

                    if !s.is_empty() {
                        theme.$field = s.into_owned();
                    }
                }
            };
        }

        set_str!(background_color, background_color);
        set_str!(border_color, border_color);
        set_str!(text_color, text_color);
        set_str!(accent_color, accent_color);
        set_str!(error_color, error_color);
        set_str!(font_family, font_family);

        THEME.write().map_or_else(
            |_| {
                set_error("Theme lock poisoned");
                Status::ErrCrypto as i32
            },
            |mut lock| {
                *lock = theme;
                Status::Ok as i32
            },
        )
    })
}

#[unsafe(no_mangle)]
/// Configures the structural layout dimensions for the captcha.
///
/// # Safety
///
/// All pointers must be valid null-terminated C strings or null to keep current values.
pub unsafe extern "C" fn anCaptcha_set_layout(
    width: *const c_char,
    max_width: *const c_char,
    margin: *const c_char,
    min_height: *const c_char,
    padding: *const c_char,
    checkbox_size: *const c_char,
) -> i32 {
    catch_panic!({
        let mut layout = ancaptcha::Layout::default();

        macro_rules! set_str {
            ($field:ident, $ptr:ident) => {
                if !$ptr.is_null() {
                    let s = unsafe { CStr::from_ptr($ptr) }.to_string_lossy();
                    if !s.is_empty() {
                        layout.$field = s.into_owned();
                    }
                }
            };
        }

        set_str!(width, width);
        set_str!(max_width, max_width);
        set_str!(margin, margin);
        set_str!(min_height, min_height);
        set_str!(padding, padding);
        set_str!(checkbox_size, checkbox_size);

        LAYOUT.write().map_or_else(
            |_| {
                set_error("Layout lock poisoned");
                Status::ErrCrypto as i32
            },
            |mut lock| {
                *lock = layout;
                Status::Ok as i32
            },
        )
    })
}

#[unsafe(no_mangle)]
/// Pre-computes asset variations for the specified captcha style.
///
/// # Safety
///
/// The function assumes the internal cache is already initialized via `set_config`.
pub unsafe extern "C" fn anCaptcha_warm_up(style: i32) -> i32 {
    catch_panic!({
        let c_style = match style {
            0 => ancaptcha::CaptchaStyle::Rotate,
            1 => ancaptcha::CaptchaStyle::Slider,
            2 => ancaptcha::CaptchaStyle::Pair,
            _ => {
                set_error("Invalid captcha style index");
                return Status::ErrCrypto as i32;
            }
        };

        ancaptcha::storage::get_cache().map_or_else(
            || {
                set_error("Cache not initialized. Call set_config first.");
                Status::ErrCrypto as i32
            },
            |cache| {
                cache.warm_up(c_style);
                Status::Ok as i32
            },
        )
    })
}
