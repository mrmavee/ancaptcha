# Security & Fuzzing

## Dependency Advisories
No security advisories are currently ignored in `deny.toml`. All dependencies are audited for known vulnerabilities.

## Memory Safety Audit

The FFI boundary and core library are periodically audited using automated instrumentation and heap profiling.

- **AddressSanitizer (ASan)**: Verified clean during daily CI runs, `ffi_bridge` fuzzing sessions, and full integration test suites. No memory corruption detected.
- **Valgrind Memcheck**: Integration tests verified with **0 bytes definitely lost** across the C-ABI boundary.
- **Valgrind Massif**: Heap profiling confirms persistent memory usage is strictly tied to the static asset cache (~10MB for 297+ optimized variants). Peak heap usage during warm-up is ~25MB.

## Fuzzing Status

Core logic is continuously stressed using `cargo-fuzz` (libFuzzer) to ensure memory safety and zero panics.

- **Corpus**: 1914 seeds (7.6M, minimized)
- **Crashes Found**: 0
- **Last Updated**: 2026-04-27

### Statistics
| Target | Coverage | Features | Corpus |
|--------|----------|----------|--------|
| `token_payload` | 510 | 775 | 33/13154b |
| `verify_request` | 901 | 5746 | 630/277Kb |
| `submissions` | 983 | 1683 | 158/18Kb |
| `ffi_bridge` | 4540 | 15121 | 959/386Kb |

### Target Descriptions
- `token_payload`: Fuzzes `ancaptcha::crypto::cipher::decrypt` (ChaCha20-Poly1305) and `ancaptcha::crypto::token::TokenPayload::from_bytes` (bincode deserialization) to ensure robust handling of malformed or corrupted data.
- `verify_request`: Stress tests the high-level `AnCaptcha::verify_request` entry point with arbitrary form maps, validating identifier unscrambling and overall integration.
- `submissions`: Directly validates the inner `verify` logic for `RotateSubmission`, `SliderSubmission`, and `PairSubmission` across multiple difficulty levels with extreme input values.
- `ffi_bridge`: Fuzzes the C-compatible `ancaptcha-ffi` APIs like `anCaptcha_verify_auto` and `anCaptcha_set_theme` ensuring raw null-terminated pointer parsing and url-encoded decoding remain memory safe.

## Resolved Vulnerabilities

- **`hex_to_rgba` UTF-8 Slice Panic** (2026-04-17)
  - **Vector**: Core library target (triggered via `ffi_bridge` fuzzer).
  - **Issue**: Six-byte non-ASCII strings bypassed `.len() == 6` check, causing thread panics on `&raw[0..2]` byte slicing.
  - **Fix**: Added `.is_ascii()` guard. ([18abee1](#))
