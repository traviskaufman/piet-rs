// TODO: Handle white
// TODO: Finish Commands
// TODO: Refactor into interpreter iterator
// TODO: Tests
// TODO: Docs

extern crate ansi_term;
extern crate image;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::io;
use std::io::prelude::*;

pub mod reader;
pub mod state;
pub mod color;
pub mod color_block;
pub mod util;

use color_block::ColorBlock;
use color::{Hue, Lightness};
use state::{State, Position, Direction};

macro_rules! get {
    ($e: expr) => (match $e { Some(e) => e, None => return ()});
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Number,
    Char,
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Nop,
    Push,
    Pop,
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Not,
    Greater,
    Pointer,
    Switch,
    Duplicate,
    Roll,
    In(DataType),
    Out(DataType),
}

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
            from_pos: Position,
            to_pos: Position,
            state: &mut State,
            from_blk: &ColorBlock) {
    static COMMAND_MATRIX: [[Command; 3]; 6] =
        [[Command::Nop, Command::Push, Command::Pop],
         [Command::Add, Command::Subtract, Command::Multiply],
         [Command::Divide, Command::Mod, Command::Not],
         [Command::Greater, Command::Pointer, Command::Switch],
         [Command::Duplicate, Command::Roll, Command::In(DataType::Number)],
         [Command::In(DataType::Char),
          Command::Out(DataType::Number),
          Command::Out(DataType::Char)]];
    let from_color = color::Color::from_px(from_px).unwrap();
    let to_color = color::Color::from_px(to_px).unwrap();
    // FIXME: Change logic wrong
    let hue_change = match (from_color.hue, to_color.hue) {
        (Hue::Magenta, Hue::Red) => 1,
        (from_hue, to_hue) => {
            let (from_i, to_i) = (from_hue as i8, to_hue as i8);
            if to_i < from_i {
                6 - (from_i - to_i)
            } else {
                to_i - from_i
            }
        }
    };
    let lightness_change = match (from_color.lightness, to_color.lightness) {
        (Lightness::Dark, Lightness::Light) => 1,
        (from_l, to_l) => {
            let (from_i, to_i) = (from_l as i8, to_l as i8);
            if to_i < from_i {
                3 - (from_i - to_i)
            } else {
                to_i - from_i
            }
        }
    };
    let cmd = COMMAND_MATRIX[hue_change as usize][lightness_change as usize];
    info!("exec_cmd: {:?} -- {} @ {} --> {} @ {} (DP: {:?}, CC: {:?})",
          cmd,
          from_color,
          from_pos,
          to_color,
          to_pos,
          state.dp(),
          state.cc());
    match cmd {
        Command::Nop => (),
        Command::Push => {
            state.stack.push(from_blk.value());
        }
        Command::Pop => {
            state.stack.pop();
        }
        Command::Add => {
            let lhs = get!(state.stack.pop());
            let rhs = get!(state.stack.pop());
            state.stack.push(lhs + rhs);
        }
        Command::Subtract => {
            let rhs = get!(state.stack.pop());
            let lhs = get!(state.stack.pop());
            state.stack.push(lhs - rhs);
        }
        Command::Multiply => {
            let lhs = get!(state.stack.pop());
            let rhs = get!(state.stack.pop());
            state.stack.push(lhs * rhs);
        }
        Command::Divide | Command::Mod => {
            let divisor = get!(state.stack.pop());
            let dividend = get!(state.stack.pop());
            if divisor == 0 {
                return;
            }
            let res = match cmd {
                Command::Divide => dividend / divisor,
                _ => dividend % divisor,
            };
            state.stack.push(res);
        }
        Command::Not => {
            let top = get!(state.stack.pop());
            state.stack.push(if top == 0 { 1 } else { 0 });
        }
        Command::Greater => {
            let first = get!(state.stack.pop());
            let second = get!(state.stack.pop());
            state.stack.push(if second > first { 1 } else { 0 });
        }
        Command::Pointer => {
            const NUM_DIRECTIONS: i32 = 4;
            let n_rotations = get!(state.stack.pop()) % NUM_DIRECTIONS;
            let rotate_clockwise = n_rotations > 0;
            for _ in 0..n_rotations.abs() {
                if rotate_clockwise {
                    state.rot_clockwise();
                } else {
                    state.rot_counterclockwise();
                }
            }
        }
        Command::Switch => {
            let n = get!(state.stack.pop());
            for _ in 0..n {
                state.toggle_cc();
            }
        }
        Command::Duplicate => {
            let last = get!(state.stack.pop());
            state.stack.push(last);
            state.stack.push(last);
        }
        Command::Roll => {
            let mut num_rolls = get!(state.stack.pop());
            let mut depth = get!(state.stack.pop()) - 1;
            let len = state.stack.len() as i32;
            trace!("ROLL: {}, {}, {}", num_rolls, depth, len);
            trace!("ROLL STACK: {:?}", state.stack);
            if depth < 0 {
                return;
            }
            // NOTE: This may not be right, but it's how I interpret the spec
            if depth >= len {
                return;
            }
            if num_rolls < 0 {
                num_rolls = -num_rolls;
                depth = len - num_rolls;
            }
            for _ in 0..num_rolls {
                let mut d = 0;
                while d < depth {
                    let offset: usize = (len - d - 1) as usize;
                    state.stack.swap(offset, offset - 1);
                    d += 1;
                }
            }
        }
        Command::In(dtype) => {
            let mut input = String::new();
            get!(std::io::stdin().read_line(&mut input).ok());
            let n: i32 = get!(input.parse().ok());
            if dtype == DataType::Char {
                get!(std::char::from_u32(n as u32));
            }
            state.stack.push(n);
        }
        Command::Out(dtype) => {
            let output = get!(state.stack.pop());
            match dtype {
                DataType::Number => write!(std::io::stdout(), "{}", output).ok(),
                DataType::Char => {
                    let outchar = get!(std::char::from_u32(output as u32));
                    write!(std::io::stdout(), "{}", outchar).ok()
                }
            };
        }
    }
    trace!("STACK: {:?}", state.stack);
}

