//! Image distortion and noise application filters.

use crate::config::NoiseIntensity;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

/// Shifts pixels randomly within a range determined by the noise intensity.
#[must_use]
pub fn apply_pixel_jitter(img: &DynamicImage, intensity: &NoiseIntensity) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);
    let jitter = i32::from(intensity.jitter_amount());

    if jitter == 0 {
        return img.clone();
    }

    for y in 0..height {
        for x in 0..width {
            let offset_x = crate::common::get_random_range(-jitter..=jitter);
            let offset_y = crate::common::get_random_range(-jitter..=jitter);

            let src_x = (i32::try_from(x).unwrap_or(0) + offset_x)
                .clamp(0, i32::try_from(width).unwrap_or(0) - 1);
            let src_y = (i32::try_from(y).unwrap_or(0) + offset_y)
                .clamp(0, i32::try_from(height).unwrap_or(0) - 1);

            let pixel = img.get_pixel(
                u32::try_from(src_x).unwrap_or(0),
                u32::try_from(src_y).unwrap_or(0),
            );
            output.put_pixel(x, y, pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// Randomly alters RGB values of pixels to distort color patterns.
#[must_use]
pub fn apply_color_shift(img: &DynamicImage, intensity: &NoiseIntensity) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);
    let shift = i16::from(intensity.color_shift_amount());

    if shift == 0 {
        return img.clone();
    }

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let Rgba([r, g, b, a]) = pixel;

            let r_shift = crate::common::get_random_range(-shift..=shift);
            let g_shift = crate::common::get_random_range(-shift..=shift);
            let b_shift = crate::common::get_random_range(-shift..=shift);

            let new_r = crate::common::clamp_to_u8(i16::from(r) + r_shift);
            let new_g = crate::common::clamp_to_u8(i16::from(g) + g_shift);
            let new_b = crate::common::clamp_to_u8(i16::from(b) + b_shift);

            output.put_pixel(x, y, Rgba([new_r, new_g, new_b, a]));
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// Randomly sets pixels to white or black based on a specified probability.
#[must_use]
pub fn apply_salt_pepper(img: &DynamicImage, intensity: &NoiseIntensity) -> DynamicImage {
    let (width, height) = img.dimensions();
    let mut output = ImageBuffer::new(width, height);
    let probability = intensity.salt_pepper_probability();

    if probability <= f32::EPSILON {
        return img.clone();
    }

    for y in 0..height {
        for x in 0..width {
            let mut pixel = img.get_pixel(x, y);

            let random_val: f32 = crate::common::get_random_probability();
            if random_val < probability {
                let is_white: bool = crate::common::get_random_bool();
                let value = if is_white { 255 } else { 0 };
                pixel = Rgba([value, value, value, pixel[3]]);
            }

            output.put_pixel(x, y, pixel);
        }
    }

    DynamicImage::ImageRgba8(output)
}

/// Sequentially applies all available noise filters to the image.
#[must_use]
pub fn apply_full_noise(img: &DynamicImage, intensity: &NoiseIntensity) -> DynamicImage {
    let noisy = apply_pixel_jitter(img, intensity);
    let noisy = apply_color_shift(&noisy, intensity);
    apply_salt_pepper(&noisy, intensity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_jitter() {
        let mut img = ImageBuffer::new(10, 10);
        for (x, _y, pixel) in img.enumerate_pixels_mut() {
            if x % 2 == 0 {
                *pixel = Rgba([255, 255, 255, 255]);
            } else {
                *pixel = Rgba([0, 0, 0, 255]);
            }
        }
        let dynamic_img = DynamicImage::ImageRgba8(img.clone());
        let result = apply_pixel_jitter(&dynamic_img, &NoiseIntensity::High);
        assert_eq!(result.dimensions(), (10, 10));
        assert_ne!(result.to_rgba8().into_raw(), img.into_raw());
    }

    #[test]
    fn color_shift() {
        let mut img = ImageBuffer::new(10, 10);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 100, 100, 255]);
        }
        let dynamic_img = DynamicImage::ImageRgba8(img.clone());
        let result = apply_color_shift(&dynamic_img, &NoiseIntensity::High);
        assert_eq!(result.dimensions(), (10, 10));
        assert_ne!(result.to_rgba8().into_raw(), img.into_raw());
    }

    #[test]
    fn salt_pepper() {
        let mut img = ImageBuffer::new(100, 100);
        for pixel in img.pixels_mut() {
            *pixel = Rgba([100, 100, 100, 255]);
        }
        let dynamic_img = DynamicImage::ImageRgba8(img.clone());

        let mut changed = false;
        let original_raw = img.as_raw();
        for _ in 0..10 {
            let result = apply_salt_pepper(&dynamic_img, &NoiseIntensity::High);
            if result.to_rgba8().as_raw() != original_raw {
                changed = true;
                break;
            }
        }
        assert!(changed);
    }

    #[test]
    fn noise_modifies_pixels() {
        let mut img = ImageBuffer::new(100, 100);
        for p in img.pixels_mut() {
            *p = Rgba([100, 100, 100, 255]);
        }
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let result = apply_full_noise(&dynamic_img, &NoiseIntensity::High);
        let result_buf = result.to_rgba8();
        let mut changed_count = 0;
        for (_x, _y, pixel) in result_buf.enumerate_pixels() {
            if pixel[0] != 100 || pixel[1] != 100 || pixel[2] != 100 {
                changed_count += 1;
            }
        }
        assert!(changed_count > 0);
    }
}
