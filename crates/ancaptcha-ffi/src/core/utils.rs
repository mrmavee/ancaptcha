//! Internal utilities and status codes for the FFI bridge.

use libc::c_char;
use std::cell::RefCell;
use std::ffi::CString;

thread_local! {
    pub static LAST_ERROR: RefCell<String> = const { RefCell::new(String::new()) };
}

/// Status codes returned by FFI functions.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    /// Success.
    Ok = 0,
    /// Challenge verification failed.
    VerificationFailed = 1,
    /// Internal panic.
    ErrPanic = -1,
    /// Encryption/decryption failure.
    ErrCrypto = -2,
    /// Memory allocation or buffer error.
    ErrMemory = -3,
    /// Token structural error.
    ErrInvalidToken = -4,
    /// Token expired.
    ErrExpiredToken = -5,
}

/// Sets the thread-local last error message.
pub fn set_error(msg: &str) {
    LAST_ERROR.with(|e| {
        *e.borrow_mut() = msg.to_string();
    });
}

macro_rules! catch_panic {
    ($($body:tt)*) => {
        match ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| { $($body)* })) {
            Ok(res) => res,
            Err(_) => {
                $crate::core::utils::set_error("Panic occurred in Rust FFI boundary");
                $crate::core::utils::Status::ErrPanic as i32
            }
        }
    };
}

pub(crate) use catch_panic;

/// Converts a Rust String into a raw C pointer.
///
/// Returns a null pointer if the conversion fails.
pub fn to_c_string(s: String) -> *mut c_char {
    CString::new(s).map_or(std::ptr::null_mut(), CString::into_raw)
}
