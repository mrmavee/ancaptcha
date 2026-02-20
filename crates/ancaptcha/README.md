# anCaptcha

A No-JS, stateless captcha engine implemented in Rust. Originally designed for the darknet and Tor hidden services, to provide human verification without requiring JavaScript.

## Core Engine

This crate provides the core logic for:
- **Stateless Tokens**: Authenticated encryption (ChaCha20-Poly1305) stores challenge state without server sessions.
- **Resource Processing**: JIT asset warming and noise injection to thwart automated solvers.
- **Multi-Modal Challenges**: Supports Rotate, Slider, and Find-the-Pair verification styles.

## Usage

### 1. Initialize Asset Cache
It is recommended to initialize and warm up the cache during startup. Since warming up involves heavy image processing, use a blocking task if in an async environment. You can warm up only the styles you intend to use.

```rust
use ancaptcha::{init_cache, CaptchaStyle};

let cache = init_cache();
// Recommended: Warm up only the styles you need in a background thread
std::thread::spawn(move || {
    cache.warm_up(CaptchaStyle::Rotate);
    // cache.warm_up(CaptchaStyle::Slider);
    // cache.warm_up(CaptchaStyle::Pair);
});
```

### 2. Generate a Challenge
Initialize the engine with a 32-byte secret key (e.g., from an environment variable).

```rust
use ancaptcha::{AnCaptcha, Config, Difficulty};

let secret = [0u8; 32]; // Replace with real secret
let ac = AnCaptcha::new(Config::new(secret).with_difficulty(Difficulty::Medium));

// Choose your style: generate_rotate, generate_slider, or generate_pair
let bundle = ac.generate_slider(None).expect("Failed to generate");

// bundle.html contains the combined HTML and CSS <style> block
// bundle.token must be included in your form submission
```

### 3. Verify Submission
You can verify the raw form-data directly (automatic) or verify individual values.

```rust
// Automatic verification from a HashMap<String, Vec<String>>
let is_valid = ac.verify_request(&form_map).unwrap_or(false);
```

## Features

- `std`: Enabled by default.
- `rayon`: Parallel asset pre-computation.

(c) 2026 Maverick. Apache-2.0.
