# anCaptcha

A No-JS, stateless captcha engine implemented in Rust. Originally designed for the darknet and Tor hidden services, to provide human verification without requiring JavaScript.

## Core Engine

This crate provides the core logic for:
- **Stateless Tokens**: Authenticated encryption (ChaCha20-Poly1305) stores challenge state without server sessions.
- **Resource Processing**: JIT asset warming and noise injection to thwart automated solvers.
- **Multi-Modal Challenges**: Supports Rotate, Slider, and Find-the-Pair verification styles.

## Usage

```rust
use ancaptcha::{AnCaptcha, Config, init_cache, CaptchaStyle};

// 1. Initialize global asset cache (recommended on startup)
let cache = init_cache();
cache.warm_up(CaptchaStyle::Rotate);

// 2. Setup engine with 32-byte secret
let secret = [0u8; 32]; // Use a secure key from env
let ac = AnCaptcha::new(Config::new(secret));

// 3. Generate a challenge
let bundle = ac.generate_rotate(None).expect("Failed to generate");

// bundle.html -> The CSS-only captcha form
// bundle.token -> Encrypted state to be echoed in the form submission
```

## Features

- `std`: Enabled by default.
- `rayon`: Parallel asset pre-computation.

(c) 2026 Maverick. Apache-2.0.
