extern crate image;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn run_app() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(String::from("Not enough arguments"));
    }

    let fname = &args[1];
    let mut f = try!(File::open(fname).map_err(|_| "Could not open file"));
    let mut buf: Vec<u8> = vec![];
    try!(f.read_to_end(&mut buf).map_err(|_| "Could not read file"));

    match image::guess_format(&buf) {
        Ok(_) => {
            println!("Detected type {:?}", image::guess_format(&buf).unwrap());
            Ok(())
        }
        Err(_) => Err(String::from("Unable to detect image type")),
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
