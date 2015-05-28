/*!
 * This module contains the functions that build ASTs from Tokens.
 */

use lexer::Token;
use lexer::Operator;
use ast::*;

use super::context::Context;

/**
 * Parse function definitions such as
 *
 *      void say_hello() {}
 *      const int number_of_processes() { return 5; }
 *
 * into the appropriate ast::Function structures.
 */
fn function_definition(context: &mut Context, declaration: Declaration) -> Result<Function, String> {
    return Err("unimplemented".to_string());
}

/**
 * Parse declarations such as
 *
 *      const int b
 *      int *a
 *      int c[10]
 *      unsigned char d, e
 *      int f
 *
 * into ast::Declaration structs. The rule for this parse looks something like
 *
 *      ident+ (asterisk+ ident)? ((comma asterisk* ident)*|(equals <expression>)?)
 *
 * A handcrafted parser may not be the most understandable way to build this construct :)
 *
 */
fn declaration(context: &mut Context) -> Result<Declaration, String> {

    //
    // parse identifiers until we see
    //
    // 1. An asterisk. This means **type** definition is essentially over and a variable name is
    //    next. Note that it is technically possible for multiple asterisks to appear.
    // 2. A token we don't recognize. This means the variable declaration is over. Return what we
    //    have so far.
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

                // TODO: support for multiple comma-separated declarations and array declarations
                // (such as [10]).
                return Ok(Declaration{
                    _type: Type{
                        modifiers:      modifiers,
                        base_name:      base_name,
                        pointer_levels: pointer_levels,
                        length:         None,
                    },
                    variable: variable,
                });
            }

            // This is a very simple case: a series of identifiers followed by a semicolon or comma
            // or something like that. We don't really care what the next token is because we might
            // be inside a function's argument list or part of a global variable declaration.
            // Example:
            //
            //      const unsigned long a;
            //
            _ => {
                context.step_back(); // don't want to absorb the next token if it exists

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
                });
            },
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
                let declaration = try!(declaration(context));

                match context.next() {
                    // Global variable declaration without initialization.
                    //
                    //      int num_rows;
                    //
                    Some(Token::Semicolon) => {
                        program.globals.push(declaration);
                        continue;
                    },

                    // TODO: handle Operator::Assign in this block for global variable
                    // declarations like
                    //
                    //      const int NUM_ROWS = 100;
                    //
                    Some(Token::Operator(Operator::Assign)) => {},

                    // Function definition
                    Some(Token::LParen) => {
                        let function = try!(function_definition(context, declaration));
                        program.functions.push(function);
                    },

                    _ => return Err("expected semicolon after global variable declaration".to_string()),
                }
            },

            Some(tok)   => return Err(format!("unexpected token {:?}", tok)),
            None        => return Ok(program),
        }
    }
}

// TODO: theoretically, each builder function should have its own unit test.

