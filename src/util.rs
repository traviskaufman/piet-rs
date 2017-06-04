use image;

use state::Position;

pub fn get_px(img: &image::RgbImage, pos: &Position) -> (u8, u8, u8) {
    let pxi = img.get_pixel(pos.left, pos.top);
    (pxi[0], pxi[1], pxi[2])
}
