/*!
 * This module contains the functions that build ASTs from Tokens.
 */

use lexer::Token;
use lexer::Operator;
use ast::*;

use super::context::Context;

/**
 * Parse declarations such as
 *
 * ```
 * const int b = 10;
 * int *a = &b;
 * int c[10] = { 0 };
 * unsigned char d, e;
 * int f = *a * b;
 * ```
 *
 * Into Declaration structs. The rule for this parse looks something like
 *
 * ```
 * ident+ (asterisk+ ident)? ((comma asterisk* ident)*|(equals <expression>)?)
 * ```
 *
 * A handcrafted parser may not be the most understandable way to build this construct :)
 */
fn declaration(context: &mut Context) -> Result<Declaration, String> {
    //
    // parse identifiers until we see
    //
    // 1. An asterisk. This means type definition is essentially over and variable names are
    //    next. Note that it is technically possible for multiple asterisks to appear.
    // 2. A semicolon. This means we have hit the end of a declaration. Step back a single token
    //    and use the last identifier as the variable name.
    // 3. A comma. This means the last identifier was a variable name and another variable name is
    //    coming.
    // 4. An equals sign. This means an initial value expression is coming. Parse that and then the
    //    semicolon afterward. Note that the last identifier before the equals sign was the
    //    variable name.
    //

    let mut identifiers: Vec<String> = vec![];

    let first_token = context.next();
    match first_token {
        Some(Token::Identifier(ident)) => identifiers.push(ident),
        _ => return Err("expected identifier at beginning of declaration".to_string()),
    };

    loop {
        match context.next() {
            // Keep pushing identifiers until we hit something else. This is the only match arm
            // that will actually continue the loop.
            Some(Token::Identifier(ident)) => identifiers.push(ident),

            // This is a very simple case: a series of identifiers followed by a Semicolon.
            // Example:
            //
            //      const unsigned long a;
            //
            Some(Token::Semicolon) => {
                if identifiers.len() < 2 {
                    return Err("expected at least two identifiers before semicolon".to_string());
                }

                // the following comments assume a declaration such as
                //
                //      const unsigned int my_integer;
                //
                let variable = identifiers.pop().unwrap();  // my_integer
                let base_name = identifiers.pop().unwrap(); // int
                let modifiers = identifiers;                // [const, unsigned]

                return Ok(Declaration{
                    _type:          Type{
                        base_name:      base_name,
                        modifiers:      modifiers,
                        length:         None,
                        pointer_levels: 0,
                    },
                    variable:       variable,
                    initial_value:  None,
                });
            },

            // This is a case where an asterisk interrupts the stream of tokens. This tells us some
            // important information about the identifiers we just grabbed. Example:
            //
            //      const int *b;
            //
            // We can easily infer that the previous token (int) was the base type, the tokens
            // before that were modifiers, and the token immediately after the asterisk is the
            // variable name. The last part there is not always true; sometimes multiple asterisks
            // are chained together as in this declaration:
            //
            //      const int ***c;
            //
            Some(Token::Operator(Operator::Asterisk)) => {
                let base_name = identifiers.pop().unwrap();
                let modifiers = identifiers;
                let mut pointer_levels = 1;
                let variable: String;

                loop {
                    // In this loop, we're looking for either another asterisk or an identifier
                    // (the variable name). This is only a loop because the number of asterisks can
                    // vary freely.
                    match context.next() {
                        Some(Token::Operator(Operator::Asterisk)) => pointer_levels += 1,
                        Some(Token::Identifier(string)) => {
                            variable = string;
                            break;
                        }
                        _ => return Err("expected either variable name or asterisk after asterisk in declaration".to_string()),
                    }
                }

                // TODO: support for multiple comma-separated declarations and "=" characters.
                match context.next() {
                    Some(Token::Semicolon) => {
                        return Ok(Declaration{
                            _type: Type{
                                base_name:      base_name,
                                modifiers:      modifiers,
                                length:         None,
                                pointer_levels: pointer_levels,
                            },
                            variable:       variable,
                            initial_value:  None,
                        });
                    },
                    Some(token) => return Err(format!("unexpected token {:?} during parse of pointer declaration", token)),
                    _ => return Err("incomplete pointer declaration".to_string()),
                };
            }

            Some(token) => return Err(format!("unexpected token {:?} during parse of declaration", token)),
            _ => return Err("incomplete declaration".to_string()),
        }
    }
}

/**
 * Look for function declarations of the form
 *
 * ```
 * void do_something(int a) {}
 * void do_something(int a); // forward declaration
 * ```
 *
 * And variable declarations of the form
 *
 * ```
 * int a;
 * ```
 */
// build::program is our only exported identifier from this module.
pub fn program(context: &mut Context) -> Result<Program, String> {
    let mut program = Program{
        globals: vec![],
        functions: vec![],
    };

    loop {
        match context.peek() {
            Some(Token::Identifier(_)) => {
                let checkpoint = context.make_checkpoint();

                // first, try to parse it as a declaration
                let declaration = declaration(context);
                if declaration.is_ok() {
                    program.globals.push(declaration.unwrap());
                    continue;
                }

                // if that fails, move back to the point where we tried to parse a declaration, and
                // then try to parse a function definitions.
                checkpoint.restore(context);

                // TODO
                return Err(declaration.unwrap_err());
            },

            Some(tok)   => return Err(format!("unexpected token {:?}", tok)),
            None        => return Ok(program),
        }
    }
}

// TODO: theoretically, each builder function should have its own unit test.
