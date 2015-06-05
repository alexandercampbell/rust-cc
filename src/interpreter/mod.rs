/*!
 * Interpreter
 * ===========
 *
 * This module provides the functions for evaluating an `ast::Program` in memory. This has the
 * advantage of being easier to write than a compiler, but it also has three large disadvantages:
 *
 * 1. Slow as fuck.
 * 2. Can't link against the standard library (or any library).
 * 3. Isn't actually a compiler.
 *
 * The advantages, are, of course:
 *
 * 1. Easier to write than an actual compiler.
 * 2. Easier debugging (because the Interpreter has more state information).
 * 3. More portable because we're not emitting any platform-specific assembly.
 *
 */

use ast;
use checker;

/**
 * Return the main() function from the program, if it exists.
 */
fn get_main(program: &ast::Program) -> Option<&ast::Function> {
    program.functions.iter().find(|&f| f.name == "main")
}

/**
 * Interpret the program, starting at main().
 */
pub fn run_program(program: &ast::Program) -> Result<(), String> {
    try!(checker::check_program(program));
    let main = match get_main(program) {
        Some(f) => f,
        None => return Err("no main function found in program".to_string()),
    };
    let _ = main;
    Ok(())
}

