//! Asset cache management with selective JIT warm-up.

use crate::config::NoiseIntensity;
use crate::config::settings::CaptchaStyle;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

static ASSET_CACHE: OnceLock<AssetCache> = OnceLock::new();

/// Pre-computed slider variant: `(x_slot_idx, y_offset, main_image_bytes, piece_bytes)`.
pub type SliderVariation = (u8, u8, Vec<u8>, Vec<u8>);

/// Thread-safe cache for pre-computed captcha assets.
///
/// Asset pre-computation (warming) is CPU-intensive and may take several seconds on
/// the first run. It is recommended to trigger warming during application startup
/// within a background thread.
pub struct AssetCache {
    intensity: NoiseIntensity,
    sources: HashMap<String, &'static [u8]>,
    names: Vec<String>,
    rotate_cache: HashMap<String, RwLock<Vec<Vec<u8>>>>,
    pair_cache: HashMap<String, RwLock<Vec<Vec<u8>>>>,
    slider_cache: HashMap<String, RwLock<Vec<SliderVariation>>>,
}

impl AssetCache {
    pub(crate) fn new(intensity: NoiseIntensity) -> Self {
        let assets = crate::assets::get_embedded_assets();
        let mut sources = HashMap::new();
        let mut names = Vec::new();
        let mut rotate_cache = HashMap::new();
        let mut pair_cache = HashMap::new();
        let mut slider_cache = HashMap::new();

        for (name, bytes) in assets {
            let name_str = (*name).to_string();
            names.push(name_str.clone());
            sources.insert(name_str.clone(), *bytes);
            rotate_cache.insert(name_str.clone(), RwLock::new(Vec::new()));
            pair_cache.insert(name_str.clone(), RwLock::new(Vec::new()));
            slider_cache.insert(name_str, RwLock::new(Vec::new()));
        }

        Self {
            intensity,
            sources,
            names,
            rotate_cache,
            pair_cache,
            slider_cache,
        }
    }

