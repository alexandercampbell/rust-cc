#![feature(box_syntax)]

use std::env;
use std::path::Path;

mod ast;
mod checker;
mod interpreter;
mod parser;
mod source;
mod util;

#[allow(dead_code)] // This shouldn't be necessary, but otherwise `cargo test` complains.
fn main() {
    // The first parameter is the filename of the C program we're going to parse.
    let input_filename = env::args().skip(1).next().unwrap();

    // Load the file.
    let file = source::File::from_disk(Path::new(&input_filename)).unwrap();

    // Parse it into an AST (see `ast.rs`)
    let program = parser::parse_str(&file.buf).unwrap();
    println!("parsed an AST {:?}", program);

    // Run the program loaded in the AST.
    interpreter::run_program(&program).unwrap();
}

