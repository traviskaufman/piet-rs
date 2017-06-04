use std::collections::HashSet;

use image::RgbImage;

use state::Position;
use util;

// See: https://en.wikipedia.org/wiki/Flood_fill
// Inspired by how npiet does color block checking
fn flood_check(img: &RgbImage,
               x: i32,
               y: i32,
               mut blk: &mut ColorBlock,
               mut seen_positions: &mut HashSet<(i32, i32)>) {
    let out_of_bounds = x < 0 || y < 0 || (x as u32) == img.width() || (y as u32) == img.height();
    if out_of_bounds {
        return;
    }
    if seen_positions.contains(&(x, y)) {
        return;
    }

    let pos = Position {
        left: x as u32,
        top: y as u32,
    };
    if util::get_px(&img, &pos) != blk.color {
        return;
    }

    seen_positions.insert((x, y));

    match pos {
        Position { left, top } if left <= blk.top_left_codel.left &&
                                  top <= blk.top_left_codel.top => {
            blk.top_left_codel = pos;
        }
        Position { left, top } if left >= blk.top_right_codel.left &&
                                  top <= blk.top_right_codel.top => {
            blk.top_right_codel = pos;
        }
        Position { left, top } if left <= blk.bottom_left_codel.left &&
                                  top >= blk.bottom_left_codel.top => {
            blk.bottom_left_codel = pos;
        }
        Position { left, top } if left >= blk.bottom_right_codel.left &&
                                  top >= blk.bottom_right_codel.top => {
            blk.bottom_right_codel = pos;
        }
        _ => (),
    }

    // TODO: Use vecdeque-based method for perf
    // South
    flood_check(&img, x, y + 1, &mut blk, &mut seen_positions);
    // North
    flood_check(&img, x, y - 1, &mut blk, &mut seen_positions);
    // East
    flood_check(&img, x - 1, y, &mut blk, &mut seen_positions);
    // West
    flood_check(&img, x + 1, y, &mut blk, &mut seen_positions);
}

#[derive(Debug)]
pub struct ColorBlock {
    top_left_codel: Position,
    top_right_codel: Position,
    bottom_left_codel: Position,
    bottom_right_codel: Position,
    color: (u8, u8, u8),
    num_codels: u32,
}

impl ColorBlock {
    pub fn from_position_in_img(img: &RgbImage, pos: &Position) -> ColorBlock {
        let mut blk = ColorBlock {
            top_left_codel: Position { left: 0, top: 0 },
            top_right_codel: Position { left: 0, top: 0 },
            bottom_left_codel: Position { left: 0, top: 0 },
            bottom_right_codel: Position { left: 0, top: 0 },
            color: (0, 0, 0),
            num_codels: 0,
        };
        let target_color = util::get_px(&img, &pos);

        blk.color = target_color;
        // Note that initial direction here does not matter
        flood_check(&img,
                    pos.left as i32,
                    pos.top as i32,
                    &mut blk,
                    &mut HashSet::new());
        blk
    }
}
