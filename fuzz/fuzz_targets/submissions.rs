#![no_main]

use ancaptcha::{Difficulty, PairSubmission, RotateSubmission, SliderSubmission};
use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct FuzzSubmission {
    token: String,
    s0: Option<String>,
    s1: Option<String>,
    s2: Option<String>,
    p_s0: Vec<String>,
    p_s1: Vec<String>,
    p_s2: Vec<String>,
}

fuzz_target!(|input: FuzzSubmission| {
    let secret = [0u8; 32];

    let rotate_sub = RotateSubmission {
        token: input.token.clone(),
        s0: input.s0.clone(),
        s1: input.s1.clone(),
        s2: input.s2.clone(),
    };
    let _ = rotate_sub.verify(&secret, Difficulty::Medium);
    let _ = rotate_sub.verify(&secret, Difficulty::Hard);

    let slider_sub = SliderSubmission {
        token: input.token.clone(),
        s0: input.s0.clone(),
        s1: input.s1.clone(),
        s2: input.s2.clone(),
    };
    let _ = slider_sub.verify(&secret, Difficulty::Medium);
    let _ = slider_sub.verify(&secret, Difficulty::Hard);

    let pair_sub = PairSubmission {
        token: input.token,
        s0: input.p_s0,
        s1: input.p_s1,
        s2: input.p_s2,
    };
    let _ = pair_sub.verify(&secret, Difficulty::Medium);
    let _ = pair_sub.verify(&secret, Difficulty::Hard);
});