    fn encode_to_binary(img: &image::DynamicImage, quality: u8) -> Option<Vec<u8>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        let enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
        img.write_with_encoder(enc).ok()?;
        Some(buf.into_inner())
    }

    fn generate_rotate_variant(&self, name: &str) -> Option<Vec<u8>> {
        let bytes = self.sources.get(name)?;
        let orig = image::load_from_memory(bytes).ok()?;
        let img = orig.resize_exact(200, 200, image::imageops::FilterType::Triangle);
        let noisy = crate::engine::apply_full_noise(&img, &self.intensity);
        Self::encode_to_binary(&noisy, 13)
    }

    fn generate_pair_variant(&self, name: &str) -> Option<Vec<u8>> {
        let bytes = self.sources.get(name)?;
        let orig = image::load_from_memory(bytes).ok()?;
        let img = orig.resize_exact(100, 100, image::imageops::FilterType::Triangle);
        let noisy = crate::engine::apply_full_noise(&img, &self.intensity);
        Self::encode_to_binary(&noisy, 10)
    }

    fn generate_slider_variant(&self, name: &str) -> Option<SliderVariation> {
        let bytes = self.sources.get(name)?;
        let orig = image::load_from_memory(bytes).ok()?;
        let img = orig.resize_exact(200, 200, image::imageops::FilterType::Triangle);

        let y_options = [10, 40, 75, 110, 140];
        let piece_y = *y_options
            .get(crate::common::get_random_index(y_options.len()))
            .unwrap_or(&140);

        let x_idx = u8::try_from(crate::common::get_random_index(15)).unwrap_or(0) + 4;
        let piece_x = u32::from(x_idx) * 8;

        let noisy = crate::engine::apply_full_noise(&img, &self.intensity);
        let (main_with_gap, piece) =
            crate::engine::create_slider_cutout(&noisy, piece_x, u32::from(piece_y), 50);

        let m_bin = Self::encode_to_binary(&main_with_gap, 13)?;
        let p_bin = Self::encode_to_binary(&piece, 13)?;

        Some((x_idx, piece_y, m_bin, p_bin))
    }

    fn get_cache_path(style: CaptchaStyle) -> Option<std::path::PathBuf> {
        let base_dir = if let Ok(xdg_cache) = std::env::var("XDG_CACHE_HOME") {
            std::path::PathBuf::from(xdg_cache)
        } else if let Ok(home) = std::env::var("HOME") {
            std::path::Path::new(&home).join(".cache")
        } else if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            std::path::PathBuf::from(local_app_data)
        } else {
            return None;
        };

        let cache_dir = base_dir.join("ancaptcha");

        let mut builder = std::fs::DirBuilder::new();
        builder.recursive(true);

        #[cfg(unix)]
        {
            use std::os::unix::fs::DirBuilderExt;
            builder.mode(0o700);
        }

        if builder.create(&cache_dir).is_err() {
            return None;
        }

        let mut path = cache_dir;
        match style {
            CaptchaStyle::Rotate => path.push("ancaptcha_rotate_v1.bin"),
            CaptchaStyle::Pair => path.push("ancaptcha_pair_v1.bin"),
            CaptchaStyle::Slider => path.push("ancaptcha_slider_v1.bin"),
        }
        Some(path)
    }

    /// Triggers pre-computation of assets for the specified captcha style.
    pub fn warm_up(&self, style: CaptchaStyle) {
        match style {
            CaptchaStyle::Rotate => self.warm_up_rotate(),
            CaptchaStyle::Pair => self.warm_up_pair(),
            CaptchaStyle::Slider => self.warm_up_slider(),
        }
    }

    fn warm_up_rotate(&self) {
        let cache_path_opt = Self::get_cache_path(CaptchaStyle::Rotate);
        let config = bincode_next::config::standard().with_limit::<{ 50 * 1024 * 1024 }>();

        if let Some(ref cache_path) = cache_path_opt
            && cache_path.exists()
            && let Ok(file) = std::fs::File::open(cache_path)
        {
            let mut reader = std::io::BufReader::new(file);
            if let Ok(data) = bincode_next::decode_from_std_read::<
                HashMap<String, Vec<Vec<u8>>>,
                _,
                _,
            >(&mut reader, config)
            {
                for (name, variants) in data {
                    if let Some(lock) = self.rotate_cache.get(&name)
                        && let Ok(mut write) = lock.write()
                        && write.is_empty()
                    {
                        *write = variants;
                    }
                }
                return;
            }
        }

        let computed: HashMap<String, Vec<Vec<u8>>> = self
            .rotate_cache
            .par_iter()
            .map(|(name, lock)| {
                if let Ok(read) = lock.read()
                    && !read.is_empty()
                {
                    return (name.clone(), read.clone());
                }

                let mut variants = Vec::with_capacity(10);
                for _ in 0..10 {
                    if let Some(bin) = self.generate_rotate_variant(name) {
                        variants.push(bin);
                    }
                }

                if let Ok(mut write) = lock.write()
                    && write.is_empty()
                {
                    write.extend(variants.clone());
                }

                (name.clone(), variants)
            })
            .collect();

        if let Some(cache_path) = cache_path_opt
            && let Ok(file) = std::fs::File::create(&cache_path)
        {
            let mut writer = std::io::BufWriter::new(file);
            let _ = bincode_next::encode_into_std_write(computed, &mut writer, config);
        }
    }

    fn warm_up_pair(&self) {
        let cache_path_opt = Self::get_cache_path(CaptchaStyle::Pair);
        let config = bincode_next::config::standard().with_limit::<{ 50 * 1024 * 1024 }>();

        if let Some(ref cache_path) = cache_path_opt
            && cache_path.exists()
            && let Ok(file) = std::fs::File::open(cache_path)
        {
            let mut reader = std::io::BufReader::new(file);
            if let Ok(data) = bincode_next::decode_from_std_read::<
                HashMap<String, Vec<Vec<u8>>>,
                _,
                _,
            >(&mut reader, config)
            {
                for (name, variants) in data {
                    if let Some(lock) = self.pair_cache.get(&name)
                        && let Ok(mut write) = lock.write()
                        && write.is_empty()
                    {
                        *write = variants;
                    }
                }
                return;
            }
        }

        let computed: HashMap<String, Vec<Vec<u8>>> = self
            .pair_cache
            .par_iter()
            .map(|(name, lock)| {
                if let Ok(read) = lock.read()
                    && !read.is_empty()
                {
                    return (name.clone(), read.clone());
                }

                let mut variants = Vec::with_capacity(10);
                for _ in 0..10 {
                    if let Some(bin) = self.generate_pair_variant(name) {
                        variants.push(bin);
                    }
                }

                if let Ok(mut write) = lock.write()
                    && write.is_empty()
                {
                    write.extend(variants.clone());
                }

                (name.clone(), variants)
            })
            .collect();

        if let Some(cache_path) = cache_path_opt
            && let Ok(file) = std::fs::File::create(&cache_path)
        {
            let mut writer = std::io::BufWriter::new(file);
            let _ = bincode_next::encode_into_std_write(computed, &mut writer, config);
        }
    }

    fn warm_up_slider(&self) {
        let cache_path_opt = Self::get_cache_path(CaptchaStyle::Slider);
        let config = bincode_next::config::standard().with_limit::<{ 50 * 1024 * 1024 }>();

        if let Some(ref cache_path) = cache_path_opt
            && cache_path.exists()
            && let Ok(file) = std::fs::File::open(cache_path)
        {
            let mut reader = std::io::BufReader::new(file);
            if let Ok(data) = bincode_next::decode_from_std_read::<
                HashMap<String, Vec<SliderVariation>>,
                _,
                _,
            >(&mut reader, config)
            {
                for (name, variants) in data {
                    if let Some(lock) = self.slider_cache.get(&name)
                        && let Ok(mut write) = lock.write()
                        && write.is_empty()
                    {
                        *write = variants;
                    }
                }
                return;
            }
        }

        let computed: HashMap<String, Vec<SliderVariation>> = self
            .slider_cache
            .par_iter()
            .map(|(name, lock)| {
                if let Ok(read) = lock.read()
                    && !read.is_empty()
                {
                    return (name.clone(), read.clone());
                }

                let mut variants = Vec::with_capacity(15);
                for _ in 0..15 {
                    if let Some(var) = self.generate_slider_variant(name) {
                        variants.push(var);
                    }
                }

                if let Ok(mut write) = lock.write()
                    && write.is_empty()
                {
                    write.extend(variants.clone());
                }

                (name.clone(), variants)
            })
            .collect();

        if let Some(cache_path) = cache_path_opt
            && let Ok(file) = std::fs::File::create(&cache_path)
        {
            let mut writer = std::io::BufWriter::new(file);
            let _ = bincode_next::encode_into_std_write(computed, &mut writer, config);
        }
    }

    /// Returns a random pre-computed variation of the specified image.
    #[must_use]
    pub fn get_image(&self, name: &str) -> Option<Vec<u8>> {
        let lock = self.rotate_cache.get(name)?;
        let read = lock
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if read.is_empty() {
            return None;
        }
        let idx = crate::common::get_random_index(read.len());
        read.get(idx).cloned()
    }

    /// Returns a random pre-computed variation of the specified image for Pair challenges.
    #[must_use]
    pub fn get_pair_image(&self, name: &str) -> Option<Vec<u8>> {
        let lock = self.pair_cache.get(name)?;
        let read = lock
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if read.is_empty() {
            return None;
        }
        let idx = crate::common::get_random_index(read.len());
        read.get(idx).cloned()
    }

    /// Returns a random pre-computed slider variation (main image and piece).
    #[must_use]
    pub fn get_slider_pair(&self, name: &str) -> Option<(u8, u8, Vec<u8>, Vec<u8>)> {
        let lock = self.slider_cache.get(name)?;
        let read = lock
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if read.is_empty() {
            return None;
        }
        let idx = crate::common::get_random_index(read.len());
        read.get(idx).cloned()
    }

    /// Returns a list of all available asset identifiers.
    #[must_use]
    pub fn all_image_names(&self) -> &[String] {
        &self.names
    }
}

