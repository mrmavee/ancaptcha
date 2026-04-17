//! Seedless RNG helpers and numeric clamping utilities.

/// Samples uniformly from `range`.
pub fn get_random_range<T, R>(range: R) -> T
where
    T: rand::distr::uniform::SampleUniform,
    R: rand::distr::uniform::SampleRange<T>,
{
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random_range(range)
}

/// Random index in `0..len`.
#[must_use]
pub fn get_random_index(len: usize) -> usize {
    get_random_range(0..len)
}

/// Random bool.
#[must_use]
pub fn get_random_bool() -> bool {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Random `f32` in `[0.0, 1.0)`.
#[must_use]
pub fn get_random_probability() -> f32 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Random `u64`.
#[must_use]
pub fn get_random_u64() -> u64 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

/// Clamps `val` to `[0, 255]` and converts to `u8`.
#[must_use]
pub fn clamp_to_u8(val: i16) -> u8 {
    u8::try_from(val.clamp(0, 255)).unwrap_or(0)
}
