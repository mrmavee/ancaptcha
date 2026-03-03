# Security & Fuzzing

## Dependency Advisories
No security advisories are currently ignored in `deny.toml`. All dependencies are audited for known vulnerabilities.

## Fuzzing Status
Core logic is continuously stressed using `cargo-fuzz` (libFuzzer) to ensure memory safety and zero panics.

- **Corpus**: 930 seeds (3.7M, minimized)
- **Crashes Found**: 0
- **Last Updated**: 2026-03-03

### Statistics
| Target | Coverage | Features | Corpus |
|--------|----------|----------|--------|
| `token_payload` | 490 | 735 | 33/13154b |
| `verify_request` | 867 | 5646 | 694/326Kb |
| `submissions` | 958 | 1874 | 183/34Kb |

### Target Descriptions
- `token_payload`: Fuzzes `ancaptcha::crypto::cipher::decrypt` (ChaCha20-Poly1305) and `ancaptcha::crypto::token::TokenPayload::from_bytes` (bincode deserialization) to ensure robust handling of malformed or corrupted data.
- `verify_request`: Stress tests the high-level `AnCaptcha::verify_request` entry point with arbitrary form maps, validating identifier unscrambling and overall integration.
- `submissions`: Directly validates the inner `verify` logic for `RotateSubmission`, `SliderSubmission`, and `PairSubmission` across multiple difficulty levels with extreme input values.