/// Initializes the global asset cache with default intensity.
///
/// This operation initializes the singleton and pre-computes variations if not already cached.
/// Variations are stored persistently in the platform's standard cache directory
/// (e.g., `~/.cache/ancaptcha` on Linux) to speed up subsequent restarts.
pub fn init_cache() -> &'static AssetCache {
    ASSET_CACHE.get_or_init(|| AssetCache::new(NoiseIntensity::Medium))
}

/// Initializes the global asset cache with the specified noise intensity.
pub fn init_with_intensity(intensity: NoiseIntensity) -> &'static AssetCache {
    ASSET_CACHE.get_or_init(|| AssetCache::new(intensity))
}

/// Returns a reference to the global asset cache if initialized.
#[must_use]
pub fn get_cache() -> Option<&'static AssetCache> {
    ASSET_CACHE.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_init() {
        let cache = init_cache();
        let names = cache.all_image_names();
        assert!(!names.is_empty());
    }

    #[test]
    fn singleton_behavior() {
        let cache1 = init_cache();
        let cache2 = init_cache();
        assert!(std::ptr::eq(cache1, cache2));
    }

    #[test]
    fn nonexistent_image() {
        let cache = init_cache();
        let result = cache.get_image("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn granular_warmup() {
        let cache = AssetCache::new(NoiseIntensity::Medium);
        let all_names = cache.all_image_names();
        let Some(sample) = all_names.first() else {
            return;
        };

        let is_cold = cache
            .rotate_cache
            .get(sample)
            .and_then(|lock| lock.read().ok())
            .is_none_or(|read| read.is_empty());
        assert!(is_cold);

        cache.warm_up(CaptchaStyle::Rotate);

        let is_warm = cache
            .rotate_cache
            .get(sample)
            .and_then(|lock| lock.read().ok())
            .is_some_and(|read| !read.is_empty());
        assert!(is_warm);

        let slider_cold = cache
            .slider_cache
            .get(sample)
            .and_then(|lock| lock.read().ok())
            .is_none_or(|read| read.is_empty());
        assert!(slider_cold);
    }

    #[test]
    fn asset_size_limits() {
        let cache = init_cache();
        cache.warm_up(CaptchaStyle::Rotate);
        cache.warm_up(CaptchaStyle::Pair);
        cache.warm_up(CaptchaStyle::Slider);

        for name in cache.all_image_names() {
            assert!(
                cache.get_image(name).is_some_and(|b| b.len() <= 4500),
                "rotate_img_invalid: {name}"
            );

            assert!(
                cache.get_pair_image(name).is_some_and(|b| b.len() <= 2000),
                "pair_img_invalid: {name}"
            );

            assert!(
                cache
                    .get_slider_pair(name)
                    .is_some_and(|(_, _, m, p)| { m.len() <= 4500 && p.len() <= 4500 }),
                "slider_pair_invalid: {name}"
            );
        }
    }
}
