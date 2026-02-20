# anCaptcha FFI

C-ABI bridge for the anCaptcha engine. This crate provides the foreign function interface necessary to integrate stateless, No-JS verification into Go, Python, PHP, and other languages.

## Distribution

The C header file and compiled libraries are distributed via the repository's `dist/` directory.

## FFI API Snapshot

### Configuration
```c
// Initialize configuration with 32-byte secret and global asset warming
Status anCaptcha_set_config(const uint8_t* secret_bytes, bool warm_up);
```

### Generation
```c
// Generate challenges (difficulty: 0=Easy, 1=Medium, 2=Hard)
// Memory for strings must be freed using anCaptcha_free_string
Status anCaptcha_generate_rotate(int32_t difficulty, const char* error_msg, char** token_out, char** html_out, char** css_out);
Status anCaptcha_generate_slider(int32_t difficulty, const char* error_msg, char** token_out, char** html_out, char** css_out);
Status anCaptcha_generate_pair(int32_t difficulty, const char* error_msg, char** token_out, char** html_out, char** css_out);
```

### Verification
```c
// Automatic verification from URL-encoded form data
Status anCaptcha_verify_auto(const char* form_data_urlencoded);

// Manual verification
Status anCaptcha_verify_rotate(const char* token, const char** values, size_t values_len);
```

## Integration

Official high-level wrappers are maintained in the main repository:
- `examples/go/`
- `examples/python/`
- `examples/php/`

(c) 2026 Maverick. Apache-2.0.
