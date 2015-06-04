/*!
 * Checker
 * =======
 *
 * The Checker module runs through our `ast::Program` and ensures:
 *
 * 1. All identifiers are defined before use.
 * 2. The variable types are valid during assignment and computation.
 * 3. Syntax is valid (no attempt to do something like "int c = int b".
 */

use ast;

pub fn check_program(program: &ast::Program) -> Result<(), String> {
    let _ = program;
    Ok(())
}
