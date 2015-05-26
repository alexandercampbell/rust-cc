
use std::env;
use std::path::Path;
extern crate getopts;

mod ast;
mod lexer;
mod parser;
mod source;

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

    let path = Path::new(&input_filename);
    let file = source::File::from_disk(path).unwrap();
    let tokens = lexer::lex(&file.buf).unwrap();
    println!("lexed {} tokens", tokens.len());
}

