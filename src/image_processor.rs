use std::io::Cursor;
use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, ImageError, Pixels, RgbImage};

use crate::window_options::InterpolationMode;

pub fn process_image(path: String, filter: InterpolationMode, x: usize, y: usize) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;
    let resized = img.resize(x as u32, y as u32, filter.convert());
    Ok(resized)
}

pub fn filter_image(image: DynamicImage) {
    
}
