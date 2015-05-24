
use std::env;
extern crate getopts;

mod preprocessor;
mod lexer;
mod parser;

#[allow(dead_code)] // This shouldn't be necessary, but otherwise `cargo test` complains.
fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("o", "", "output binary location", "a.out");

    let matches = opts.parse(env::args().skip(1)).unwrap();
    let output_filename = matches.opt_str("o").unwrap_or("a.out".to_string());

    if matches.free.len() != 1 {
        panic!("expected exactly one argument (the filename of a C program)")
    }
    let input_filename = matches.free[0].clone();

    println!("reading from {:?}", input_filename);
    println!("writing to   {:?}", output_filename);
}

