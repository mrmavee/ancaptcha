//! Seedless RNG helpers and numeric clamping utilities.

pub fn get_random_range<T, R>(range: R) -> T
where
    T: rand::distr::uniform::SampleUniform,
    R: rand::distr::uniform::SampleRange<T>,
{
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random_range(range)
}

#[must_use]
pub fn get_random_index(len: usize) -> usize {
    get_random_range(0..len)
}

#[must_use]
pub fn get_random_bool() -> bool {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

#[must_use]
pub fn get_random_probability() -> f32 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

#[must_use]
pub fn get_random_u64() -> u64 {
    use rand::RngExt;
    let mut rng = rand::rng();
    rng.random()
}

#[must_use]
pub fn clamp_to_u8(val: i16) -> u8 {
    u8::try_from(val.clamp(0, 255)).unwrap_or(0)
}
