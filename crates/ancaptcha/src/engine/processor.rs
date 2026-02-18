//! Specialized image processing for challenge asset generation.

use image::{DynamicImage, GenericImageView, Rgba};

/// Extracts a square piece from an image and replaces the source area with transparency.
#[must_use]
pub fn create_slider_cutout(
    img: &DynamicImage,
    piece_x: u32,
    piece_y: u32,
    piece_size: u32,
) -> (DynamicImage, DynamicImage) {
    let mut main_img = img.to_rgba8();
    let piece_img = img
        .view(piece_x, piece_y, piece_size, piece_size)
        .to_image();

    for y in piece_y..piece_y + piece_size {
        for x in piece_x..piece_x + piece_size {
            main_img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
        }
    }

    (
        DynamicImage::ImageRgba8(main_img),
        DynamicImage::ImageRgba8(piece_img),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slider_cutout_gap_creation() {
        let img = DynamicImage::new_rgb8(100, 100);
        let (main, piece) = create_slider_cutout(&img, 25, 25, 20);

        assert_eq!(main.dimensions(), (100, 100));
        assert_eq!(piece.dimensions(), (20, 20));

        let pixel = main.get_pixel(30, 30);
        assert_eq!(pixel[3], 0);
    }
}
