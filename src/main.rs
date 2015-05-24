
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
    let output_file = matches.opt_str("o").unwrap_or("a.out".to_string());
    println!("{:?}", output_file);
}

