//! Specialized image processing for challenge asset generation.

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};

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

#[must_use]
pub fn stitch_vertical(images: &[&[u8]], cell_w: u32, cell_h: u32) -> Option<Vec<u8>> {
    if images.is_empty() {
        return None;
    }
    let count = u32::try_from(images.len()).ok()?;
    let total_h = cell_h.checked_mul(count)?;
    let mut canvas = ImageBuffer::<Rgba<u8>, _>::new(cell_w, total_h);

    for (i, raw) in images.iter().enumerate() {
        let img = image::load_from_memory(raw).ok()?;
        let resized = img.resize_exact(cell_w, cell_h, image::imageops::FilterType::Triangle);
        let y_off = u32::try_from(i).ok()? * cell_h;
        canvas.copy_from(&resized.to_rgba8(), 0, y_off).ok()?;
    }

    encode_rgba_jpeg(&canvas, 13)
}

#[must_use]
pub fn stitch_grid(images: &[&[u8]], cell_size: u32, cols: u32) -> Option<Vec<u8>> {
    if images.is_empty() {
        return None;
    }
    let count = u32::try_from(images.len()).ok()?;
    let rows = count.div_ceil(cols);
    let total_w = cell_size.checked_mul(cols)?;
    let total_h = cell_size.checked_mul(rows)?;
    let mut canvas = ImageBuffer::<Rgba<u8>, _>::new(total_w, total_h);

    for (i, raw) in images.iter().enumerate() {
        let idx = u32::try_from(i).ok()?;
        let col = idx % cols;
        let row = idx / cols;
        let img = image::load_from_memory(raw).ok()?;
        let resized = img.resize_exact(cell_size, cell_size, image::imageops::FilterType::Triangle);
        canvas
            .copy_from(&resized.to_rgba8(), col * cell_size, row * cell_size)
            .ok()?;
    }

    encode_rgba_jpeg(&canvas, 10)
}

#[must_use]
pub fn stitch_horizontal(images: &[&[u8]], cell_w: u32, cell_h: u32) -> Option<Vec<u8>> {
    if images.is_empty() {
        return None;
    }
    let count = u32::try_from(images.len()).ok()?;
    let total_w = cell_w.checked_mul(count)?;
    let mut canvas = ImageBuffer::<Rgba<u8>, _>::new(total_w, cell_h);

    for (i, raw) in images.iter().enumerate() {
        let img = image::load_from_memory(raw).ok()?;
        let resized = img.resize_exact(cell_w, cell_h, image::imageops::FilterType::Triangle);
        let x_off = u32::try_from(i).ok()? * cell_w;
        canvas.copy_from(&resized.to_rgba8(), x_off, 0).ok()?;
    }

    encode_rgba_jpeg(&canvas, 13)
}

fn encode_rgba_jpeg(buf: &ImageBuffer<Rgba<u8>, Vec<u8>>, quality: u8) -> Option<Vec<u8>> {
    let mut out = std::io::Cursor::new(Vec::new());
    let enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, quality);
    DynamicImage::ImageRgba8(buf.clone())
        .write_with_encoder(enc)
        .ok()?;
    Some(out.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn slider_cutout_gap_creation() {
        let img = DynamicImage::new_rgb8(100, 100);
        let (main, piece) = create_slider_cutout(&img, 25, 25, 20);

        assert_eq!(main.dimensions(), (100, 100));
        assert_eq!(piece.dimensions(), (20, 20));

        let pixel = main.get_pixel(30, 30);
        assert_eq!(pixel[3], 0);
    }

    #[test]
    fn vertical_stitch() -> TestResult {
        let img1 = DynamicImage::new_rgb8(10, 10);
        let img2 = DynamicImage::new_rgb8(10, 10);
        let b1 = encode_test_jpeg(&img1)?;
        let b2 = encode_test_jpeg(&img2)?;
        let refs: Vec<&[u8]> = vec![&b1, &b2];
        let result = stitch_vertical(&refs, 10, 10).ok_or("stitch failed")?;
        let sprite = image::load_from_memory(&result)?;
        assert_eq!(sprite.dimensions(), (10, 20));
        Ok(())
    }

    #[test]
    fn grid_stitch() -> TestResult {
        let imgs: Vec<DynamicImage> = (0..9).map(|_| DynamicImage::new_rgb8(10, 10)).collect();
        let bufs: Vec<Vec<u8>> = imgs
            .iter()
            .map(|i| encode_test_jpeg(i))
            .collect::<std::result::Result<Vec<_>, _>>()?;
        let refs: Vec<&[u8]> = bufs.iter().map(Vec::as_slice).collect();
        let result = stitch_grid(&refs, 10, 3).ok_or("stitch failed")?;
        let sprite = image::load_from_memory(&result)?;
        assert_eq!(sprite.dimensions(), (30, 30));
        Ok(())
    }

    #[test]
    fn horizontal_stitch() -> TestResult {
        let img1 = DynamicImage::new_rgb8(10, 10);
        let img2 = DynamicImage::new_rgb8(10, 10);
        let img3 = DynamicImage::new_rgb8(10, 10);
        let b1 = encode_test_jpeg(&img1)?;
        let b2 = encode_test_jpeg(&img2)?;
        let b3 = encode_test_jpeg(&img3)?;
        let refs: Vec<&[u8]> = vec![&b1, &b2, &b3];
        let result = stitch_horizontal(&refs, 10, 10).ok_or("stitch failed")?;
        let sprite = image::load_from_memory(&result)?;
        assert_eq!(sprite.dimensions(), (30, 10));
        Ok(())
    }

    fn encode_test_jpeg(
        img: &DynamicImage,
    ) -> std::result::Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buf = std::io::Cursor::new(Vec::new());
        let enc = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 50);
        img.write_with_encoder(enc)?;
        Ok(buf.into_inner())
    }
}
