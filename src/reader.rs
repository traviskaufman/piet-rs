//! Reads an image
//! Currently only supports 1px codels

use std::error::Error;
use std::path::Path;

use image::{self, RgbImage};

pub fn read<P>(path: P) -> Result<RgbImage, String>
    where P: AsRef<Path>
{
    match image::open(path) {
        Ok(img) => Ok(img.to_rgb()),
        Err(e) => Err(format!("Could not open image: {}", e.description())),
    }
}
