#![no_main]

use ancaptcha::crypto::cipher::decrypt;
use ancaptcha::crypto::token::TokenPayload;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let secret = [0u8; 32];

    if let Ok(decrypted) = decrypt(&secret, data) {
        let _ = TokenPayload::from_bytes(&decrypted);
    }
});
