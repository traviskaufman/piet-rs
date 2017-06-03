extern crate image;

use std::env;
use std::io;
use std::io::prelude::*;

fn run_app() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Not enough arguments"));
    }

    let fname = &args[1];
    let img = try!(image::open(fname).map_err(|_| "Could not read image")).to_rgb();
    println!("{:?}", img);
    Ok(())
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
