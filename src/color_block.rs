use std::collections::HashSet;

use image::RgbImage;

use state::{Position, Direction};
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

    blk.num_codels += 1;

    // TODO: Change
    // What we need, the *topmost*, *bottommost*, *leftmost*, and *rightmost* edges of the color
    // block. That is:
    // - topmost_edge: The min `top` value of any codel in the block
    // - bottommost_edge: The max `top` value of any codel in the block
    // - leftmost_edge: The min `left` value of any codel in the block
    // - rightmost_edge: The max `left` value of any codel in the block
    //
    // Given those, we can scan that dimension and determine the following:
    // - DP=UP && CC=LEFT => Scan along topmost edge and find the min `left`
    // - DP=DOWN && CC=LEFT => Scan along bottommost edge and find the max `left` codel
    // - DP=LEFT && CC=LEFT => Scan along leftmost edge and find max `top` codel
    // - DP=RIGHT && CC=LEFT => Scan along rightmost edge and find min `top` codel
    // - DP=UP && CC=RIGHT => Scan along topmost edge to find max `left` codel
    // - DP=DOWN && CC=RIGHT => Scan along bottommost edge and find min `left` codel
    // - DP=LEFT && CC=RIGHT => Scan along leftmost edge and find min `top` codel
    // - DP=RIGHT && CC=RIGHT => Scan along rightmost edge and find max `top` codel
    // - NOTE: Save coords in a HashSet so that you can check membership of color block when traversing

    // NOTE: we do left/top equality checks below so that we know that the dp can travel to it
    if pos.left < blk.furthest_left_codel.left && pos.top == blk.furthest_left_codel.top {
        blk.furthest_left_codel = pos;
    } else if pos.left > blk.furthest_right_codel.left && pos.top == blk.furthest_right_codel.top {
        blk.furthest_right_codel = pos;
    }

    if pos.top < blk.furthest_up_codel.top && pos.left == blk.furthest_up_codel.left {
        blk.furthest_up_codel = pos;
    } else if pos.top > blk.furthest_down_codel.top && pos.left == blk.furthest_down_codel.left {
        blk.furthest_down_codel = pos;
    }

    // TODO(perf): Use vecdeque-based method
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
    pub furthest_left_codel: Position,
    pub furthest_right_codel: Position,
    pub furthest_up_codel: Position,
    pub furthest_down_codel: Position,
    pub color: (u8, u8, u8),
    pub num_codels: u32,
}

impl ColorBlock {
    pub fn from_position_in_img(img: &RgbImage, pos: &Position) -> ColorBlock {
        let mut blk = ColorBlock {
            furthest_left_codel: *pos,
            furthest_right_codel: *pos,
            furthest_up_codel: *pos,
            furthest_down_codel: *pos,
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

    pub fn boundary_codel_for_direction(&self, dir: &Direction) -> Position {
        match *dir {
            Direction::Right => self.furthest_right_codel,
            Direction::Down => self.furthest_down_codel,
            Direction::Left => self.furthest_left_codel,
            Direction::Up => self.furthest_up_codel,
        }
    }
}
