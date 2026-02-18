//! C-compatible error reporting and memory management.

use libc::{c_char, size_t};
use std::ffi::CString;
use std::panic;
use std::ptr;

use crate::core::utils::{LAST_ERROR, catch_panic};

#[unsafe(no_mangle)]
/// Returns the length of the last error message in bytes.
pub extern "C" fn anCaptcha_last_error_length() -> size_t {
    LAST_ERROR.with(|e: &std::cell::RefCell<String>| e.borrow().len())
}

#[unsafe(no_mangle)]
/// Copies the last error message into the provided C buffer.
///
/// # Safety
///
/// `buffer` must be a valid pointer to a memory area of at least `length` bytes.
pub unsafe extern "C" fn anCaptcha_last_error_message(buffer: *mut c_char, length: size_t) -> i32 {
    catch_panic!({
        if buffer.is_null() || length == 0 {
            return -1;
        }

        LAST_ERROR.with(|e| {
            let err = e.borrow();
            let len = err.len().min(length.saturating_sub(1));
            if len > 0 {
                unsafe {
                    ptr::copy_nonoverlapping(err.as_ptr().cast::<c_char>(), buffer, len);
                }
                let terminator = unsafe { buffer.add(len) };
                unsafe { *terminator = 0 };
            }
            i32::try_from(len).unwrap_or(0)
        })
    })
}

#[unsafe(no_mangle)]
/// Frees a C string allocated by the Rust library.
///
/// # Safety
///
/// `ptr` must be a valid pointer previously returned by a library function.
pub unsafe extern "C" fn anCaptcha_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = panic::catch_unwind(panic::AssertUnwindSafe(|| unsafe {
            let _ = CString::from_raw(ptr);
        }));
    }
}
