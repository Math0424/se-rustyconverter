use std::io::Cursor;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageError, Pixels, RgbImage};

use crate::window_options::InterpolationMode;

pub fn process_lcd_image(path: String, filter: InterpolationMode, preserve_aspect: bool, x: usize, y: usize) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;
    if preserve_aspect {
        Ok(img.resize(x as u32, y as u32, filter.convert()))
    } else {
        Ok(img.resize_exact(x as u32, y as u32, filter.convert()))
    }
}

pub fn to_raw_pixels(image: DynamicImage) -> (Vec<u8>, (u32, u32)) {
    let rgba8: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image.to_rgba8();
    let dim: (u32, u32) = image.dimensions();
    (rgba8.into_raw(), dim)
}