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

pub fn run_program(program: &ast::Program) -> Result<(), String> {
    try!(checker::check_program(program));
    let _ = program;
    Ok(())
}

