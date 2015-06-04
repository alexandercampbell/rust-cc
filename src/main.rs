#![feature(box_syntax)]

use std::env;
use std::path::Path;
extern crate getopts;

mod ast;
mod checker;
mod interpreter;
mod parser;
mod source;

#[allow(dead_code)] // This shouldn't be necessary, but otherwise `cargo test` complains.
fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("o", "", "output binary location", "a.out");

    let matches = opts.parse(env::args().skip(1)).unwrap();
    let output_filename = matches.opt_str("o").unwrap_or("a.out".to_string());

    // We're not emitting any machine code yet, so no output file is being created.
    let _ = output_filename;

    if matches.free.len() != 1 {
        panic!("expected exactly one argument (the filename of a C program)")
    }
    let input_filename = matches.free[0].clone();

    let path = Path::new(&input_filename);
    let file = source::File::from_disk(path).unwrap();
    let tokens = parser::lexer::lex(&file.buf).unwrap();
    let root_ast_node = parser::parse(tokens).unwrap();
    println!("parsed an AST {:?}", root_ast_node);

    // interpreter will call checker
    interpreter::run_program(&root_ast_node).unwrap();
}

