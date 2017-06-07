use std::collections::HashSet;

use image::RgbImage;

use state::{Position, Direction};
use util;

// See: https://en.wikipedia.org/wiki/Flood_fill
// Inspired by how npiet does color block checking
fn flood_check(img: &RgbImage, x: i32, y: i32, mut blk: &mut ColorBlock) {
    let out_of_bounds = x < 0 || y < 0 || (x as u32) == img.width() || (y as u32) == img.height();
    if out_of_bounds {
        return;
    }
    let pos = Position {
        left: x as u32,
        top: y as u32,
    };
    if blk.codels.contains(&pos) {
        return;
    }

    if util::get_px(&img, &pos) != blk.color {
        return;
    }

    blk.codels.insert(pos);

    // TODO(perf): Use vecdeque-based method
    // South
    flood_check(&img, x, y + 1, &mut blk);
    // North
    flood_check(&img, x, y - 1, &mut blk);
    // East
    flood_check(&img, x - 1, y, &mut blk);
    // West
    flood_check(&img, x + 1, y, &mut blk);
}

#[derive(Debug)]
pub struct ColorBlock {
    pub color: (u8, u8, u8),
    codels: HashSet<Position>,
}

impl ColorBlock {
    pub fn from_position_in_img(img: &RgbImage, pos: &Position) -> ColorBlock {
        let mut blk = ColorBlock {
            color: (0, 0, 0),
            codels: HashSet::new(),
        };
        let target_color = util::get_px(&img, &pos);

        blk.color = target_color;
        // Note that initial direction here does not matter
        flood_check(&img, pos.left as i32, pos.top as i32, &mut blk);
        blk
    }

    pub fn value(&self) -> u32 {
        self.codels.len() as u32
    }

    // TODO: See if there's a better way to do this than what this, which is copied from
    // Piet_py.txt but that I don't necessarily understand what it's doing.
    // TODO: THIS IS STILL WRONG
    pub fn boundary_codel_position(&self, dp: &Direction, cc: &Direction) -> Position {
        let initially_sort_x = match *dp {
            Direction::Up | Direction::Down => true,
            _ => false,
        };
        let subsequently_sort_x = !initially_sort_x;
        let reverse_first_sort = match (*dp, *cc) {
            (Direction::Right, Direction::Right) => true,
            (Direction::Down, Direction::Right) => true,
            (Direction::Left, Direction::Left) => true,
            (Direction::Up, Direction::Right) => true,
            _ => false,
        };
        let reverse_second_sort = match *dp {
            Direction::Right | Direction::Down => true,
            _ => false,
        };

        let mut cvec: Vec<&Position> = self.codels.iter().collect();
        cvec.sort_by(|p1, p2| {
            let order = if initially_sort_x {
                p1.left.cmp(&p2.left)
            } else {
                p1.top.cmp(&p2.top)
            };
            if reverse_first_sort {
                order.reverse()
            } else {
                order
            }
        });
        cvec.sort_by(|p1, p2| {
            let order = if subsequently_sort_x {
                p1.left.cmp(&p2.left)
            } else {
                p1.top.cmp(&p2.top)
            };
            if reverse_second_sort {
                order.reverse()
            } else {
                order
            }
        });

        *cvec[0]
    }
}