fn run_app() -> Result<(), String> {
    try!(env_logger::init().map_err(|_| "Could not instantiate logger"));
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Not enough arguments"));
    }

    let fname = &args[1];
    let img = try!(reader::read(fname));

    // interpret code
    let mut state = State::new();
    loop {
        let mut seen_white = false;
        let blk = ColorBlock::from_position_in_img(&img, &state.pos);
        state.pos = blk.boundary_codel_position(&state.dp(), &state.cc());

        if blk.color == (255, 255, 255) {
            debug!("Sliding through white color block");
            seen_white = true;
            while !would_hit_restriction(&img, &state) &&
                  util::get_px(&img, &state.peek_pos()) == (0, 0, 0) {
                state.advance();
            }
        }

        // Boundary / end of program checks
        let orig_dp = state.dp();
        let orig_cc = state.cc();
        let mut toggle_cc = true;
        let mut first_restriction_check = true;
        while would_hit_restriction(&img, &state) {
            let is_end_of_program = !first_restriction_check && state.dp() == orig_dp &&
                                    state.cc() == orig_cc;
            if is_end_of_program {
                info!("END OF PROGRAM!");
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

            state.pos = ColorBlock::from_position_in_img(&img, &state.pos)
                .boundary_codel_position(&state.dp(), &state.cc());
        }

        // Advance to next color block and exec color cmd
        let last_pos = state.pos;
        state.advance();
        let nextcolor = util::get_px(&img, &state.pos);
        seen_white = seen_white || nextcolor == (255, 255, 255);

        if !seen_white {
            debug!("Seen white: not executing command");
            exec_cmd(&blk.color,
                     &nextcolor,
                     last_pos,
                     state.pos,
                     &mut state,
                     &blk);
        }
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
