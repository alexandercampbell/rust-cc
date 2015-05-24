
use std::env;
extern crate getopts;

#[allow(dead_code)] // This shouldn't be necessary, but otherwise `cargo test` complains.
fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("o", "", "output binary location", "a.out");

    let matches = opts.parse(&args[1..]).unwrap();
    let output_file = matches.opt_str("o").unwrap_or("a.out".to_string());
    println!("{:?}", output_file);
}

