# anCaptcha FFI

C-ABI bridge for the anCaptcha engine. This crate provides the foreign function interface necessary to integrate stateless, No-JS verification into Go, Python, PHP, and other languages.

## Distribution

The C header file and compiled libraries are typically distributed via the main repository's `dist/` directory.

## FFI API Snapshot

```c
// Initialize configuration and global cache
Status anCaptcha_set_config(const char* secret_hex);

// Generate challenges
Status anCaptcha_generate_rotate(const char* error_msg, CaptchaBundle* out);

// Automatic request verification (scans form-data/HashMap equivalents)
Status anCaptcha_verify_auto(const char* secret_hex, char** keys, char*** vals, size_t keys_len, bool* out);
```

## Integration

Official high-level wrappers are maintained in the main repository:
- `examples/go/`
- `examples/python/`
- `examples/php/`

(c) 2026 Maverick. Apache-2.0.
