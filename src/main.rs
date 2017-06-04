extern crate image;

use std::env;
use std::io;
use std::io::prelude::*;

pub mod reader;
pub mod state;
pub mod color_block;
pub mod util;

use state::{State, Position, Direction};

// TODO: Account for black, rename to hit_restriction
fn out_of_bounds<I>(img: &I, pos: &Position) -> bool
    where I: image::GenericImage
{
    pos.left >= img.width() || pos.top >= img.height()
}

fn exec_cmd(from_px: &(u8, u8, u8), to_px: &(u8, u8, u8), from_pos: &Position, to_pos: &Position) {
    println!("exec_cmd: from {:?} @ {:?} --> {:?} @ {:?}",
             from_px,
             from_pos,
             to_px,
             to_pos);
}

fn run_app() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Not enough arguments"));
    }

    let fname = &args[1];
    let img = try!(reader::read(fname));

    // interpret code
    let mut state = State::new();
    loop {
        // Find the edge of the current color block in the furthest direction from the DP
        let px = util::get_px(&img, &state.pos);

        let mut nextpos = state.peek_pos();
        if out_of_bounds(&img, &nextpos) {
            let orig_dp = state.dp();
            let orig_cc = state.cc();
            loop {
                state.toggle_cc();
                nextpos = state.peek_pos();
                if !out_of_bounds(&img, &nextpos) {
                    break;
                }

                state.rot_clockwise();
                if !(state.dp() == Direction::Left && state.pos.left == 0 ||
                     state.dp() == Direction::Up && state.pos.top == 0) {
                    nextpos = state.peek_pos();
                    if !out_of_bounds(&img, &nextpos) {
                        break;
                    }
                }

                if state.dp() == orig_dp && state.cc() == orig_cc {
                    // End of Program
                    return Ok(());
                }
            }
        }

        let nextpx = util::get_px(&img, &nextpos);
        if nextpx != px {
            state.choosing_codel = true;
            let mut chosen_codel = px;
            loop {
                match state.codel_direction() {
                    Direction::Right => {
                        if state.pos.left + 1 == img.width() {
                            break;
                        }
                    }
                    Direction::Down => {
                        if state.pos.top + 1 == img.height() {
                            break;
                        }
                    }
                    Direction::Left => {
                        if state.pos.left == 0 {
                            break;
                        }
                    }
                    Direction::Up => {
                        if state.pos.top == 0 {
                            break;
                        }
                    }
                }
                state.advance();
                chosen_codel = util::get_px(&img, &state.pos);
            }
            state.choosing_codel = false;

            let new_block_pos = state.peek_pos();
            let new_block_px = util::get_px(&img, &new_block_pos);
            exec_cmd(&chosen_codel, &new_block_px, &state.pos, &new_block_pos);
        }

        state.advance();
    }
}

fn main() {
    ::std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            writeln!(io::stderr(), "error: {}", err).unwrap();
            1
        }
    });
}
