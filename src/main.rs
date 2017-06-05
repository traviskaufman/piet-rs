extern crate image;

use std::env;
use std::io;
use std::io::prelude::*;

pub mod reader;
pub mod state;
pub mod color_block;
pub mod util;

use color_block::ColorBlock;
use state::{State, Position, Direction};

fn would_hit_restriction(img: &image::RgbImage, state: &State) -> bool {
    if state.dp() == Direction::Left && state.pos.left == 0 ||
       state.dp() == Direction::Up && state.pos.top == 0 {
        return true;
    }

    let nextpos = state.peek_pos();
    if nextpos.left == img.width() || nextpos.top == img.height() {
        return true;
    }
    return util::get_px(&img, &nextpos) == (0, 0, 0);
}

fn exec_cmd(from_px: &(u8, u8, u8),
            to_px: &(u8, u8, u8),
            from_pos: &Position,
            to_pos: &Position,
            state: &State) {
    println!("exec_cmd: {} --> {} (DP: {:?}, CC: {:?})",
             from_pos,
             to_pos,
             state.dp(),
             state.cc());
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
        // Travel in the direction of the dp
        let blk = ColorBlock::from_position_in_img(&img, &state.pos);
        state.pos = blk.boundary_codel_for_direction(&state.dp());

        // Travel in the direction of the cc until we hit a new color
        // TODO: Find a way to move within a color block
        let codel_blk = ColorBlock::from_position_in_img(&img, &state.pos);
        state.pos = codel_blk.boundary_codel_for_direction(&state.codel_direction());

        // Boundary / end of program checks
        let orig_dp = state.dp();
        let orig_cc = state.cc();
        let mut toggle_cc = true;
        let mut first_restriction_check = true;
        while would_hit_restriction(&img, &state) {
            let is_end_of_program = !first_restriction_check && state.dp() == orig_dp &&
                                    state.cc() == orig_cc;
            if is_end_of_program {
                println!("END OF PROGRAM!");
                return Ok(());
            }

            first_restriction_check = false;
            if toggle_cc {
                state.toggle_cc();
                toggle_cc = false;
            } else {
                state.rot_clockwise();
                toggle_cc = true;
            }

            let new_dp_pos = ColorBlock::from_position_in_img(&img, &state.pos)
                .boundary_codel_for_direction(&state.dp());
            state.pos = ColorBlock::from_position_in_img(&img, &new_dp_pos)
                .boundary_codel_for_direction(&state.codel_direction());
            println!("Hit restriction: new state: {:?}", state);
        }

        // Advance to next color block and exec color cmd
        let last_pos = state.pos;
        state.advance();
        let nextcolor = util::get_px(&img, &state.pos);
        exec_cmd(&blk.color, &nextcolor, &last_pos, &state.pos, &state);
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
